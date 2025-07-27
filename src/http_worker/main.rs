use anyhow::Result;
use diesel::SqliteConnection;
use indicatif::ProgressBar;
use my_lib::{db, models::NewCard};
use reqwest::{self, Client, Url};
use response::Result2;
use std::sync::mpsc::sync_channel;
use tokio::runtime::Runtime;

mod response;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let page_payload = r#"{"algorithm":"sales_synonym_v2","from":0,"size":24,"filters":{"term":{"productLineName":["pokemon"],"productTypeName":["Cards"]},"range":{},"match":{}},"listingSearch":{"context":{"cart":{}},"filters":{"term":{"sellerStatus":"Live","channelId":0},"range":{"quantity":{"gte":1}},"exclude":{"channelExclusion":0}}},"context":{"cart":{},"shippingCountry":"US","userProfile":{}},"settings":{"useFuzzySearch":true,"didYouMean":{}},"sort":{}}"#;

    let url = Url::parse(
        "https://mp-search-api.tcgplayer.com/v1/search/request?q=&isList=false&mpfev=2435",
    )?;

    env_logger::init();

    let mut dbconn = my_lib::db::establish_connection();

    // Create an MPSC channel
    let (tx, rx) = sync_channel(64);

    // Spawn a separate thread to execute the SQL queries
    std::thread::spawn(move || {
        while let Ok(rr) = rx.recv() {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                store_card(&mut dbconn, rr)
                    .await
                    .unwrap_or_else(|err| eprintln!("Error storing card: {:?}", err));
            });
        }
    });

    let pages = 417;
    let pb = ProgressBar::new(pages);

    for i in 0..pages {
        let payload = page_payload.replace(r#"from":0"#, &format!(r#"from":{}"#, i * 24));

        // println!("{}", payload);

        let response = client
            .post(url.clone())
            .body(payload)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let bytes = response.bytes().await?;

        // let text = std::str::from_utf8(&bytes)?;
        // println!("{}", text);

        match serde_json::from_slice::<response::Root>(&bytes) {
            Ok(root) => {
                for r in root.results {
                    for rr in r.results {
                        tx.clone().send(rr)?;
                    }
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
        pb.inc(1);
    }

    pb.finish();

    Ok(())
}

async fn store_card(mut dbconn: &mut SqliteConnection, card: Result2) -> Result<()> {
    let var_name = &format!(
        "https://product-images.tcgplayer.com/fit-in/736x736/{}.jpg",
        card.product_id as u64
    );
    let image_url = var_name.as_str();

    let resp = reqwest::get(image_url).await?.bytes().await?;
    // println!("{resp:#?}");

    let url = format!(
        "https://www.tcgplayer.com/product/{}",
        card.product_id as u64
    );

    // Receive and process card details concurrently
    let new_card: NewCard = NewCard {
        title: card.product_name,
        image: Some(resp.to_vec()),
        price: Some(card.market_price),
        // https://www.tcgplayer.com/product/478139/pokemon-crown-zenith-bidoof?page=1&Language=English
        url: Some(url),
        image_url: Some(image_url.to_string()),
    };

    db::insert_card(&mut dbconn, new_card)?;

    Ok(())
}
