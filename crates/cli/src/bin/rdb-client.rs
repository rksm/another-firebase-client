use eyre::Result;
use clap::{Parser, ValueEnum};
use futures::StreamExt;

#[derive(Parser)]
#[command(
    version,
    author,
    about("Read and modify values of firebase realtime db.")
)]
struct Options {
    #[arg(short, long, value_enum, default_value_t = Env::Dev)]
    env: Env,

    #[arg(long, help("Shallow get?"))]
    shallow: bool,

    #[arg(value_enum)]
    method: Method,

    #[arg(help("The path to get."))]
    path: String,

    #[arg(help("The JSON value to send."))]
    value: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, Eq, PartialEq, PartialOrd, Ord)]
enum Env {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Copy, ValueEnum, Eq, PartialEq, PartialOrd, Ord)]
enum Method {
    Get,
    Put,
    Post,
    Patch,
    Delete,
    Watch,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Options::parse();

    let env_file = match args.env {
        Env::Prod => ".env",
        Env::Dev => ".env-dev",
    };

    dotenv::from_filename(env_file).expect(".env");

    let path_with_ext = if !args.path.ends_with(".json") {
        format!("{}.json", args.path)
    } else {
        args.path.to_string()
    };

    let value = args
        .value
        .map(|val| serde_json::from_str(&val).unwrap())
        .unwrap_or(serde_json::Value::Null);

    let acct = firebase_client_auth::GoogleServiceAccount::from_default_env_var()?;
    let mut rdb_client = firebase_client::rdb::RdbClient::for_account(acct)?;
    // rdb_client
    //     .token
    //     .cached(cache_dir.join("rdb-token.json"))
    //     .await?;

    match args.method {
        Method::Get => {
            let shallow = args.shallow;
            let result: serde_json::Value =
                rdb_client.shallow(shallow).get_path(&path_with_ext).await?;
            let result = serde_json::to_string_pretty(&result)?;
            println!("{}", result);
        }
        Method::Put => {
            rdb_client.put_path(&path_with_ext, &value).await?;
        }
        Method::Post => {
            let result = rdb_client.post_path(&path_with_ext, &value).await?;
            println!("path={}/{}", args.path, result.name);
        }
        Method::Patch => {
            rdb_client.patch_path(&path_with_ext, &value).await?;
        }
        Method::Delete => {
            rdb_client.delete_path(&path_with_ext).await?;
        }
        Method::Watch => {
            let stream =
                firebase_client::rdb::listener::Listener::new(rdb_client).stream(args.path);
            futures::pin_mut!(stream);

            while let Some((path, val)) = stream.next().await {
                tracing::trace!("changed {:?}", path);

                let locked = val.lock().await;
                match serde_json::to_string_pretty(locked.as_ref()) {
                    Ok(serialized) => println!("{}", serialized),
                    Err(err) => eprintln!("Error printing changed value {}", err),
                }
            }
        }
    }

    Ok(())
}
