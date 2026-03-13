pub mod crawl;
pub mod events;
pub mod reviews;
pub mod live;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/crawl")
                    .route("/stats", web::get().to(crawl::stats))
                    .route("/history", web::get().to(crawl::history))
                    .route("/sources", web::get().to(crawl::sources))
            )
            .service(
                web::scope("/events")
                    .route("/trending", web::get().to(events::trending))
                    .route("/timeline", web::get().to(events::timeline))
            )
            .service(
                web::scope("/reviews")
                    .route("/stats", web::get().to(reviews::stats))
                    .route("/recent", web::get().to(reviews::recent))
            )
            .route("/live", web::get().to(live::stream))
    );
}
