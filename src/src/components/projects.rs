use yew::{function_component, html, Html};
#[allow(non_snake_case)]
#[function_component]
pub fn Projects() -> Html {
    let Rust_for_Arduboy = entrys(
        "Rust for Arduboy [Rust]",
        2023,
        "The Arduboy is a little 8-bit Game console and a AVR micro controller.\nI'm the maintainer of the Rust for Arduboy library. With this library its possible to write some games in Rust for the Arduboy",
        "https://github.com/ZennDev1337/Rust-for-Arduboy",
    );
    let Ferristype = entrys(
        "Ferristype [Rust]",
        2023,
        "Ferris Type is a minimalist typing game where your sole task is to type whatever appears on your terminal screen. Embrace the simplicity, hone your typing skills, and reach new levels of typing mastery in this immersive terminal-based game.",
        "https://github.com/ZennDev1337/Ferristype",
    );
    let Session_auto_mod = entrys(
        "Session Auto Mod [C#] WIP",
        2023,
        "This is a tool for the modding community of Session Sim.\nThis tool automates many steps in the modding process.",
        "https://github.com/ZennDev1337/Session-Auto-Mod",
    );
    let Karma_bot = entrys(
        "Karma Telegram Bot [Go]",
        2022,
        "The Karma-Bot is programmed in GO and runs with an SQlite 3 database",
        "https://github.com/ZennDev1337/Karma-Bot-Telegram-Bot",
    );
    html! {
        <div id="projects" class="projects" data-aos="zoom-out"
        data-aos-duration="1000" data-aos-offset="">
            <div id="projects-title" class="projects-title is-italic">
                {"Projects"}
            </div>
            {Rust_for_Arduboy}
            {Ferristype}
            {Session_auto_mod}

            {Karma_bot}

        </div>
    }
}

fn entrys(name: &str, year: u32, text: &str, link: &'static str) -> Html {
    html!(
        <div class="project" data-aos="fade-left"
        data-aos-duration="1000" data-aos-offset="180">
            <h2 class="project-title">{name} <span class="project-year">{year}</span></h2>
            <p class="project-text">{text}</p>
            <a class="project-link" href={link}>{name.to_owned()+" >"}</a>
        </div>
    )
}
