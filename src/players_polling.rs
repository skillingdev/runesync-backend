mod db_types;
mod osrs;

use std::env;
use std::time::Duration;

use db_types::UsernameEntry;
use mongodb::Client;
use mongodb::bson::doc;
use mongodb::options::{InsertManyOptions, UpdateOptions};
use osrs::HiscoresUser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongodb_url = env::var("MONGODB_URI")?;
    let client = Client::with_uri_str(mongodb_url).await?;

    let usernames: mongodb::Collection<UsernameEntry> =
        client.database("test").collection("usernames");

    let mut i = 0;

    loop {
        let usernames = usernames.clone();
        println!("Updating usernames with hiscores page {}...", i);

        if let Some(Some(page)) = osrs::hiscores_index(i)
            .await
            .ok()
        {
            let users = page.users
                .into_iter()
                .map(|HiscoresUser { name, score: _ }| UsernameEntry {
                    display_name: name,
                });

            for user in users {
                usernames.update_one(doc! { "displayName": user.display_name.clone() }, doc! { "displayName": user.display_name }, UpdateOptions::builder().upsert(true).build()).await.ok();
            }

            i = i + 1;
        } else {
            i = 0;
        }

        println!("Waiting....");
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
