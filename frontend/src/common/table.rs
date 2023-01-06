use yew::{function_component, html, AttrValue, Html, Properties};

pub type RowProps = Vec<Html>;

#[derive(PartialEq, Properties)]
struct RowWithStyleProps {
    style: AttrValue,
    data: RowProps,
}

#[function_component(RowWithStyle)]
fn get_row_with_style(props: &RowWithStyleProps) -> Html {
    let columns = props
        .data
        .iter()
        .map(|data| {
            html! {
                <div class="col">
                    { data.to_owned() }
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <div class={props.style.to_string()}>
            { columns.to_owned() }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableWithTagsProps {
    pub tags: RowProps,
    pub data: Vec<RowProps>,
}

#[function_component(TableWithTags)]
pub fn get_table(props: &TableWithTagsProps) -> Html {
    let rows = props
        .data
        .iter()
        .map(|row| {
            html! {
                <RowWithStyle style={"row border-top"} data={row.clone()} />
            }
        })
        .collect::<Html>();
    html! {
        <>
            <RowWithStyle style={"row fst-italic"} data={props.tags.to_owned()}/>
            { rows }
        </>
    }
}
