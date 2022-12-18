use anyhow::Result;
use yew::{html, Html, Component};

use common::litter::LitterData;
use crate::page::routes::AnimalLink;
use crate::common::table::{RowProps, TableWithTags};

fn tags() -> RowProps {
    vec! [
        "id miotu".into(),
        "id ojca".into(),
        "id matki".into(),
    ]
}

fn get_animal_link(id: &str) -> Html {
    html! {
        <AnimalLink id={id.to_owned()} />
    }
}

fn litter_to_row(litter: &LitterData) -> RowProps {
    vec! [
        litter.id.clone().into(),
        get_animal_link(&litter.id_father),
        get_animal_link(&litter.id_mother)
    ]
}

pub struct LitterList {
    litters: Option<Vec<LitterData>>
}

impl Component for LitterList {
    type Message = Result<Vec<LitterData>>;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future({
            async move {
                let litters = backend_api::get_litter_list().await;
                litters
            }
        });
        LitterList { litters: None }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.litters = msg.ok();
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
        <>
        <h2>{ "Lista Miotów" }</h2>
        {match &self.litters {
            None => html!{},
            Some(animals) => {
                let animal_list: Vec<RowProps> = animals.iter()
                    .map(litter_to_row).collect();
                let tags = tags();
                html! {
                    <div id="animal_list">
                        <TableWithTags tags={tags} data={animal_list} />
                    </div>
                }
            },
        }}
        </>
        }
    }
}