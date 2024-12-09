use std::{error::Error, fs::File, io::Write, sync::Arc};
use reqwest::Client;
use serde_json::Value;
use tokio::{task, time::{sleep, Duration}};
use rand::random;

const URL: &str = "https://randomfox.ca/floof/";
const LINK_INDEX: &str = "image";
const DOWNLOAD_AMOUNT: i32 = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Arc::from(Client::new());
    let mut handles: Vec<task::JoinHandle<()>> = vec![];
    for index in 1..=DOWNLOAD_AMOUNT {
        handles.push(task::spawn(get_fox_image(client.clone(), index)));
    }
    for handle in handles {
        match handle.await {
            Ok(_) => {},
            Err(error) => {println!("Error: {error}")},
        };
    }
    

    Ok(())
}

async fn get_fox_image(client: Arc<Client>, num: i32) {
    let sleep_time = random::<f32>()*2 as f32; //please do not do high download values without this *2, I don't wanna strain randomfox.ca's server.
    let empty = Value::from("");

    sleep(Duration::from_secs_f32(sleep_time)).await;

    let body: Value = client.get(URL).send().await.unwrap().json().await.unwrap();
    let image_link = body.get(LINK_INDEX).unwrap_or(&empty).as_str().unwrap();
    let image_data = client.get(image_link).send().await.unwrap().bytes().await.unwrap();
    let mut image_file = File::create(format!("data/{num}.png")).unwrap();
    image_file.write(&image_data.to_vec()).unwrap();
}
