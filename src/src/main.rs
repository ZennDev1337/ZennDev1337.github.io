use yew::prelude::*;
mod components;
use components::{Description, Header, Infos, Links, Navbar, Projects};

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
                <div style="height:100px;">
                    {""}
                </div>
            </div>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
