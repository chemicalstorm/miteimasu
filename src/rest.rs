use std::convert::Infallible;

use warp::hyper::StatusCode;
use warp::Filter;

use super::settings::Settings;

pub fn routes(
    settings: Settings,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_v1 = warp::path!("api" / "v1" / ..);

    let health = api_v1
        .and(warp::get())
        .and(warp::path("health"))
        .map(|| StatusCode::NO_CONTENT);

    let camera_list = api_v1
        .and(warp::get())
        .and(warp::path("cameras"))
        .and(with_settings(settings))
        .and_then(list_cameras);

    health.or(camera_list)
}

fn with_settings(
    settings: Settings,
) -> impl Filter<Extract = (Settings,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || settings.clone())
}

async fn list_cameras(settings: Settings) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&settings.cameras))
}
