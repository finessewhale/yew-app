use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::HomePage;
use crate::pages::ArticlesPage;
use crate::pages::ArticlePage;
use crate::pages::NotFoundPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    #[at("/articles/:id")]
    Article {id: usize},
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <HomePage/> },
    Route::Articles => html! { <ArticlesPage/> },
    Route::Article { id } => html!( <ArticlePage id={id}/>),
    Route::NotFound => html! { <NotFoundPage/> },
  }
}
