use yew::{function_component, html, Html};

#[function_component]
pub fn Pgp() -> Html {
    html! {
        <div id="pgp-keys" class="pgp-keys" >
            <div id="pgp-keys-title" class="pgp-keys-title is-italic" data-aos="zoom-out"
        data-aos-duration="1000" data-aos-offset="">
                {"PGP-Key"}
            </div>
            {pgpkey()}
        </div>
    }
}

fn pgpkey() -> Html {
    html!(
        <div class="pgp-key" data-aos="fade-left"
                data-aos-duration="1000" data-aos-offset="180">
                <p>
                {"-----BEGIN PGP PUBLIC KEY BLOCK-----"}
                <br/>
                <br/>
                {"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"}
                <br/>
                {"=F9wK"}
                <br/>
                {"-----END PGP PUBLIC KEY BLOCK-----"}
                </p>
            </div>
    )
}
