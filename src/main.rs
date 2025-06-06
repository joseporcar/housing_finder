mod send_email;
fn main() {
    dotenvy::dotenv().expect("There is an issue with the .env file");
    send_email::send_email("there is housing in your area");
}
