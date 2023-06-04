use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                    <Route path="" view=|cx| view! { cx, <HomePage initial_value=3/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, initial_value: i64) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, initial_value);
    
    let on_click_pos = move |_| set_count.update(|count| *count += 1);
    let on_click_neg = move |_| set_count.update(|count| *count -= 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click_pos>"+"</button>
        <button on:click=on_click_neg>"-"</button>
        <div>{count}</div>
    }
}
