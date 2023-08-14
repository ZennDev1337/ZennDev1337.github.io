use yew::{function_component, html, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <section class="section">
          <h1 id="hacker-title" class="title">
          {"ZennDev1337 "} <nobr> {"aka Zenny 🦀"} </nobr>
          </h1>
          <h2 id="hacker-subtitle" class="subtitle ">
            {"A Backend Developer"}
          </h2>
      </section>


    }
}
