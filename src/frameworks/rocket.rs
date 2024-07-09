use rocket::get;
use rocket::http::Status;
use rocket::routes;

use crate::profiling::generate_profile;

#[get("/debug/pprof/profile?<seconds>")]
async fn pprof_profile_rocket(seconds: Option<u64>) -> Result<Vec<u8>, Status>  {
    let duration = seconds.unwrap_or(30);
    if let Ok(body) = generate_profile(duration).await {
        Ok(body)
    } else {
        Err(Status::InternalServerError)
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![pprof_profile_rocket]
}
