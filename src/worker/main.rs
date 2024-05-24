use crate::http_client::fetch_image;
use anyhow::Ok;
use anyhow::Result;
use bytes::Bytes;
use fantoccini::{Client, Locator};
use futures::stream;
use futures::StreamExt;
use my_lib::db;
use my_lib::models::NewCard;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio::time;
use tracing::info;
mod http_client;
mod workers;
use std::sync::Once;

static ONCE: Once = Once::new();

fn cleanup() {}

#[tokio::main]
async fn main() -> Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let base_port = 4000;
    let max_size = 8;

    let drivers = workers::launch_drivers(base_port, max_size)?;

    time::sleep(Duration::from_millis(500)).await;

    let pool = workers::create_pool(base_port, max_size).await;

    let dbconn = my_lib::db::establish_connection();
    let (tx, mut rx) = mpsc::channel(max_size);
    let mgr1 = pool.get().await.expect("get browser");

    mgr1.goto(
        "https://www.tcgplayer.com/search/pokemon/product?productLineName=pokemon&page=1&view=grid",
    )
    .await
    .expect("first page loaded");

    let total_pages = get_total_pages(&mgr1).await?;

    // let cleanup_closure = move || {
    //     println!("closing drivers");

    //     workers::close_drivers(drivers);

    //     cleanup();
    // };

    // ONCE.call_once(cleanup_closure);

    println!("total pages: {}", total_pages);

    drop(mgr1);

    let mut jobs = Vec::<JoinHandle<()>>::new();

    let shared_pool = Arc::new(pool);

    for page in 1..=10 {
        let tx = tx.clone();
        let pool = shared_pool.clone();
        let job = tokio::spawn(async move {
            if let Err(e) = fetch_cards_list_page(page, pool, tx).await {
                eprintln!("Error fetching card details for page {}: {}", page, e);
            }
        });

        jobs.push(job);
    }

    drop(tx); // Stop sending new tasks

    // Receive and process card details concurrently
    let dbconn = Arc::new(Mutex::new(dbconn));
    while let Some(card_info) = rx.recv().await {
        let mut dbconn = dbconn.lock().await;
        if let Err(e) = db::insert_card(&mut dbconn, card_info) {
            eprintln!("Error inserting card: {}", e);
        }
    }

    for h in jobs {
        h.await?;
    }

    // pool.close();
    // drop(pool);

    // workers::close_drivers(drivers);

    Ok(())
}

async fn fetch_cards_list_page(
    page: i32,
    pool: Arc<workers::Pool<Client>>,
    tx: mpsc::Sender<NewCard>,
) -> Result<()> {
    let mgr_page = pool.get().await.expect("get browser");
    let page_url = format!("https://www.tcgplayer.com/search/pokemon/product?productLineName=pokemon&page={}&view=grid", page);

    println!("fetch page {}", page_url);

    mgr_page.goto(page_url.as_str()).await?;

    mgr_page
        .wait()
        .at_most(Duration::from_secs(15))
        .for_element(Locator::Css(".search-result a"))
        .await?;

    let cards = parse_cards(&mgr_page).await?;

    drop(mgr_page);

    for card in cards {
        let mgr_card = pool.get().await.expect("get browser for card details");
        let tx = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = fetch_card_info(mgr_card, card, tx).await {
                eprintln!("Error fetching card info: {}", e);
            }
        });
    }

    Ok(())
}

struct CardInfo {
    title: String,
    image: Option<Bytes>,
    price: Option<f64>,
    url: Option<String>,
    image_url: Option<String>,
}

async fn fetch_card_info(
    mgr_card: deadpool::unmanaged::Object<Client>,
    card: String,
    tx: mpsc::Sender<NewCard>,
) -> Result<()> {
    mgr_card.goto(card.as_str()).await?;

    mgr_card
        .wait()
        .at_most(Duration::from_secs(15))
        .for_element(Locator::Css(".v-lazy-image-loaded"))
        .await?;

    let card_info = get_card_info(&mgr_card).await?;

    let new_card: NewCard = NewCard {
        title: card_info.title,
        image: card_info.image.map(|b| b.to_vec()),
        price: card_info.price,
        url: card_info.url,
        image_url: card_info.image_url,
    };

    drop(mgr_card);
    tx.send(new_card).await?;

    Ok(())
}

async fn get_card_info(client: &Client) -> Result<CardInfo> {
    // https://product-images.tcgplayer.com/fit-in/820x820/497563.jpg

    // image
    // #app > div > div > section.marketplace__content > section > div.product-details-container > div.product-details__product > section.image-set__grid.fit-contain > section > div > div > div > div > div > div > img

    let title_element = client.find(Locator::Css(".product-details__name")).await;
    let title = title_element.ok().unwrap().text().await.ok().unwrap();

    // Price
    let price_el = client.find(Locator::Css(".spotlight__price")).await;
    let price_str = price_el.ok().unwrap().text().await.ok().unwrap();
    let price = price_str.replace("$", "").parse::<f64>().ok();

    let image_element = client
        .find(Locator::Css(".product-details__product section img"))
        .await;

    let image_url = image_element
        .ok()
        .map(|i| async move { i.attr("src").await.ok().unwrap() })
        .unwrap()
        .await;

    info!("image_url: {:?}", image_url);

    let image = image_url
        .clone()
        .map(|url| async { fetch_image(url).await.ok() })
        .unwrap()
        .await;

    let url = client.current_url().await.ok().map(|url| url.to_string());

    Ok(CardInfo {
        image,
        title,
        price,
        url,
        image_url,
    })
}

async fn parse_cards(client: &Client) -> Result<Vec<String>> {
    // #app > div > div > section.marketplace__content > .search-result
    let search_results = client.find_all(Locator::Css(".search-result a")).await?;

    return Ok(stream::iter(search_results)
        .filter_map(|el| async move {
            let url = el.attr("href").await.expect("href");

            url.and_then(|u| Some(format!("https://www.tcgplayer.com{}", u)))
        })
        .collect::<Vec<String>>()
        .await);
}

async fn get_total_pages(client: &Client) -> Result<i32> {
    client
        .wait()
        .at_most(Duration::from_secs(15))
        .for_element(Locator::Css(".search-result"))
        .await?;

    // find total pages
    // #app section.marketplace__content div.search-layout__pagination a
    let pagination_links = client
        .find_all(Locator::Css("div.search-layout__pagination a"))
        .await?;

    let pages: Vec<String> = stream::iter(pagination_links)
        .filter_map(|a| async move { a.text().await.ok() })
        .collect()
        .await;

    let nums = pages
        .iter()
        .filter_map(|p| p.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    Ok(*nums.iter().max().unwrap_or(&10))
}
