Page [extends: root] [marker-directive] @[load: do_thing()] {
    title: "",
    description: "",
    body {
        h1 {
            text: "Hello world!"
        }
        button @[click: show_msg = true] {
            text: "Show Paragraph"
        }
        p [if: show_msg] {
            html: "This is some <b>bold</b> and <em>italic</em> text. This is both <b><em>italic and bold</em></b>"
        }
        p @[click: hello()] {
            text: "my_var = {my_var}"
        }
        input {
            value: 1
        }
        input {
            value: 1 + 1
        }
        input {
            value: 1 - 1
        }
        input {
            value: my_num + 1
        }
        input {
            value: my_num + my_obj.my_field
        }
        div
            [repeat: post in posts]
            [class: "featured" if post.featured elif post.pinned "" else "not-featured"]
            [class: "popular" if post.votes > 5]
        {
            class: "custom-class",
            p {
                a {
                    href: "https://sturdyframework.com",
                    target: "_blank",
                    text: "post.title"
                }
            }
        }
        CustomComponent {
            customAttribute: "",
        }
        div {}
    }
}
