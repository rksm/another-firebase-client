# rust firebase client

This implements a firebase client for Rust.

It works uses the firebase and gcloud GRPC and REST endpoints.

## Documentation

To see API documentation, run `cargo doc --open -p firebase-client -p firebase-client-auth`.

It implements an interface for reading, writing, deleting, and streaming firestore and realtime db data. See the `firebase_client::rdb` and `firebase_client::firestore` modules for more details.

## Examples

An example for both the authentication and client usage can be found at [examples/firebase-client-example/src/main.rs](examples/firebase-client-example/src/main.rs).

Run it with:

```bash
cargo install just
just firebase-client-example -- --email "<EMAIL>" --password "<PASSWORD>"
```
