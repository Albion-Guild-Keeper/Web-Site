use sycamore::prelude::*;
use sycamore_rstml::html;

pub fn home_page() -> View {
    html! {
        <h1> "Home Page" </h1>
        <p> "This is the content of the home page." </p>
        <p> "We are excited to share more with you soon!" </p>
    }
}

// ! @todo Nella home poi che ci sará da selezionare i discords/gilde si puó vedere in quale discord una persona é dentro grazie all'api 
// ! https://discord.com/api/users/@me/guilds 
// ! Puoi vedere tutti i discord nel quale la persona é dentro poi guardi
// ! se anche il bot é all'interno ed é abilitato e allora aggiungilo alla lista dei discords da selezionare