
mod components;

use crate::components::app::App;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
