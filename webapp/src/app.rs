use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;

use crate::app::components::home::HomePage;
use crate::app::components::home::HomePageProps;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage path="/links"/> }/> // default
                    <Route path="/info" view=|cx| view! { cx, <HomePage path="/info"/> }/>
                    <Route path="/links" view=|cx| view! { cx, <HomePage path="/links"/> }/>
                </Routes>
            </main>
        </Router>
    }
}

