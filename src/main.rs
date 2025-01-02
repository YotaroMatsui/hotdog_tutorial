use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
struct TitleState(String);

impl TitleState {
    fn get_title(&self) -> String {
        self.0.clone()
    }
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog!".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
            h1 { "{title.get_title()}" }
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
    status: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move{
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let save = move | _event | {};


    rsx! {
        div { id: "dogView",
            img {src: img_src.cloned().unwrap_or_default(), alt: "dog"}
        }
        div { id: "buttons",
            button { onclick: move|_| img_src.restart(), id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}