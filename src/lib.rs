use yew::prelude::*;
use yew_router::prelude::*;
mod router;
mod components;
mod layouts;
mod pages;
mod types;

use router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
  html!(
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  )
}


