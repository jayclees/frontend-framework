App {
    title: "",
    description: "",
    link {
        rel: "stylesheet",
    }
    body {
        class: "d-flex justify-center align-items-center"
        div {
            h1 [if: my_var] {
                text: "Hello world!",
            }
            p {
                text: "This is a paragraph.",
            }
        }
    }
}
