use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let port_from_cli = std::env::args()
        .nth(1)
        .expect("Please provide a port number as the first argument");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json_serialize", get(json_serialize));

    axum::Server::bind(&format!("0.0.0.0:{port_from_cli}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Clone)]
struct Post {
    id: u32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug, Serialize)]
struct JsonResponse {
    user_id: String,
    username: String,
    email: String,
    first_name: String,
    last_name: String,
    tags: Vec<String>,
    posts: Vec<Post>,
}

#[derive(Deserialize)]
struct QueryInput {
    user_list: Vec<String>,
}

async fn json_serialize(Query(query): Query<QueryInput>) -> impl IntoResponse {
    let mut posts = Vec::with_capacity(100);

    for v in 0..100 {
        posts.push(Post {
            id: v,
            title: "Hello, World!".to_string(),
            body: "Hello, World!".to_string(),
            published: true,
        });
    }

    let mut tags = Vec::with_capacity(100);

    for v in 0..100 {
        tags.push(format!("tag{}", v));
    }

    let mut users = Vec::new();

    for v in query.user_list {
        users.push(JsonResponse {
            user_id: v.clone(),
            username: format!("Username{v}"),
            email: format!("email{v}@something.com"),
            first_name: format!("FirstName{v}"),
            last_name: format!("LastName{v}"),
            tags: tags.clone(),
            posts: posts.clone(),
        });
    }

    axum::Json(users)
}
