mod db_types;

use lambda_http::{run, service_fn, Error, IntoResponse, RequestPayloadExt};
use mongodb::bson::doc;
use mongodb::error::{TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT};
use mongodb::options::{FindOneAndReplaceOptions, ReplaceOptions};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::env;

use self::db_types::{AccountEntry, UsernameEntry};

#[derive(Deserialize)]
struct Request {
    account_hash: String,
    display_name: String,
}

#[derive(Serialize)]
struct Response {
    ok: bool,
}

async fn setup_user(
    client: Client,
    request: lambda_http::Request,
) -> Result<impl IntoResponse, Error> {
    let body: Request = request.payload()?.ok_or("Empty request")?;

    // Lookup if account_hash exists in `accounts` collection
    // if no: insert record, done
    // if yes:
    //     check if display_name matches
    //          if yes: do nothing, done
    //          if no: name has changed. need to migrate name in stats table, and update in
    //          usernames so polling is updated.
    //
    let account = AccountEntry {
        account_hash: body.account_hash,
        display_name: body.display_name.clone(),
    };
    let username = UsernameEntry {
        display_name: body.display_name,
    };

    let accounts: mongodb::Collection<AccountEntry> =
        client.database("test").collection("accounts");
    let usernames: mongodb::Collection<UsernameEntry> =
        client.database("test").collection("usernames");

    let mut session = client.start_session(None).await?;
    session.start_transaction(None).await?;

    // TRANSACTION BEGIN
    let account_query = doc! { "account_hash": account.account_hash.clone() };
    let username_query = doc! { "display_name": username.display_name.clone() };

    let seen: Option<AccountEntry> = accounts
        .find_one_and_replace_with_session(
            account_query.clone(),
            account.clone(),
            FindOneAndReplaceOptions::builder().upsert(true).build(),
            &mut session,
        )
        .await?;

    usernames
        .replace_one_with_session(
            username_query.clone(),
            username.clone(),
            Some(ReplaceOptions::builder().upsert(true).build()),
            &mut session,
        )
        .await?;

    if let Some(AccountEntry { display_name, .. }) = seen {
        if display_name != account.display_name {
            tracing::info!("detected username change, migrating...");

            // handle migrating usernames table
            let old_username_query = doc! { "display_name": display_name.clone() };
            usernames
                .delete_one_with_session(old_username_query, None, &mut session)
                .await?;

            // TODO: handle migrating existing stats entries
        }
    }

    // TRANSACTION END

    while let Err(err) = session.commit_transaction().await {
        if err.contains_label(UNKNOWN_TRANSACTION_COMMIT_RESULT)
            || err.contains_label(TRANSIENT_TRANSACTION_ERROR)
        {
            continue;
        }

        return Err("Error with DB transaction.".into());
    }

    let resp = Response { ok: true };

    Ok(serde_json::to_string(&resp).map_err(Box::new)?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mongodb_url = env::var("mongodb_url").unwrap();
    let client = Client::with_uri_str(mongodb_url).await?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(|event: lambda_http::Request| {
        setup_user(client.clone(), event)
    }))
    .await
}
