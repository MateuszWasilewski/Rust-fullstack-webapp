use wasm_bindgen::JsCast;
use yew::{Event, function_component, html, Html, Properties, Callback, MouseEvent};
use web_sys::HtmlInputElement;

pub fn get_text_value(input: Event) -> String {
    let target = input.target().unwrap();
    let element = target.unchecked_into::<HtmlInputElement>();
    element.value()
}

#[derive(PartialEq, Properties)]
pub struct InputFormProps {
    pub action: Callback<Event>,
    pub text: String,
    pub id: String,
}

#[function_component]
pub fn InputForm(props: &InputFormProps) -> Html {
    let onchange = props.action.clone();
    let id = props.id.clone();
    let text = props.text.clone();
    html!{
        <div class="row g-3 align-items-center">
            <div class="col-auto">
                <label for={id.clone()} class="col-form-label">{text}</label>
            </div>
            <div class="col-auto">
                <input type={"text"} id={id} class="form-control" {onchange}/>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct DropdownFormProps {
    pub id: String,
    pub text: String,
    pub options: Vec<(String, Callback<MouseEvent>)>,
    pub default: Option<(String, Callback<MouseEvent>)>
}

#[function_component]
pub fn DropdownForm(props: &DropdownFormProps) -> Html {
    let id = props.id.clone();
    let text = props.text.clone();
    let options = props.options.iter().map(|(text, onclick)| {
        html!{
            <option {onclick}>{text}</option>
        }
    }).collect::<Html>();
    
    html! {
        <div class="row g-3 align-items-center">
            <div class="col-auto">
                <label for={id.clone()} class="col-form-label">{text}</label>
            </div>
            <select class="form-select" id={id} style="width: auto" >
                if let Some((text, onclick)) = &props.default {
                    <option {onclick}>{text}</option>
                }
                {options}
            </select>
        </div>
    }
}