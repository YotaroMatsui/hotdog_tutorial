use backend::save_dog;
use dioxus::prelude::*;

mod components;
mod backend;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
}

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route>{}
    }
}


#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogView",
            img { src: img_src.cloned().unwrap_or_default(), alt: "dog" }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button {
                id: "save",
                onclick: move |_| async move {
                    let current_image = img_src.cloned().unwrap();
                    img_src.restart();
                    let _ = save_dog(current_image).await;
                },
                "save!"
            }
        }
    }
}
