use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use styled::style;

use crate::app::components::info::InfoPage;
use crate::app::components::info::InfoPageProps;
use crate::app::components::links::LinksPage;
use crate::app::components::links::LinksPageProps;

#[component]
pub fn HomePage(cx: Scope, path: &'static str) -> impl IntoView {

    let layout = match path {
        "/info" => { view! { cx, <div> <InfoPage/> </div> } },
        "/links" => { view! { cx, <div> <LinksPage/> </div> } },
        &_ => todo!(),
    };
    
    let styles = style!(
        .background {
            background-color: #5B6C5D;
            border-radius: 5px;
        }
        .title {
          font-size: 50px;
          color: white;
          margin: 20px;
        }
        .tabs {
            background-color: #59C9A5;
            width: 100%;
            height: 30px;
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;
        }
        .tab_item {
            padding: 10px;
            color: white;
            text-decoration: none;
            background-color: #465775;
        }
        .divader {
            background-color: #FFFFFF;
            width: 1px;
        }
    );


    styled::view! { cx, styles,
        <div class="background">
            <div class="title">"Blazy Shorts"</div>
            <div class="tabs">
                <a class="tab_item" href="/links">"links"</a>
                <div class="divader"></div>
                <a class="tab_item" href="/info">"info"</a>
            </div>
            <div> {layout} </div>
        </div>
    }
}
