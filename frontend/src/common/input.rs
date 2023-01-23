use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, EventTarget, InputEvent};
use yew::{function_component, html, Callback, Event, Html, MouseEvent, Properties};

fn target_to_text(target: EventTarget) -> Option<String> {
    let element = target.unchecked_into::<HtmlInputElement>();
    let text = element.value();
    if text == "" {
        None
    } else {
        Some(text)
    }
}

pub fn event_to_text(event: Event) -> Option<String> {
   event.target().and_then(target_to_text)
}

pub fn input_event_to_text(event: InputEvent) -> Option<String> {
    event.target().and_then(target_to_text)
}

#[derive(PartialEq, Properties)]
pub struct TextInputTempProps {
    pub onchange: Callback<Event>,
    pub text: String,
    pub id: String,
}

#[function_component]
pub fn TextInput(props: &TextInputTempProps) -> Html {
    let id = props.id.clone();
    let text = props.text.clone();
    let onchange = props.onchange.clone();

    html! {
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
pub struct DropdownFormTempProps {
    pub set_value: Callback<Option<String>>,
    pub id: String,
    pub text: String,
    pub options: Rc<Vec<String>>,
    pub default: Option<String>,
}

#[function_component]
pub fn DropdownForm(props: &DropdownFormTempProps) -> Html {
    let id = props.id.clone();
    let text = props.text.clone();
    let options = props
        .options
        .iter()
        .map(|value| {
            let value = value.clone();
            let text = value.clone();
            let set_value = props.set_value.clone();
            let onclick = Callback::from(move |_: MouseEvent| {
                set_value.emit(Some(value.clone()));
            });
            html! {
                <option {onclick}>{text}</option>
            }
        })
        .collect::<Html>();

    let default = props.default.clone().and_then(|text| {
        let onclick = {
            let set_value = props.set_value.clone();
            Callback::from(move |_: MouseEvent| set_value.emit(None))
        };
        Some((text, onclick))
    });

    html! {
        <div class="row g-3 align-items-center">
            <div class="col-auto">
                <label for={id.clone()} class="col-form-label">{text}</label>
            </div>
            <select class="form-select" id={id} style="width: auto" >
                if let Some((text, onclick)) = default {
                    <option {onclick} selected={true} >{text}</option>
                }
                {options}
            </select>
        </div>
    }
}
