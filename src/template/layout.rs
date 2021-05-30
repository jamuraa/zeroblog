markup::define! {
    Layout<'a, Body: markup::Render>(
        title: &'a str,
        body: Body
    ) {
        @markup::doctype()
        html {
            head {
                title { @title }
            }
            body {
                @body
            }
        }
    }
}
