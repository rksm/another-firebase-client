# firebase-client-auth

Use this to get OAuth tokens from Google Cloud service accounts.

Example usage that generates a `bearer_token` string which can be used as an
Authorization header value:

```rust
let account = auth::GoogleServiceAccount::from_env_var("GOOGLE_SERVICE_ACCOUNT")?;
let mut token = auth::GToken::new(account, &[auth::scopes::AUTH_LOGGING_READ]);
let bearer_token = token.refresh_if_necessary().await?;
```
