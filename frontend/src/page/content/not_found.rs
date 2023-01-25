
use yew::{function_component, Html, html};


#[function_component]
pub fn NotFound() -> Html {
  html!{
    <h2>{"Nie znaleziono strony o podanym adresie"}</h2>
  }
}