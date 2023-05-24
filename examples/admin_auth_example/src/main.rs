use clap::Parser;
use firebase_client::{
    admin_auth::{self, User},
    auth, firestore,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(
        short,
        long,
        env = "GOOGLE_SERVICE_ACCOUNT",
        help = "The service account key JSON",
        value_parser = auth::GoogleServiceAccount::from_json_str
    )]
    google_service_account: auth::GoogleServiceAccount,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .parse_lossy("info,firebase=debug,coscreen=debug"),
        )
        .init();

    let _opts = Opts::parse();
    let auth = auth::auth_from_env_or_cli().expect("Failed to get auth");

    // let client = firestore::FirebaseClient::new(auth);

    let stream = admin_auth::AccountBatchGet::new(auth.box_clone())
        .max_results(500)
        .fetch()
        .await;

    let mut stream = std::pin::pin!(stream);

    let mut users = Vec::new();

    while let Some(next) = stream.next().await {
        let user = match next {
            Err(e) => {
                tracing::error!("Error getting user: {:?}", e);
                continue;
            }
            Ok(next) => next,
        };

        if let User::FullUser(user) = user {
            users.push(user);
        }
    }

    println!("{}", serde_json::to_string_pretty(&users).unwrap());

    for u in users {
        println!("{u:#?}");
    }

    // let client = reqwest::Client::new();

    // let req = client
    //     .get(format!(
    //     "https://identitytoolkit.googleapis.com/v1/projects/{target_project_id}/accounts:batchGet",
    // ))
    //     .query(&[
    //         ("maxResults", max_results.to_string()),
    //         ("access_token", token),
    //     ])
    //     .build()
    //     .expect("Failed to build request");

    // let res = client
    //     .execute(req)
    //     .await
    //     .expect("Failed to execute request");

    // // dbg!(res.status());
    // // dbg!(res.text().await.expect("Failed to get text"));

    // let content = res
    //     .json::<serde_json::Value>()
    //     .await
    //     .expect("Failed to get json");

    // println!("{}", serde_json::to_string_pretty(&content).unwrap());

    // https://www.googleapis.com/auth/identitytoolkit
    // https://www.googleapis.com/auth/firebase
    // https://www.googleapis.com/auth/cloud-platform

    // ...
}
