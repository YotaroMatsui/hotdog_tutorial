use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

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
            h1 { title.get_title() }
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
    status: String,
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string() );

    let fetch_new = move |_| async move{
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        img_src.set(response.message);
    };

    // let skip = move | event | {};
    let save = move | event | {};


    rsx! {
        div { id: "dogView",
            img {src: {img_src}, alt: "dog"}
        }
        div { id: "buttons",
            button { onclick: fetch_new, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}