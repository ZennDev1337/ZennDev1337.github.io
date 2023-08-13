use yew::{function_component, html, Html};

#[function_component]
fn Location() -> Html {
    html! {
        <div class="location" >
        <div class="location-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" class="bi bi-geo-alt" viewBox="0 0 16 16">
      <path d="M12.166 8.94c-.524 1.062-1.234 2.12-1.96 3.07A31.493 31.493 0 0 1 8 14.58a31.481 31.481 0 0 1-2.206-2.57c-.726-.95-1.436-2.008-1.96-3.07C3.304 7.867 3 6.862 3 6a5 5 0 0 1 10 0c0 .862-.305 1.867-.834 2.94zM8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10z"/>
      <path d="M8 8a2 2 0 1 1 0-4 2 2 0 0 1 0 4zm0 1a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
    </svg>
        </div>
         <p>{" Somewhere in Switzerland"}</p>
        </div>    }
}

#[function_component]
fn Status() -> Html {
    let dot = "dot";

    html! {
        <div class="status" >
            <span class={dot}></span>
            <div class="status-text">
                <p>{" Available for questions"}</p>
            </div>
        </div>
    }
}

#[function_component]
pub fn Infos() -> Html {
    html! {
        <div class="infos" data-aos="zoom-in" data-aos-delay="" data-aos-duration="1000">
        <div class="columns">
          <div class="column">
            <Location/>
          </div>
          <div class="column">
            <Status/>
          </div>
          </div>
          </div>
    }
}
