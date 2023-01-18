use anyhow::Result;
use yew::{html, Component, Html};

use crate::common::table::{RowProps, TableWithTags};
use common::litter::LitterData;
use frontend_routing::links::{get_litter_link, get_animal_link};

fn tags() -> RowProps {
    vec!["id miotu".into(), "id ojca".into(), "id matki".into()]
}

fn litter_to_row(litter: &LitterData) -> RowProps {
    vec![
        get_litter_link(&litter.id),
        get_animal_link(&litter.id_father),
        get_animal_link(&litter.id_mother),
    ]
}

pub struct LitterList {
    litters: Option<Vec<LitterData>>,
}

impl Component for LitterList {
    type Message = Result<Vec<LitterData>>;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future({
            async move {
                let litters = backend_api::get::litter_list().await;
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
