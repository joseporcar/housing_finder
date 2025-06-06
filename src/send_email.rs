use std::env;
use lettre::address::AddressError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Debug)] 
struct Data {
    sender: String,
    password: String,
    recipient: String,
}
impl Default for Data {
    fn default() -> Self {
        Self { sender: env::var("SENDER_EMAIL").unwrap(), 
        password: env::var("PASSWORD").unwrap(), 
        recipient: env::var("RECIPIENT_EMAIL").unwrap() }
    }
}
impl Data {
    fn sender(&self) -> &str {
        &self.sender
    }

    fn credentials(&self) -> Credentials {
        Credentials::new(self.sender.clone(), self.password.clone())
    }

    fn recipient(&self) -> &str {
        &self.recipient
    }
}

pub fn send_email(content: &str) -> Result<(), AddressError> {
    let data = Data::default();
    let message = Message::builder()
        .from(data.sender().parse()?)
        .to(data.recipient().parse()?)
        .subject("New housing found!")
        .body(content.to_owned()).unwrap();

    let mailer = SmtpTransport::relay("smtp.gmail.com").unwrap().credentials(data.credentials()).build();

    match mailer.send(&message) {
        Ok(_) => println!("email sent"),
        Err(e) => println!("error sending email: {:?}", e)
    }
    Ok(())
}