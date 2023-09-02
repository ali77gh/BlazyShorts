
#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn LinkList(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "List of links here"
        }
    })
}