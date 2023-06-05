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
        .title {
          background-color: red;
          color: white;
          font-size: 30px;
        }
    );


    styled::view! { cx, styles,
        <div class="title">"Blazy Shorts"</div>
        {layout}
    }
}



#[component]
pub fn Tabs(cx: Scope) -> impl IntoView {

}