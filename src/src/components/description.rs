use yew::{function_component, html, Html};

#[function_component]
pub fn Description() -> Html {
    html! {
        <div data-aos="zoom-in" data-aos-delay="" data-aos-duration="1000" >
            <p class="description">
                {"Welcome to my personal website! "}<nobr>{"I am "}</nobr>{" passionate about programming and specialize in backend and embedded domains. With a strong command over languages like Rust, Go, C#, and C, "}<nobr>{"I craft"}</nobr>{" efficient and robust solutions that power the digital world. From designing intricate backend architectures to delving into the world of embedded systems, "}<nobr>{"I thrive"}</nobr>{" on challenges that require innovative coding approaches."}
            </p>
        </div>
    }
}
