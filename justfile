set dotenv-load

default:
    just --list

example *args='':
    cargo run --example {{args}}

firebase-client-example *args='':
    FIREBASE_CLIENT_CONFIG=$(aws --profile exec-coscreen-account-admin secretsmanager get-secret-value --output=json --region=us-east-2 --secret-id prod/firebase/native-client-config | jq -rc '.SecretString|fromjson') \
      cargo run --bin firebase-client-example {{args}}

