use yew::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
  html!(
    <h1 class="text-3xl text-white font-mplus font-bold text-center mx-auto my-14">
      {"Not found!"}
    </h1>
  )
}
