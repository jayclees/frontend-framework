// this is a comment
# and another comment
Page {
    title: "",
    description: "",
    body {
        h1 {
            text: "Hello world!"
        }
        button {
            class: ""
            text: counter
            @click: my_func()
        }
        p {
            html: "This is some <b>bold</b> and <em>italic</em> text. This is both <b><em>italic</em></b>"
        }
        div {
            [repeat]: post in posts
            [class]:
                - "featured" if post.featured
                - "test"
            class: "custom-class"
            p {
                a {
                    href: "https://sturdyframework.com"
                    target: "_blank"
                    text: post.title
                }
            }
        }
        CustomComponent {
            customAttribute:
        }
    }
}
