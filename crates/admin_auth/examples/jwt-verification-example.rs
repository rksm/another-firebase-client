use firebase_client_admin_auth::{TokenVerification, TokenVerificationError};

#[tokio::main]
async fn main() -> Result<(), TokenVerificationError> {
    // let required_subject = Some("74WxppgUPAZWAzN5e3acqwhOiIA3".to_string());
    let required_subject = None;
    let project_id = "podwriter-io";
    let token ="eyJhbGciOiJSUzI1NiIsImtpZCI6ImIyZGZmNzhhMGJkZDVhMDIyMTIwNjM0OTlkNzdlZjRkZWVkMWY2NWIiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vcG9kd3JpdGVyLWlvIiwiYXVkIjoicG9kd3JpdGVyLWlvIiwiYXV0aF90aW1lIjoxNjkwMjI2NTU3LCJ1c2VyX2lkIjoiNzRXeHBwZ1VQUFpXQXpONWUzYWNxd2hPaUlBMyIsInN1YiI6Ijc0V3hwcGdVUFBaV0F6TjVlM2FjcXdoT2lJQTMiLCJpYXQiOjE2OTAyMjY1NTcsImV4cCI6MTY5MDIzMDE1NywiZW1haWwiOiJmb29AYmFyLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwiZmlyZWJhc2UiOnsiaWRlbnRpdGllcyI6eyJlbWFpbCI6WyJmb29AYmFyLmNvbSJdfSwic2lnbl9pbl9wcm92aWRlciI6InBhc3N3b3JkIn19.kdKKrIzH-U9OyjO27gNX8s3CenuECA19ReVbOMep0LW4yTc8s6kKHoZKqGU8NLjYgLDgkXtF3vOIoMglAFnNS3Xezdr14fFe69fVxbEglFxCG--aIUNWdGZEZzizcNSfOYCiKZerf9q59RK01GRuJHBf3EtAdYYekLGWIFYnClytF6JfaT4y1hi-EaO0KwroR8FeM0PJQfv3XfRaJaYWD3rddede4_tS8WD7yK-M7RFSDPIK_V6NHoxiO9zhuJsDNOKcezBwIUeXSUgqfKjeQQxLi68Wn51H-__d-ZD0U67CH9MSqUqbr3gGVgfb1ErWtqtCryPL21isXaiSMBabdA";

    let claims = TokenVerification::builder()
        .project_id(project_id)
        .token(token)
        .required_subject(required_subject)
        .build()
        .verify()
        .await?;

    dbg!(claims);

    Ok(())
}
