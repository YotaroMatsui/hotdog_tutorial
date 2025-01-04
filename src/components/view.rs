use dioxus::prelude::*;

use crate::backend::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
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