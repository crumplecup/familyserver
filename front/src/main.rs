#![allow(non_snake_case)]
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
mod components;
use components::{Footer, Header, UserCard};
use dioxus::prelude::*;
use tracing::info;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
    info!("🚀 Client started successfully");
    log::info!("Browser logging initiated in Dioxus.");
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
            main {
    // +       class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
               Header {}
               section {
                   class: "md:container md:mx-auto md:py-8 flex-1",
               }
               UserCard {}
               Footer {}
           }
        })
}
