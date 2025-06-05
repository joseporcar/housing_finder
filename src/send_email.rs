use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(content: &str) -> Result(_, _) {
    let email = Message::builder().from();
}