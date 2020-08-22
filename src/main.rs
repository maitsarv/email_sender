#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod connection;
mod config;
mod schema;
mod email_queue;

fn main() {
    dotenv::dotenv().ok();
    let connection = connection::create_connection();
    email_queue::check_queue(&connection);
}
