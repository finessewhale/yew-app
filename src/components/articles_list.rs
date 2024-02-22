use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::types::Article;


#[derive(Properties, PartialEq)]
pub struct ArticlesListProps {
  pub articles: Vec<Article>
}

#[function_component]
pub fn ArticlesList(ArticlesListProps { articles }: &ArticlesListProps ) -> Html {
  articles.iter().map(|article| html!(
    <p class="text-white w-full flex flex-col mb-8" key={article.id}>
      <h2 class="text-md opacity-40">
        {&article.published_at}
      </h2>
      <Link<Route> to={Route::Article {id: article.id}}>
        <span class="text-3xl font-bold font-mplus hover:opacity-40 transition-all">
          {&article.title}
        </span>
      </Link<Route>>
    </p>
  )).collect()
}
