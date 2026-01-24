//
// Outer API - presented by a worker that proxies the requests to DO instances.
//
use axum::{
    Router,
    //body::Body,
    http::StatusCode,
    extract::{Path, State},
    response::Json,
    routing::{get, post}
};
use axum_macros::debug_handler;
use tower_service::Service;     // needed for '.call()'

use serde::{Serialize, Deserialize};
//use serde_json::json;
use time::format_description::well_known::Iso8601;
use time::UtcDateTime::{self};
use worker::{ Context, Env, Headers, HttpRequest, Method, Request, RequestInit, console_debug, event };

use crate::weather::Sample as WeatherSample;

// Router as initialized in 'worker-rs' 'examples/axum' example:
//  -> https://github.com/cloudflare/workers-rs/blob/main/examples/axum/src/lib.rs
//
fn router(env: Env) -> Router {
    let r = Router::new();
    #[cfg(feature = "_hello_api")]
    let r = r.route("/hello", get(hello));    // just for testing
    r
        .route("/:location", post(save_handler))
        .route("/:location", get(list_handler))
        .with_state(env)
}

#[cfg(feature = "_hello_api")]
async fn hello() ->&'static str {
    "Hello from Worker!"
}

#[derive(Serialize, Deserialize)]
struct Sample {
    when: String, // ISO8601
    temperature_c: f32
}

/*
* Body:
*   <<
*       { "samples": [ { "when": "2025-01-01T12:00:00Z", "temperature_c": 3.5 }, ... ] }
*   <<
*/
#[worker::send]
#[debug_handler]
async fn save_handler(
    Path(location): Path<String>,
    State(env): State<Env>,
    Json(payload): Json<Vec<Sample>>
) -> Result<String, axum::http::StatusCode> {

    let stub = env.durable_object("WEATHER")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .id_from_name(&location)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get_stub()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let v: Vec<WeatherSample> = payload.into_iter()
        .map(| Sample{ when, temperature_c } | {
            let when_s = parse_iso8601_to_u64(&when)
                .map_err(|_| StatusCode::BAD_REQUEST)?;     // tbd. could tell 'when' was not ISO8601
            Ok(WeatherSample::new(when_s, temperature_c))
        })
        .collect::<Result<_,axum::http::StatusCode>>()?;

    let body = serde_json::to_string(&v).unwrap();

    let mut tmp;
    let req = {
        // Note: Using '.unwrap()' within here - we are not working on unexpected input.
        //
        tmp = RequestInit::new();
        let tmp = tmp
            .with_method(Method::Post)
            .with_headers({
                let hh = Headers::new();
                hh.set("content-type", "application/json").unwrap();
                hh
            })
            .with_body(Some(body.into()));

        Request::new_with_init("http://_/save", &tmp).unwrap()
    };

    console_debug!("C, req: {:?}", req);

    let _resp = stub.fetch_with_request(req).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("".into())
}

/*
* Gets existing samples, for a location (all of them), in ascending order of their time stamps.
*
*   <<
*       { "samples": [ { "when": "2025-01-01T12:00:00Z", "temperature_c": 3.5 }, ... ] }
*   <<
*/
#[worker::send]
#[debug_handler]
async fn list_handler(
    Path(location): Path<String>,
    State(env): State<Env>
) -> Result<Json<Vec<Sample>>, axum::http::StatusCode> {

    let stub = env.durable_object("WEATHER")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .id_from_name(&location)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get_stub()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut resp = stub.fetch_with_str("/list").await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let raw: Vec<WeatherSample> = resp.json().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let a_sorted_asc = raw.into_iter()
        .map(|WeatherSample{ when: ts, temperature_c }| Sample {
            when: to_iso8601(ts),
            temperature_c,
        })
        .collect();

    Ok(Json(a_sorted_asc))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>, worker::Error> {

    // Spread the request via 'axum's pipe.
    //
    Ok(router(env).call(req).await?)
}

/*
* Unix epoch *seconds* to ISO 8601.
*/
fn to_iso8601(ts: u64) -> String {

    let x = UtcDateTime::from_unix_timestamp((ts * 1000) as _)
        .expect("a meaningful time stamp");

    x.format(&Iso8601::DEFAULT).unwrap().to_owned()
}

/*
* ISO 8601 to Unix epoch *seconds*. Milliseconds, if existing in the string, are truncated away.
*/
fn parse_iso8601_to_u64(s: &str) -> Result<u64, time::error::Parse> {

    let x = UtcDateTime::parse(s, &Iso8601::DEFAULT)?;
    Ok(x.unix_timestamp() as _)
}
