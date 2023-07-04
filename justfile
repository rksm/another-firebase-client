set dotenv-load

export RUST_BACKTRACE := "1"

default:
    just --list

example *args='':
    cargo run --example {{args}}

firebase-client-example *args='':
    FIREBASE_CLIENT_CONFIG=$(aws --profile exec-coscreen-account-admin secretsmanager get-secret-value --output=json --region=us-east-2 --secret-id prod/firebase/native-client-config | jq -rc '.SecretString|fromjson') \
      cargo run --bin firebase-client-example -- {{args}}

anon-example *args='':
    FIREBASE_CLIENT_CONFIG=$(aws --profile exec-coscreen-account-admin secretsmanager get-secret-value --output=json --region=us-east-2 --secret-id prod/firebase/native-client-config | jq -rc '.SecretString|fromjson') \
      cargo run --bin anon-example -- {{args}}

join-session-example *args='':
    FIREBASE_CLIENT_CONFIG=$(aws --profile exec-coscreen-account-admin secretsmanager get-secret-value --output=json --region=us-east-2 --secret-id prod/firebase/native-client-config | jq -rc '.SecretString|fromjson') \
      cargo run --bin join-session-example -- {{args}}

admin-auth-example *args='':
    cd examples/admin_auth_example && ./run.sh {{args}}

test:
    cargo test -- --nocapture

install:
    cargo install --path crates/cli
