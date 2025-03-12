use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::functions::is_auth::is_auth;

#[component]
pub async fn Login() -> View {
    let result = is_auth().await.unwrap();
    console_log!("{:?}", result);
    
    let input = create_signal(String::new());
    
    // use sycamore_web::events::KeyboardEvent;
    // let on_keydown = move |ev: KeyboardEvent| {
    //     if ev.key() == "Enter" && !input.with(String::is_empty) {
    //         // Reset the input.
    //         input.set(String::new());
    //     }
    // };
    // <input bind:value=input on:keydown=on_keydown /> @todo interessante per fare qualcosa quando si preme invio

    let handle_login = move |_| {
        console_log!("Login clicked");

        console_log!("Input: {:?}", input);
    };
    
    html! {
        <article>
            <input bind:value=input />
            <button on:click=handle_login> "Login with Discord" </button>
            <p>  "Is Auth: "  { result.to_string() } </p>
            <p>  "Input: "  { input } </p>
        </article>
    }
}
