use clap::Parser;
use firebase_client_auth::WebUserAnonAuth;

#[derive(Debug, Parser)]
struct Opts {
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
    let auth = Box::new(WebUserAnonAuth::new(config));

    // the firestore client
    let client = firebase_client::firestore::FirebaseClient::new(auth);

    // list all coscreens of that user
    let login = client
        .get_document("login-tokens/0006bfd7-ae3d-4014-8d30-d2ccd68dc8b8")
        .fetch()
        .await
        .expect("failed to fetch document");

    dbg!(login);
}
