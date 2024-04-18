use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/PaginaTeste")]
    PaginaTeste,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <App /> },
        Route::PaginaTeste => html! { <PaginaTeste /> },
    }
}
