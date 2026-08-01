// this is a comment
# and another comment
/*
hello
*/


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
            html: "This is some <b>bold</b> and <em>italic</em> text. This is both <b><em>italic</em></b>"
        }
        p {
            text: "my_var = {my_var}"
        }
        div
            [repeat: post in posts]
            [class: "featured" if post.featured elif post.pinned "" else "not-featured"]
            [class: "popular" if post.votes > 5]
        {
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
        div {}
    }
}
