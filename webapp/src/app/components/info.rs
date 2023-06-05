use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use styled::style;

#[component]
pub fn InfoPage(cx: Scope) -> impl IntoView {

    let styles = style!(
        .row {
            margin-left: 20px;
            width: 100%;
            height: 30px;
            display: flex;
            flex-direction: row;
            justify-content: left;
            align-items: center;
            color: white;
        }
        a { 
            color: white;
        }
        h3 {
            color: white;
        }
    );

    styled::view! { cx, styles,
        <h3>"Info: "</h3>
        <div class="row">
            <h5>"Source Code => "</h5> 
            <a href="https://github.com/ali77gh/BlazyShorts">"Github"</a>
        </div>

        <div class="row">
            <h5>"FrontEnd Framework => "</h5> 
            <a href="https://github.com/ali77gh/BlazyShorts">"Leptos"</a>
        </div>

        <div class="row">
            <h5>"BackEnd Framework => "</h5> 
            <a href="https://github.com/ali77gh/BlazyShorts">"Axum"</a>
        </div>

        <div class="row">
            <h5>"DBMS => "</h5> 
            <a href="https://github.com/ali77gh/BlazyShorts">"SurrealDB"</a>
        </div>
    }
}