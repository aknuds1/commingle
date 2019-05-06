#![feature(async_await)]

fn main() -> Result<(), std::io::Error> {
    let mut app = tide::App::new(());
    app.at("/").get(async move |_| "Hello, world!");
    app.at("/healthz").get(async move |_| ());
    Ok(app.serve("127.0.0.1:8000")?)
}
