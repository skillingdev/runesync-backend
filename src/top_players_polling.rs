mod db_types;
mod osrs;

use std::{env, time::Duration};

use db_types::TopPlayerEntry;
use mongodb::Client;
use osrs::HiscoresUser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongodb_url = env::var("MONGODB_URI")?;
    let client = Client::with_uri_str(mongodb_url).await?;

    let top_players: mongodb::Collection<TopPlayerEntry> =
        client.database("test").collection("topPlayers");

    loop {
        let top_players = top_players.clone();
        println!("Updating top players...");
        top_players.drop(None).await?;
        for i in 1..5 {
            if let Some(page) = osrs::hiscores_index(i)
                .await
                .map_err(|_| "hiscores_index failed")?
            {
                top_players
                    .insert_many(
                        page.users
                            .into_iter()
                            .map(|HiscoresUser { name, score }| TopPlayerEntry {
                                display_name: name,
                                league_points: score,
                            }),
                        None,
                    )
                    .await?;
            } else {
                println!("Index call failed..")
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        println!("Waiting....");
        tokio::time::sleep(Duration::from_secs(60 * 15)).await;
    }
}
