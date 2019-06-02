#![feature(async_await)]
#![feature(proc_macro_hygiene)]

extern crate maud;

mod render;

fn main() -> Result<(), std::io::Error> {
    let mut app = tide::App::new(());
    app.at("/").get(render::group::get_group);
    app.at("/healthz").get(async move |_| ());
    Ok(app.serve("0.0.0.0:8000")?)
}
