use yew::{function_component, html, Html};
enum Language {
    Rust,
    Csharp,
    C,
    Go,
    Javascript,
}
#[allow(non_snake_case)]
#[function_component]
pub fn Projects() -> Html {
    let Rust_for_Arduboy = entrys(
        "Rust for Arduboy",
        2023,
        Language::Rust,
        "The Arduboy is a little 8-bit Game console and a AVR micro controller.\nI'm the maintainer of the Rust for Arduboy library. With this library its possible to write some games in Rust for the Arduboy",
        "Rust for Arduboy on GitHub",
        "https://github.com/ZennDev1337/Rust-for-Arduboy",
    );
    let Ferristype = entrys(
        "Ferristype",
        2023,
        Language::Rust,
        "Ferris Type is a minimalist typing game where your sole task is to type whatever appears on your terminal screen. Embrace the simplicity, hone your typing skills, and reach new levels of typing mastery in this immersive terminal-based game.",
        "Ferristype on GitHub",
        "https://github.com/ZennDev1337/Ferristype",
    );
    let Session_auto_mod = entrys(
        "Session Auto Mod WIP",
        2023,
        Language::Csharp,
        "This is a tool for the modding community of Session Sim.\nThis tool automates many steps in the modding process.",
        "Session Auto Mod on GitHub",
        "https://github.com/ZennDev1337/Session-Auto-Mod",
    );
    let Karma_bot = entrys(
        "Karma Telegram Bot",
        2022,
        Language::Go,
        "The Karma-Bot is programmed in GO and runs with an SQlite 3 database",
        "Karma Telegram Bot on GitHub",
        "https://github.com/ZennDev1337/Karma-Bot-Telegram-Bot",
    );
    html! {
        <div id="projects" class="projects" >
            <div id="projects-title" class="projects-title is-italic" data-aos="zoom-out"
        data-aos-duration="1000" data-aos-offset="">
                <p>
                    {"Projects"}
                </p>
            </div>
            {Rust_for_Arduboy}
            {Ferristype}
            {Session_auto_mod}

            {Karma_bot}

        </div>
    }
}

fn entrys(
    name: &str,
    year: u32,
    language: Language,
    text: &str,
    link_text: &str,
    link: &'static str,
) -> Html {
    html!(
        <div class="project" data-aos="fade-left"
        data-aos-duration="1000" data-aos-offset="180">
            <h2 class="project-title">{name} {languagelogo(language)} <span class="project-year">{year}</span> </h2>
            <p class="project-text">{text}</p>
            <a class="project-link" href={link}>{link_text.to_owned()+" >"}</a>
        </div>
    )
}

fn languagelogo(language: Language) -> Html {
    match language {
        Language::Rust => html!(
            <span class="project-year" style="background-color: rgb(170, 86, 12);">{"Rust"}</span>
        ),
        Language::C => html!(
            <span class="project-year" style="background-color: rgb(40, 40, 40);">{"C"}</span>
        ),
        Language::Csharp => html!(
            <span class="project-year" style="background-color: rgb(16, 104, 4);">{"C#"}</span>
        ),
        Language::Go => html!(
            <span class="project-year" style="background-color: rgb(5, 63, 123);">{"Go-Lang"}</span>
        ),
        Language::Javascript => html!(
            <span class="project-year" style="background-color: rgb(104, 93, 1);">{"JavaSkript"}</span>
        ),
    }
}
