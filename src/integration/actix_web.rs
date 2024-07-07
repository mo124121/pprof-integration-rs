use actix_web::{get, web, HttpResponse, Responder};
use crate::profiling::{generate_profile, ProfileParams};

#[get("/debug/pprof/profile")]
async fn pprof_profile_actix(params: web::Query<ProfileParams>) -> impl Responder {
    let duration = params.seconds.unwrap_or(30);
    match generate_profile(duration).await {
        Ok(body) => HttpResponse::Ok()
                    .content_type("application/octet-stream")
                    .append_header(("Content-Disposition", "attachment; filename=\"profile.pb.gz\""))
                    .body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(pprof_profile_actix);
}
