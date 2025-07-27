use anyhow::Result;
use bytes::Bytes;

pub async fn fetch_image(url: String) -> Result<Bytes> {
    let resp = reqwest::get(url).await?.bytes().await?;
    // println!("{resp:#?}");

    Ok(resp)
}
