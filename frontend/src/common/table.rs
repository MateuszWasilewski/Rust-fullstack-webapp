use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct RowProps {
    pub data: Vec<Html>
}

#[derive(PartialEq, Properties)]
pub struct TableWithTagsProps {
    pub tags: RowProps,
    pub data: Vec<RowProps> 
}

#[function_component(Row)]
pub fn get_row(props: &RowProps) -> Html {
    let columns = props.data.iter().map(|data| html! {
        <div class="col">
            { data.to_owned() }
        </div>
    }).collect::<Html>();
    html! {
        <div class="row border-top">
            { columns.to_owned() }
        </div>
    }
}

#[function_component(RowTags)]
pub fn get_row_tags(props: &RowProps) -> Html {
    let columns = props.data.iter().map(|data| html! {
        <div class="col">
            { data.to_owned() }
        </div>
    }).collect::<Html>();
    html! {
        <div class="row fst-italic">
            { columns.to_owned() }
        </div>
    }
}

#[function_component(TableWithTags)]
pub fn get_table(props: &TableWithTagsProps) -> Html {
    let rows = props.data.iter().map(|row| {
        html! {
            <Row data={row.data.to_owned()} />
        }
    }).collect::<Html>();
    html! {
        <>
            <RowTags data={props.tags.data.to_owned()}/>
            { rows }
        </>
    }
}