use sycamore::prelude::*;
use sycamore_rstml::html;

#[component]
pub fn SignUp() -> View {
    let selected_tab = create_signal(1);

    let render_tab_content = move || match selected_tab.get() {
        1 => html! { <p> "Content for Step 1" </p> },
        2 => html! { <p> "Content for Step 2" </p> },
        3 => html! { <p> "Content for Step 3" </p> },
        4 => html! { <p> "Content for Step 4" </p> <button> "Submit" </button> },
        _ => html! { <p> "Invalid Step" </p> },
    };

    html! {
        <div>
            <h2> { format!("You are on Step {}", selected_tab.get()) } </h2>
            <br></br>
            <div>
            { render_tab_content() }
            </div>
            <br></br>
            <div>
                <button
                    on:click=move |_| {
                        if selected_tab.get() > 1 {
                            selected_tab.set(selected_tab.get() - 1);
                        }
                    }
                > "Previous" </button>
                <button
                    on:click=move |_| {
                        if selected_tab.get() < 4 {
                            selected_tab.set(selected_tab.get() + 1);
                        }
                    }
                > "Next" </button>
            </div>
        </div>
    }
}
