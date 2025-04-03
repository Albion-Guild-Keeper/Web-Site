use sycamore::prelude::*;
use sycamore_rstml::html;

use crate::components::login_button_component::LoginButton;

#[derive(Props)]
pub struct LoginDialogProps {
    open: bool,
}
#[component]
pub fn LoginDialog(props: LoginDialogProps) -> View {
    let open = create_signal(props.open);
    html! {
        <dialog id="login-dialog" class={if open.get() { "open" } else { "" }}>
            <form method="dialog" class="login-form">
                <h2> "Login" </h2>
                <LoginButton/>
                </form>
        </dialog>
    }
}