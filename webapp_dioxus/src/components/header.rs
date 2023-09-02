
#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            background: "var(--primary)",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            font_size: "3rem",
            color: "var(--accent)",
            padding_top: "1rem",
            padding_bottom: "6rem",
            div{
                display: "flex",
                flex_direction: "column",
                align_items: "center",
                img { src: "./img/shorts.png", width: "100px", height: "100px"}
                "Blazy-Short"
            }
        }
    })
}