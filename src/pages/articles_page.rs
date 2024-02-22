use yew::prelude::*;
use reqwasm::http::Request;
use crate::layouts::ArticleLayout;
use crate::components::ArticlesList;
use crate::types::Article;


#[function_component]
pub fn ArticlesPage() -> Html {
  let url = "https://api.tanzanite.space/articles";

  let articles = use_state(|| vec![]);

  {
    let articles = articles.clone();
    use_effect_with((), move |_| {
      let articles = articles.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let data:Vec<Article> = Request::get(url)
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();
        articles.set(data);
      });
      || ()
    });
  }

 html!(
    <ArticleLayout>
      <ArticlesList articles={(*articles).clone()}/>
    </ArticleLayout>
  )
}
