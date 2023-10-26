mod db_types;
mod osrs;

use std::{env, time::Duration};

use db_types::{StatEntry, TopPlayerEntry, UsernameEntry};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, DateTime},
    options::FindOneOptions,
    Client,
};
use osrs::HiscoresUser;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongodb_url = env::var("MONGODB_URI")?;
    let client = Client::with_uri_str(mongodb_url).await?;

    let usernames: mongodb::Collection<UsernameEntry> =
        client.database("test").collection("usernames");
    let stats: mongodb::Collection<StatEntry> = client.database("test").collection("stats");
    let top_players: mongodb::Collection<TopPlayerEntry> =
        client.database("test").collection("topPlayers");

    loop {
        let mut cursor = usernames.find(doc! {}, None).await?;
        let mut set: JoinSet<()> = JoinSet::new();

        while let Some(UsernameEntry { display_name }) = cursor.try_next().await? {
            let stats = stats.clone();
            set.spawn(async move {
                println!("Fetching stats for {}", display_name);

                if let Ok(Some(hiscores)) = osrs::user_hiscore(display_name.clone()).await {
                    if let Ok(old) = stats
                        .find_one(
                            doc! { "displayName": display_name.clone() },
                            FindOneOptions::builder()
                                .sort(doc! { "timestamp": -1 })
                                .build(),
                        )
                        .await
                    {
                        let player_stats = StatEntry {
                            timestamp: DateTime::now(),
                            display_name: display_name.clone(),
                            stats: hiscores.clone(),
                        };

                        match old {
                            Some(old) if old.stats == hiscores => {
                                println!("Hiscores match for {}, skipping...", display_name);
                            }
                            _ => {
                                println!("Hiscores different for {}, updating...", display_name);
                                if let Some(err) = stats.insert_one(player_stats, None).await.err()
                                {
                                    println!("{:?}", err);
                                }
                            }
                        }
                    } else {
                        println!("Failed to lookup previous entries.");
                    }
                } else {
                    println!("Failed to load hiscores for {}", display_name);
                }

                ()
            });
        }

        while let Some(_res) = set.join_next().await {
            // Wait for all to finish.
        }

        let top_players = top_players.clone();
        println!("Updating top players");
        top_players.drop(None).await?;
        for i in 1..5 {
            if let Ok(Some(page)) = osrs::hiscores_index(i).await {
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
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        println!("Waiting....");
        tokio::time::sleep(Duration::from_secs(60 * 15)).await;
    }
}
