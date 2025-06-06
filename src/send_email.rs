use std::env;
use lettre::address::AddressError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Debug)] 
pub struct Mailer {
    sender: String,
    recipient: String,
    relay: SmtpTransport,
}

impl Mailer {
    pub fn new() -> Self {
        Self { sender: env::var("SENDER_EMAIL").unwrap(), 
            recipient: env::var("RECIPIENT_EMAIL").unwrap() ,
            relay: SmtpTransport::relay("smtp.gmail.com").unwrap().credentials(Credentials::new(env::var("SENDER_EMAIL").unwrap(), env::var("PASSWORD").unwrap())).build()
        }
    }

    fn message(&self, content: &str) -> Result<Message, AddressError> {
        Ok(Message::builder()
                    .from(self.sender.parse()?)
                    .to(self.recipient.parse()?)
                    .subject("New housing found!")
                    .body(content.to_owned()).unwrap())
    }

    pub fn send_email(&self, content: &str) {
        match self.relay.send(&self.message(content).expect("invalid email adress")) {
            Ok(_) => println!("email sent"),
            Err(e) => println!("error sending email: {:?}", e)
        }
    }
}