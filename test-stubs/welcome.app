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
                div {
                    h1 [if: myVar] {
                        text: "Hello world!",
                    }
                    p [else] {
                        text: "This is the first page!",
                    }
                }
                div {
                    class: "posts",
                    div [for: post in posts] {
                        id: "post_" + post.id,
                        h1 {
                            text: post.title,
                        }
                        p {
                            text: "Written by " + post.author + " on " + post.date,
                        }
                        span [if: post.featured] {
                            class: "featured",
                            text: "Featured article"
                        }
                        span [else] {
                            class: "guest",
                            text: "Guest article"
                        }
                    }
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
