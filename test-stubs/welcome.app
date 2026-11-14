html {
    lang: "en",
    data-bs-theme: "dark",
    class: "h-100",
    head {
        meta { charset: "UTF-8" }
        meta { name: "UTF-8", content: "width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0" }
        meta { http-equiv: "X-UA-Compatible", content:"ie=edge" }
        title { text: "Welcome to Wire" }
        link {
            ref: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/bootstrap/5.3.9/css/bootstrap.min.css",
            integrity: "sha512-ZuV2yPmu5OcwTgfUHURSM2QYEMPA9UIPpMr/U9gJEdRB+1VcCJaUc/GJrO9WwZVNR3JB7mgzkRtRwor+BcaPAA==",
            crossorigin: "anonymous",
            referrerpolicy: "no-referrer",
        }
    }
    body {
        class: "h-100",
        div {
            class: "h-100 container",
            div {
                class: "h-100 d-flex justify-content-center align-items-center",
                h1 {
                    text: "Hello world!",
                }
                p {
                    text: "This is the first page!",
                }
            }
        }
        script { src: "https://cdnjs.cloudflare.com/ajax/libs/bootstrap/5.3.9/js/bootstrap.min.js" }
    }
}
