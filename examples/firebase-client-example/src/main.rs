use clap::Parser;
use firebase_client::firestore::types::structured_query::field_filter::Operator::*;
use firebase_client::firestore::FromFirestoreDocument;
use firebase_client_auth::{EmailSignin, WebUserAuth};
use serde_json::Value;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    email: String,
    #[clap(short, long)]
    password: String,

    #[clap(short, long, env = "FIREBASE_CLIENT_CONFIG")]
    client_config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .parse_lossy("info,firebase=debug,coscreen=debug"),
        )
        .init();

    let opts = Opts::parse();

    // This is the "web app" config that can be shipped with the client. It's not a secret.
    let config = serde_json::from_str(&opts.client_config).expect("failed to parse client config");

    // Login using firebase auth REST API
    let signin = EmailSignin::new(&opts.email, &opts.password);
    let login = signin.send(&config).await.expect("failed to login");

    let uid = login.uid.clone();

    // that's what the firestore and realtime db clients use to make authenticated requests
    let auth = Box::new(WebUserAuth::new(config, login));

    // the firestore client
    let client = firebase_client::firestore::FirebaseClient::new(auth);

    loop {
        // list all coscreens of that user
        let coscreens = client
            .run_query()
            .from("coscreens")
            .field_filter("members", ArrayContains, &uid)
            .fetch()
            .await
            .expect("failed to run query");

        println!("User {} can access the following CoScreens:", opts.email);

        for coscreen in coscreens {
            // Note: types for CoScreen objects are not yet extracted.
            // Currently they are defined via https://gitlab.com/coscreen/coscreen-backend-rs/blob/master/crates/coscreen-db/src/db/
            // but that is not yet suitable for client-side consumption
            let coscreen = Value::convert_doc(coscreen).unwrap();
            println!("{:#?}", coscreen);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
