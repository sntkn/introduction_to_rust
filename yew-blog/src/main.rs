use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Properties, PartialEq)]
struct PostsListProps {
    posts: Vec<Post>,
    on_click: Callback<Post>,
}

#[function_component(PostList)]
fn posts_list(PostsListProps { posts, on_click }: &PostsListProps) -> Html {
    posts
        .iter()
        .map(|post| {
            let on_post_select = {
                let on_click = on_click.clone();
                let post = post.clone();
                Callback::from(move |_| on_click.emit(post.clone()))
            };
            html! {
                <p onclick={on_post_select}>{
                    format!("{} {}",
                    post.id, post.title)
                }</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct PostDetailProps {
    post: Post,
}

#[function_component(PostDetail)]
fn post_detail(PostDetailProps { post }: &PostDetailProps) -> Html {
    html! {
        <div>
            <h3>{ post.title.clone() }</h3>
            <p>{ post.body.clone() }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
