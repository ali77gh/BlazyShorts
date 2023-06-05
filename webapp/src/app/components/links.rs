use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Div;
use crate::app::leptos_dom::console_log;

use common::api::{add_link_api::{AddLinkApi, RequestBody}, BaseApi};


#[component]
pub fn LinksPage(cx: Scope) -> impl IntoView {
    
    let (link, set_link) = create_signal(cx, "".to_string());

    let err: Memo<String> = create_memo(cx, move |_| {
        match AddLinkApi::validate(&RequestBody{link:link.get()}) {
            Err(err) => err,
            Ok(_) => "OK".to_string()
        }
    });


    view! { cx,
        <div>
            <h3>"Add link:"</h3>
            <input type="text" on:input=move |ev| { set_link(event_target_value(&ev)); } ></input>
            <div>{err}</div>
        </div>
    }
}