use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Adding '{}' '{}' as a new subscriber",
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
                 INSERT INTO subscriptions (id, email, name, subscribed_at)
                 VALUES ($1, $2, $3, $4)
                 "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details hae been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::info!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
