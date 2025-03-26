use sycamore::prelude::*;

pub fn NavBar() -> View {
    view! {
        nav {
            ul(id="section1") {
                li {
                    a(href="/") { "Home" }
                }
                li {
                    a(href="/about") { "About" }
                }
            }
            ul(id="section2") {
                span {
                    "SECTION 2"
                }
                li {
                    a(href="/") { "Home" }
                }
                li {
                    a(href="/about") { "About" }
                }
                span {
                    "SECTION 3"
                }
            }
        }
    }
}
