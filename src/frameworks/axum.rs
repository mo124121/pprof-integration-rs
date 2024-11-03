use crate::profiling::{generate_profile, ProfileParams};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

pub async fn pprof_profile_axum(
    Query(params): Query<ProfileParams>,
) -> Result<Response, StatusCode> {
    let duration = params.seconds.unwrap_or(3);
    match generate_profile(duration).await {
        Ok(body) => Ok((
            StatusCode::OK,
            [("Content-Type", "application/octet-stream")],
            [(
                "Content-Disposition",
                "attachment; filename=\"profile.pb.gz\"",
            )],
            body,
        )
            .into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn app() -> Router {
    Router::new().route("/debug/pprof/profile", get(pprof_profile_axum))
}
