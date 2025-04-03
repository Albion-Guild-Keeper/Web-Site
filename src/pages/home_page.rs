use sycamore::prelude::*;
use sycamore_rstml::html;

#[derive(Props)]
pub struct HomePageProps {
    signal: Signal<bool>,
}
#[component]
pub fn HomePage(props: HomePageProps) -> View {
    view! {
        section(id="home") {
            p {
                "This is the home page of the website."
            }
            button(on:click=move |_| {
                // Call the callback function to update the signal in main.rs
                props.signal.set(!props.signal.get());
            }) { "Update Main Signal" }
        }
    }
}
