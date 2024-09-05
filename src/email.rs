use lettre::message::{header::ContentType, Mailbox, Message};
use serde::Serialize;

pub struct Settings {
    pub username: String,
    pub password: String,
    pub smtp_relay: String,
    pub from: Mailbox,
    pub reply_to: Mailbox,
}

#[derive(Serialize)]
pub struct Output {
    pub email: String,
    pub sent: bool,
}

pub fn create_email(
    from: &Mailbox,
    reply_to: &Mailbox,
    to: &Mailbox,
    subject: &str,
    body: &str,
) -> Message {
    Message::builder()
        .from(from.clone())
        .reply_to(reply_to.clone())
        .to(to.clone())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .unwrap()
}
