use eyre::Result;
use clap::Parser;
use firebase_client::auth::{Authorization, EmailSignin, WebUserAuth};
use firebase_client::firestore::FromFirestoreDocument;
use firebase_client::{
    firestore::types::structured_query::field_filter::Operator::*, rdb::RdbClient,
};
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
    let fb_client = firebase_client::firestore::FirebaseClient::new(auth.box_clone());
    let rdb_client = firebase_client::rdb::RdbClient::new(auth);

    // list all coscreens of that user
    let coscreens = fb_client
        .run_query()
        .from("coscreens")
        .field_filter("members", ArrayContains, &uid)
        .fetch()
        .await
        .expect("failed to run query");

    let Some(coscreen) = coscreens.into_iter().find(|coscreen| {
        let coscreen = Value::convert_doc(coscreen.clone()).unwrap();
        coscreen["fields"]["name"] == "foo bar"
    }) else {
        panic!("no coscreen found");
    };

    println!("{:#?}", coscreen);
}

struct CoScreenSessionConnection {
    id: String,
    rdb_client: RdbClient,
}

impl CoScreenSessionConnection {
    pub fn new(id: String, rdb_client: RdbClient) -> Self {
        Self { id, rdb_client }
    }

    pub async fn join_waiting(&self) -> Result<()> {
        let mut rdb = self.rdb_client.get_connection().await?;
        let mut tx = rdb.transaction().await?;

        let mut waiting = tx
            .get::<_, Value>(&format!("coscreens/{}/waiting", self.id))
            .await?;
        let mut connected = tx
            .get::<_, Value>(&format!("coscreens/{}/connected", self.id))
            .await?;

        let mut waiting = waiting.as_array_mut().unwrap();
        let mut connected = connected.as_array_mut().unwrap();

        let mut session = waiting.pop().unwrap();
        let session_id = session["id"].as_str().unwrap().to_string();

        connected.push(session);

        tx.set(&format!("coscreens/{}/waiting", self.id), waiting)
            .await?;
        tx.set(&format!("coscreens/{}/connected", self.id), connected)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
