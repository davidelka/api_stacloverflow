#[macro_use]
extern crate log;

use dotenvy;
use pretty_env_logger;
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use log::info;

mod handlers;
mod models;
mod persistance;

use crate::persistance::answers_dao::{AnswersDao, AnswersDaoImpl};
use crate::persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};
use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(7)
        .connect(&std::env::var("DATABASE_URL").expect("env var missing"))
        .await
        .expect("Database connection failed!");

    // below are simple query test
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("fetching failed");

    info!("********* Question Records *********");
    info!("Found {} recs", recs.len());
    info!("{:?}", recs);

    info!("Starting server");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool.clone()));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
