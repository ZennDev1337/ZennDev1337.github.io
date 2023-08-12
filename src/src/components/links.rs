use yew::{function_component, html, Html};

fn entry(text: &str, link: &'static str, icon: Html) -> Html {
    html! {
        <div class="column is-half is-align-content-center" data-aos="fade-up"
        data-aos-duration="1000" data-aos-offset="">
            <div class="links-item">
                <div class="link-icon">
                    {icon}
                </div>
                <a class="link-text" href={link}>{text}</a>
            </div>
        </div>
    }
}

#[function_component]
pub fn Links() -> Html {
    let github = entry(
        "ZennDev1337",
        "https://github.com/ZennDev1337",
        html!(<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
  </svg>),
    );
    let discord = entry(
        "zenndev",
        "https://discord.com/users/461251276085395466",
        html!(<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-discord" viewBox="0 0 16 16">
  <path d="M13.545 2.907a13.227 13.227 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.19 12.19 0 0 0-3.658 0 8.258 8.258 0 0 0-.412-.833.051.051 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.041.041 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032c.001.014.01.028.021.037a13.276 13.276 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019c.308-.42.582-.863.818-1.329a.05.05 0 0 0-.01-.059.051.051 0 0 0-.018-.011 8.875 8.875 0 0 1-1.248-.595.05.05 0 0 1-.02-.066.051.051 0 0 1 .015-.019c.084-.063.168-.129.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.052.052 0 0 1 .053.007c.08.066.164.132.248.195a.051.051 0 0 1-.004.085 8.254 8.254 0 0 1-1.249.594.05.05 0 0 0-.03.03.052.052 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.235 13.235 0 0 0 4.001-2.02.049.049 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.034.034 0 0 0-.02-.019Zm-8.198 7.307c-.789 0-1.438-.724-1.438-1.612 0-.889.637-1.613 1.438-1.613.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612Zm5.316 0c-.788 0-1.438-.724-1.438-1.612 0-.889.637-1.613 1.438-1.613.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612Z"/>
</svg>),
    );
    let arduboy = entry(
        "Arduboy Community",
        "https://community.arduboy.com/u/zenndev1337",
        html!(<svg version="1.0" xmlns="http://www.w3.org/2000/svg"
    width="20px" height="20px" viewBox="0 0 24.000000 24.000000"
    preserveAspectRatio="xMidYMid meet">
   
   <g transform="translate(0.000000,24.000000) scale(0.100000,-0.100000)"
   fill="#c2c2c2" stroke="none">
   <path d="M43 219 c-10 -10 -8 -164 2 -197 4 -9 25 -12 77 -10 l73 3 8 94 c5
   66 4 97 -4 107 -14 17 -140 19 -156 3z m127 -49 l0 -40 -50 0 -50 0 0 40 0 40
   50 0 50 0 0 -40z m-40 -60 c0 -5 -4 -10 -10 -10 -5 0 -10 5 -10 10 0 6 5 10
   10 10 6 0 10 -4 10 -10z m-24 -25 c14 -13 14 -17 0 -31 -15 -14 -17 -14 -32 0
   -12 13 -13 18 -2 30 16 19 16 19 34 1z m72 -18 c4 -20 -25 -34 -40 -19 -15 15
   -1 44 19 40 10 -2 19 -11 21 -21z"/>
   </g>
   </svg>),
    );

    let mail = entry(
        "Mail",
        "mailto:ZennDev@protonmail.com",
        html!(<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-envelope-fill" viewBox="0 0 16 16">
    <path d="M.05 3.555A2 2 0 0 1 2 2h12a2 2 0 0 1 1.95 1.555L8 8.414.05 3.555ZM0 4.697v7.104l5.803-3.558L0 4.697ZM6.761 8.83l-6.57 4.027A2 2 0 0 0 2 14h12a2 2 0 0 0 1.808-1.144l-6.57-4.027L8 9.586l-1.239-.757Zm3.436-.586L16 11.801V4.697l-5.803 3.546Z"/>
  </svg>),
    );
    html! {
        <div id="links" class="links" >
            <div id="links-title" class="links-title is-italic" data-aos="zoom-out"
            data-aos-duration="1000" data-aos-offset="">
                {"Links"}
            </div>
            <div class="columns">
                {github}
                {discord}
            </div>
            <div class="columns">
                {arduboy}
                {mail}
            </div>
        </div>
    }
}
