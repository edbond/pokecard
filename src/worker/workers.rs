use anyhow::Result;
use deadpool::{unmanaged::Pool, unmanaged::PoolConfig};
use fantoccini::{wd::Capabilities, Client, ClientBuilder};
use std::process::{Child, Command};
use tracing::{info, warn};

pub fn launch_drivers(base_port: i16, max_size: usize) -> Result<Vec<Child>> {
    let childs = (0..max_size)
        .map(|i| {
            // launch shell process
            // ./chromedriver --port="$port"

            let port = base_port as usize + i;

            info!("launching driver on port {}", port);

            let mut binding = Command::new("./chromedriver");
            let cmd = binding.arg(format!("--port={}", port));

            info!("running command {:#?}", cmd);

            cmd.spawn().expect("failed to execute process")
        })
        .collect();

    Ok(childs)
}

pub fn close_drivers(mut drivers: Vec<Child>) -> () {
    drivers.iter_mut().for_each(|c| match c.kill() {
        Err(error) => warn!("failed to kill process: {}", error),
        _ => (),
    })
}

pub async fn create_pool(base_port: i16, max_size: usize) -> Pool<Client> {
    let pool = Pool::from_config(&PoolConfig::new(max_size));

    for i in 0..max_size {
        let browser = format!("http://localhost:{}", base_port as usize + i);
        let browser_url = browser.as_str();

        let cap: Capabilities = serde_json::from_str(
            r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
        )
        .unwrap();

        let client = ClientBuilder::native()
            .capabilities(cap)
            .connect(browser_url)
            .await
            .expect("failed to connect to WebDriver");

        pool.add(client).await.expect("client added");
    }

    pool
}
