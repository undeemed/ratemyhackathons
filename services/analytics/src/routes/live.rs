use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use tokio::time::{interval, Duration};

use crate::db;

/// SSE endpoint: streams latest crawl + review events every 5 seconds.
pub async fn stream(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let pool = pool.into_inner();

    let stream = async_stream::stream! {
        let mut tick = interval(Duration::from_secs(5));
        let mut last_review_count: i64 = 0;
        let mut last_crawl_count: i64 = 0;

        loop {
            tick.tick().await;

            // Check for new reviews
            let review_count: Result<(i64,), _> = sqlx::query_as(
                "SELECT COUNT(*) FROM reviews"
            ).fetch_one(pool.as_ref()).await;

            // Check for new crawl entries
            let crawl_count: Result<(i64,), _> = sqlx::query_as(
                "SELECT COUNT(*) FROM crawl_sources"
            ).fetch_one(pool.as_ref()).await;

            let mut events = Vec::new();

            if let Ok((count,)) = review_count {
                if count > last_review_count && last_review_count > 0 {
                    let new = count - last_review_count;
                    events.push(format!(
                        "{{\"type\":\"review\",\"count\":{new},\"total\":{count}}}"
                    ));
                }
                last_review_count = count;
            }

            if let Ok((count,)) = crawl_count {
                if count > last_crawl_count && last_crawl_count > 0 {
                    let new = count - last_crawl_count;
                    events.push(format!(
                        "{{\"type\":\"crawl\",\"count\":{new},\"total\":{count}}}"
                    ));
                }
                last_crawl_count = count;
            }

            for event in events {
                let msg = format!("data: {event}\n\n");
                yield Ok::<_, actix_web::Error>(
                    actix_web::web::Bytes::from(msg)
                );
            }

            // Heartbeat
            yield Ok::<_, actix_web::Error>(
                actix_web::web::Bytes::from(":\n\n")
            );
        }
    };

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}
