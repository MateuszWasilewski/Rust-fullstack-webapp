use leptos::{component, view, IntoView};
use leptos_router::A;

use crate::page::navigation::paths;

#[component]
pub fn Link(href: String, text: String, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let class = format!("nav-link active {}", class.unwrap_or_default());
    view!(
        <A class={class} href={href}>{text}</A>
    )
}

#[component]
pub fn AnimalLink(id: String) -> impl IntoView {
    view!(
        <Link href={paths::animal(&id)} text={id} class={"text-primary".to_owned()} />
    )
}

#[component]
pub fn LitterLink(id: String) -> impl IntoView {
    view!(
        <Link href={paths::litter(&id)} text={id} class={"text-primary".to_owned()} />
    )
}

pub fn get_animal_link(id: String) -> impl IntoView {
    view!(
        <AnimalLink id={id} />
    )
}

pub fn get_litter_link(id: String) -> impl IntoView {
    view!(
        <LitterLink id={id} />
    )
}

pub fn get_optional_animal_link(id: Option<String>) -> impl IntoView {
    id.map(|id| get_animal_link(id))
}

pub fn get_optional_litter_link(id: Option<String>) -> impl IntoView {
    id.map(|id| get_litter_link(id))
}
