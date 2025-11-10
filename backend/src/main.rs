use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use backend::entities;
use chrono::NaiveDate;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Serialize)]
struct PostResponse {
    id: i32,
    title: String,
    date: NaiveDate,
    slug: String,
    tags: Vec<String>,
    description: String,
}

async fn get_posts(State(state): State<AppState>) -> impl IntoResponse {
    let posts = entities::front_matter::Entity::find()
        .all(&state.conn)
        .await;
    match posts {
        Ok(results) => {
            let response: Vec<PostResponse> = results
                .into_iter()
                .map(|fm| PostResponse {
                    id: fm.id,
                    title: fm.title,
                    date: fm.date,
                    slug: fm.slug,
                    tags: fm.tags,
                    description: fm.description,
                })
                .collect();

            Json(response).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

pub async fn run() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("Failed to find DATABASE_URL");
    let conn: DatabaseConnection = Database::connect(database_url).await?;

    let state = AppState { conn };

    let app = Router::new()
        .route("/posts", get(get_posts))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    match run().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {e}")
        }
    }
}
