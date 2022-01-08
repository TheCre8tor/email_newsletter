use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

use log;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state! -->
    pool: web::Data<PgPool>,
) -> HttpResponse {
    log::info!(
        "Adding Subscriber -> Name: '{}' Email: '{}' as a new subscriber.",
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in the database.");

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
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log::error!("Failed to execute query: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
