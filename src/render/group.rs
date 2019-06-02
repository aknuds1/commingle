
use maud::html;

pub async fn get_group(_: tide::Context<()>) -> tide::EndpointResult<maud::PreEscaped<String>> {
    Ok(html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        link rel="stylesheet" href="https://unpkg.com/tachyons@4.10.0/css/tachyons.min.css";
        body {
            div#container.pv4.ph5 {
                h1.f1.ma0 {"Rust Berlin Group"}
                p {"Welcome to Rust Berlin!"}
            }
        }
    })
}
