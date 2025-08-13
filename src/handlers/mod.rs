use crate::AppState;
use crate::handlers::handlers_inner::HandlerError;
use crate::models::*;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::create_question(question, questions_dao.clone().as_ref()).await;

    match response {
        Ok(r_question) => Ok(Json(r_question).into_response()),
        Err(error) => Err(error.into_response()),
    }
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::read_questions(questions_dao.as_ref()).await;

    match response {
        Ok(r_questions) => Ok(Json(r_questions).into_response()),
        Err(error) => Err(error.into_response()),
    }
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::delete_question(question_uuid, questions_dao.as_ref()).await;

    match response {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into_response()),
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::create_answer(answer, answers_dao.as_ref()).await;

    match response {
        Ok(r_answer) => Ok(Json(r_answer).into_response()),
        Err(error) => Err(error.into_response()),
    }
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::read_answers(question_uuid, answers_dao.as_ref()).await;

    match response {
        Ok(r_answers) => Ok(Json(r_answers).into_response()),
        Err(error) => Err(error.into_response()),
    }
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await;

    match response {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into_response()),
    }
}
