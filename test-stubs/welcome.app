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
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bootstrap@6.0.0-alpha.1/dist/css/bootstrap.min.css",
            integrity: "sha384-B/GM4XqrwHnWXNOWMbloTmrYXZg10cakYGmpfsR/bbzQ6JAJI4ihuyADKLnBgrCe",
            crossorigin: "anonymous",
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
        script {
            type: "module",
            src: "https://cdn.jsdelivr.net/npm/bootstrap@6.0.0-alpha.1/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-1a/pXj49ZQ1aHEmrJ+gMw1otqoVsYwlEnlD8mIfY2TV03r20Y0CN7uqx1tQogjPL",
            crossorigin: "anonymous"
        }
    }
}
