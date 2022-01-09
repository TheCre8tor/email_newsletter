use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

use tracing::Instrument;

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
    let request_id: Uuid = Uuid::new_v4();

    let request_span = tracing::info_span!(
        /* Notice that we preﬁxed all of them with a % symbol:
        we are telling tracing to use their Display implementation
        for logging purposes. */
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Request ID: {}, New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            tracing::error!(
                "Request ID: {}, Failed to execute query: {:?}",
                request_id,
                err
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
