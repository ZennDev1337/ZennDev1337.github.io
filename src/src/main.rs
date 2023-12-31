use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod components;
use components::{Description, Header, Infos, Links, Navbar, Pgp, Projects};

#[wasm_bindgen(module = "/assets/main.js")]
extern "C" {
    fn mainjs();
    fn hacker_text(id: String, interactions: u32, speed: u32);
}

#[function_component]
fn App() -> Html {
    html! {
        <div id="particles-js" class="background">
            <div class={classes!("container")}>
                <Header/>
                <Description/>
                <Infos/>
                <Navbar/>
                <Projects/>
                <Links/>
                <Pgp/>
                <div style="height:100px;">
                    {""}
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    let timeout = Timeout::new(1, move || {
        // Do something after the one second timeout is up!
        hacker_text("hacker-title".to_owned(), 7, 10);
        hacker_text("hacker-subtitle".to_owned(), 6, 15);
        hacker_text("projects-title".to_owned(), 10, 20);
        hacker_text("links-title".to_owned(), 10, 30);
        hacker_text("pgp-keys-title".to_owned(), 10, 30);
        mainjs();
    });
    timeout.forget();
}
