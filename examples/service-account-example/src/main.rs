use clap::Parser;
use firebase_client::{auth, firestore};

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

    let client = firestore::FirebaseClient::new(auth);

    // ...
}
