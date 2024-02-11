use std::vec;

use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};

use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .route("/games", get(list_games))
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    title: String,
}

async fn index() -> impl IntoResponse {
    IndexTemplate {
        title: "Home".to_string(),
    }
}

#[derive(Template)]
#[template(path = "pages/list_games.html")]
struct ListGamesTemplate {
    title: String,
    games: Vec<Game>,
}

async fn list_games() -> impl IntoResponse {
    ListGamesTemplate {
        title: "Games".to_string(),
        games: vec![
            Game { id: 1 },
            Game { id: 2 },
            Game { id: 3 },
            Game { id: 4 },
            Game { id: 5 },
            Game { id: 6 },
        ],
    }
}

struct Game {
    id: u32,
}
