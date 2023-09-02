use dioxus::prelude::*;

use crate::components::{
    header::Header,
    footer::Footer,
};


pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            background: "var(--background)",
            position: "absolute",
            width: "100%",
            height: "100%",
            Header{}
            div{
                margin_top: "-50px",
                background: "var(--primary_dark)",
                "yo"
            },
            Footer{}
        }
    })
}