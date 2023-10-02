use leptos::*;

pub type RowView = Vec<View>;

pub trait Listable {
    fn get_column_tags(&self) -> ReadSignal<RowView>;

    fn get_rows(&self) -> ReadSignal<Vec<RowView>>;
}

#[component]
fn RowWithStyle(style: &'static str, row: ReadSignal<RowView>) -> impl IntoView {
    let row_view = move || {
        row.get()
            .iter()
            .map(|field_data| {
                view!(
                    <div class="col">
                        { field_data }
                    </div>
                )
            })
            .collect_view()
    };
    view!(
        <div class={style}>
            {row_view}
        </div>
    )
}

fn list_to_view(rows: ReadSignal<Vec<RowView>>) -> impl IntoView {
    rows.get()
        .iter()
        .map(|row| {
            let row = row.clone();
            let (row, _) = create_signal(row);
            view!(<RowWithStyle style="row border-top" row={row}/>)
        })
        .collect_view()
}

#[component]
pub fn List<T>(data: T) -> impl IntoView
where
    T: Listable,
{
    let tags = data.get_column_tags();

    let rows = data.get_rows();
    let get_rows = move || list_to_view(rows);

    view!(
        <RowWithStyle style="row fst-italic" row={tags}/>
        {get_rows}
    )
}
