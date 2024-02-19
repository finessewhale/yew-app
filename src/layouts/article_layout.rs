use yew::prelude::*;

use crate::components::Nav;
use crate::components::Footer;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Html,
}

#[function_component]
pub fn ArticleLayout(props: &Props) -> Html {
  html!(
  <main class="pb-8">
    <Nav/>
    <div class="container mx-auto max-w-[604px] px-4 pt-14 md:pt-20 w-full flex flex-col items-center">
      {props.children.clone()}
      <Footer/>
    </div>
  </main>
  )
}
