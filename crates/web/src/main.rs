use dioxus::prelude::*;
use gloo_net::http::Request;
use types::StatusResponse;

const SERVER_HOST: &str = env!("SERVER_HOST");
const SERVER_PORT: &str = env!("SERVER_PORT");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let status = use_resource(|| async {
        let url = format!("http://{}:{}/api/status", SERVER_HOST, SERVER_PORT);
        Request::get(&url)
            .send()
            .await
            .ok()?
            .json::<StatusResponse>()
            .await
            .ok()
    });

    let text = status
        .read()
        .clone()
        .flatten()
        .map(|s| format!("{} v{}", s.service, s.version));

    rsx! {
        div {
            h1 { "Hello, world!" }
            match &text {
                Some(text) => rsx! {
                    p { "{text}" }
                },
                None => rsx! {
                    p { "Connecting to server..." }
                },
            }
        }
    }
}
