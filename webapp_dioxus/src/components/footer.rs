
#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "about us"
        }
    })
}