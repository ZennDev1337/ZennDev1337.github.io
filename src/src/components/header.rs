use yew::{function_component, html, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <section class="section">
          <h1 id="hacker-title" class="title is-italic ">
          {"ZennDev1337 aka Zenny 🦀"}
          </h1>
          <h2 id="hacker-subtitle" class="subtitle ">
            {"A Backend Developer"}
          </h2>
      </section>


    }
}
