use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

static mut CONNECTION: Option<PgConnection> = None;

pub unsafe fn init() {
    // use haje::schema::accounts::dsl::*;
    CONNECTION = Some(establish_connection());
}

pub fn get_connection() -> &'static Option<PgConnection> {
    unsafe { &CONNECTION }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    log::info!("Connection established");

    return connection;
}
