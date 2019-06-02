#![feature(async_await)]
#![feature(proc_macro_hygiene)]

extern crate maud;
#[macro_use]
extern crate diesel;

mod render;
mod error;

use diesel::prelude::*;
use diesel::{PgConnection, ConnectionError};
use std::env;
use error::Error;

fn establish_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

fn main() -> Result<(), Error> {
    let conn = establish_connection()?;

    let mut app = tide::App::new(());
    app.at("/").get(render::group::get_group);
    app.at("/healthz").get(async move |_| ());
    Ok(app.serve("0.0.0.0:8000")?)
}
