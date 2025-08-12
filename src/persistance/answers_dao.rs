use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|e| {
            DBError::InvalidUUID(format!(
                "uuid parse failed {} {} ",
                answer.question_uuid,
                e.to_string()
            ))
        })?;

        let record = sqlx::query!(
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| match e {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                        return DBError::InvalidUUID(format!(
                            "invalid uuid: {}",
                            answer.question_uuid
                        ));
                    }
                }
                DBError::Other(Box::new(e))
            }
            _ => DBError::Other(Box::new(e)),
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content.to_string(),
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {

        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|e| {
            DBError::InvalidUUID(format!(
                "uuid parse failed {} {} ",
                answer_uuid,
                e.to_string()
            ))
        })?;

        // TODO maybe log number of raws affected
        sqlx::query!(
            r#"
                                DELETE FROM answers WHERE answer_uuid = $1
            "#,
            uuid,
        )
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid).map_err(|e| {
            DBError::InvalidUUID(format!(
                "uuid parse failed {} {} ",
                question_uuid,
                e.to_string()
            ))
        })?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|record| AnswerDetail {
                answer_uuid: record.answer_uuid.to_string(),
                question_uuid: record.question_uuid.to_string(),
                content: record.content.clone(),
                created_at: record.created_at.to_string(),
            })
            .collect::<Vec<AnswerDetail>>();

        Ok(answers)
    }
}
