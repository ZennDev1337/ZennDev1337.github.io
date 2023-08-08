window.onload = function () {
    /* particlesJS.load(@dom-id, @path-json, @callback (optional)); */
    setTimeout(() => {
        particlesJS.load("particles-js", "particles.json", function () {
            console.log("callback - particles.js config loaded");
        });
        AOS.init();
        hack("hacker-title", 3, 20);
        hack("hacker-subtitle", 3, 20);
        hack("projects-title", 7, 20);
    }, 50);

    // window.scrollTo(0, 0);
    // console.log(window.location.href);
    // if (window.location.href.includes("#")) {
    //     window.location.assign("");
    // }
};

window.onbeforeunload = function () {};
