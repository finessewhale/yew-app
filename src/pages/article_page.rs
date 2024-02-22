use yew::prelude::*;
use reqwasm::http::Request;
use crate::layouts::ArticleLayout;
use crate::types::Article;

#[derive(Properties, PartialEq)]
pub struct ArticleProperties {
  pub id: usize
}

#[function_component]
pub fn ArticlePage(ArticleProperties { id }: &ArticleProperties) -> Html {
  let url = format!("https://api.tanzanite.space/articles/{}",id);

  let article = use_state(|| Article {
    id: 0,
    title: "".to_string(),
    content:"".to_string(),
    published_at:"".to_string()
  });

  {
    let article = article.clone();
    use_effect_with((), move |_| {
      let articles = article.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let data:Article = Request::get(&url)
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
      <article class="mb-6">
          <div class="text-white mb-4">
            <h1 class="text-3xl font-mplus font-bold">
              {article.title.clone()}
            </h1>
            <h3 class="text-md font-mplus opacity-40">
              {article.published_at.clone()}
            </h3>
          </div>
        <p class="text-xl text-white">
          {article.content.clone()}
        </p>
      </article>
    </ArticleLayout>
  )
}
