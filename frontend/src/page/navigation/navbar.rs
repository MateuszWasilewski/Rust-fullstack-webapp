use crate::page::navigation::links::Link;
use crate::page::navigation::paths;
use leptos::*;

#[component]
fn NavLink(cx: Scope, href: &'static str, text: &'static str) -> impl IntoView {
    view! {
        cx,
        <li class="nav-item dropdown">
            <Link href={href.to_owned()} text={text.to_owned()}/>
        </li>
    }
}

#[component]
fn DropdownLink(cx: Scope, href: &'static str, text: &'static str) -> impl IntoView {
    view! {
        cx,
        <li class="dropdown-item">
            <Link href={href.to_owned()} text={text.to_owned()}/>
        </li>
    }
}

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                <a class="navbar-brand" href={paths::HOME}>"Baza Myszy"</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
            <ul class="navbar-nav">
                <NavLink href={paths::ANIMAL_LIST} text="Lista Myszy"/>
                <NavLink href="/phenotypes" text="Lista Fenotypów"/>
                <NavLink href={paths::LITTER_LIST} text="Lista Miotów"/>
                <li class="nav-item dropdown">
                <div class="btn-group">
                    <a class="nav-link dropdown-toggle" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                        {"Dodaj"}
                    </a>
                    <ul class="dropdown-menu">
                        <DropdownLink href="/litter" text="Miot"/>
                        <DropdownLink href="/animal" text="Mysz"/>
                        <DropdownLink href="/phenotype" text="Fenotyp"/>
                    </ul>
                </div>
                </li>
                // <SearchBar/>
            </ul>
            </div>
            </div>
        </nav>
    }
}
