use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <>
      <h2>{"Witamy na stronie!"} </h2>
      <p>{"Strona stanowi interfejs graficzny do bazy myszy kolorowych, należacej oraz zarządzanej przez studentów z Sekcji Genetycznej Koła Naukowego Zoologów SGGW"}</p>
      <p>{"W chwili obecnej dostęp do bazy jest ogólnodostępny. W planach jest wprowadzenie możliwości logowania oraz uprawnień edycji."}</p>
      <p>{"Prosimy o nie naruszanie wprowadzonych danych, gdyż zwiększy to wysiłki na wprowadzenie ograniczeń dostępu."} </p>
      <p>{"Strona jest dostępna tylko dla osób posiadających link."}</p>
      </>
    }
}

