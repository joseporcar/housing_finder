mod send_email;
use crate::send_email::Mailer;
fn main() {
    dotenvy::dotenv().expect("There is an issue with the .env file");
    let mailer = Mailer::new();

    mailer.send_email("there is housing in your area");
}
