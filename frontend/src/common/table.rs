use leptos::*;

pub type RowView = Vec<View>;

pub trait Listable {
    fn get_column_tags(&self, cx: Scope) -> ReadSignal<RowView>;

    fn get_rows(&self, cx: Scope) -> ReadSignal<Vec<RowView>>;
}

#[component]
fn RowWithStyle(cx: Scope, style: &'static str, row: ReadSignal<RowView>) -> impl IntoView {
    let row_view = move || {
        row.get()
            .iter()
            .map(|field_data| {
                view!(cx,
                    <div class="col">
                        { field_data }
                    </div>
                )
            })
            .collect_view(cx)
    };
    view!(cx,
        <div class={style}>
            {row_view}
        </div>
    )
}

fn list_to_view(cx: Scope, rows: ReadSignal<Vec<RowView>>) -> impl IntoView {
    rows.get()
        .iter()
        .map(|row| {
            let row = row.clone();
            let (row, _) = create_signal(cx, row);
            view!(cx, <RowWithStyle style="row border-top" row={row}/>)
        })
        .collect_view(cx)
}

#[component]
pub fn List<T>(cx: Scope, data: T) -> impl IntoView
where
    T: Listable,
{
    let tags = data.get_column_tags(cx);

    let rows = data.get_rows(cx);
    let get_rows = move || list_to_view(cx, rows);

    view!(cx,
        <RowWithStyle style="row fst-italic" row={tags}/>
        {get_rows}
    )
}
