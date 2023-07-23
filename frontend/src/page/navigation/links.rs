use leptos::{component, view, IntoView, Scope};
use leptos_router::A;

use crate::page::navigation::paths;

#[component]
pub fn Link(
    cx: Scope,
    href: String,
    text: String,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let class = format!("nav-link active {}", class.unwrap_or_default());
    view!(cx,
        <A class={class} href={href}>{text}</A>
    )
}

#[component]
pub fn AnimalLink(cx: Scope, id: String) -> impl IntoView {
    view!(cx,
    <Link href={paths::animal(&id)} text={id} class={"text-primary".to_owned()} />
    )
}

#[component]
pub fn LitterLink(cx: Scope, id: String) -> impl IntoView {
    view!(cx,
    <Link href={paths::litter(&id)} text={id} class={"text-primary".to_owned()} />
    )
}

pub fn get_animal_link(cx: Scope, id: String) -> impl IntoView {
    view!(cx,
        <AnimalLink id={id} />
    )
}

pub fn get_litter_link(cx: Scope, id: String) -> impl IntoView {
    view!(cx,
        <LitterLink id={id} />
    )
}

pub fn get_optional_animal_link(cx: Scope, id: Option<String>) -> impl IntoView {
    id.map(|id| get_animal_link(cx, id))
}

pub fn get_optional_litter_link(cx: Scope, id: Option<String>) -> impl IntoView {
    id.map(|id| get_litter_link(cx, id))
}
