use std::vec;

use askama::Template;
use axum::{extract::State, response::IntoResponse, routing::get, Router};

use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    active_page: String,
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    let state = AppState { active_page: "Home".to_string()};

    let mut router = Router::new()
        .route("/", get(index))
        .route("/games", get(list_games))
        .route("/about", get(about))
        .with_state(state)
        .nest_service("/assets", ServeDir::new("assets"));

    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    Ok(router.into())
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    title: String,
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    IndexTemplate {
        state: State,
        title: "Home".to_string(),
    }
}

#[derive(Template)]
#[template(path = "pages/about.html")]
struct AboutTemplate {
    title: String,
}

async fn about(State(state): State<AppState>) -> impl IntoResponse {
    AboutTemplate {
        title: "About".to_string(),
    }
}


#[derive(Template)]
#[template(path = "pages/list_games.html")]
struct ListGamesTemplate {
    title: String,
    games: Vec<Game>,
}

async fn list_games(State(state): State<AppState>) -> impl IntoResponse {
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

