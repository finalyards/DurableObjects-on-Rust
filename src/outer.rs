//
// Outer API - presented by a worker that proxies the requests to DO instances.
//
use axum::{
    Router,
    //body::Body,
    http::{Response, StatusCode},
    extract::{Path, State},
    response::Json,
    routing::{get, post}
};
use axum_macros::debug_handler;
//Ruse tower_service::Service;

use serde::{Serialize, Deserialize};
use time::format_description::well_known::Iso8601;
use time::UtcDateTime::{self};
use worker::*;

// Router as initialized in 'worker-rs' 'examples/axum' example:
//  -> https://github.com/cloudflare/workers-rs/blob/main/examples/axum/src/lib.rs
//
fn router(env: Env) -> Router {
    let r = Router::new();
    #[cfg(feature = "_hello")]
    let r = r.route("/hello", get(hello));    // just for testing
    r
        .route("/:location", post(save_handler))
        .route("/:location", get(list_handler))
        .with_state(env)
}

#[cfg(feature = "_hello")]
async fn hello() ->&'static str {
    "Hello from Worker!"
}

#[derive(Serialize, Deserialize)]
struct Sample {
    when: String, // ISO8601
    temperature_c: f32
}

#[derive(Serialize, Deserialize)]
struct SampleInner {
    when: u64,      // _secs_ since UNIX Epoch
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
) -> Result<Json<()>, StatusCode> {

    let stub = env.durable_object("WEATHER")?
        .id_from_name(&location)?
        .get_stub()?;

    let vv: Vec<(u64, f32)> = payload.into_iter()
        .map(| Sample{ when, temperature_c } | {
            let when = parse_iso8601_to_u64(&when)?;
            Ok((when, temperature_c))
        })
        .collect::<Result<_,_>>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let req = {
        Request::new_with_init("http://_/save", RequestInit::new()
            .with_method(Method::Post)
            .with_headers({
                let hh = Headers::new();
                hh.set("content-type", "application/json")?;
                hh
            })
            .with_body(Some(vv))
        )
    }?;

    console_debug!("C, req: {:?}", req);
    let resp = stub.fetch_with_request(req).await?;

    //stub.fetch_with_method_and_body("/save", "POST", vv).await
    //    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(resp)
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
) -> Result<Json<Vec<Sample>>, StatusCode> {

    let stub = env.durable_object("WEATHER")?
        .id_from_name(&location)?
        .get_stub()?;

    let raw: Vec<(u64, f32)> = stub.fetch("list").await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let a_sorted_asc = raw.into_iter()
        .map(|(ts, temp_c)| Sample {
            when: format_iso8601(ts),
            temperature_c: temp_c,
        })
        .collect();

    Ok(Json(a_sorted_asc))
}

// The Worker 'fetch'
//
// We use 'axum' library for parsing the incoming requests. Check for correctness of the requests,
// authentication etc. - like a bouncer at a bar. Durable Object 'fetch' gets passed only well-formed
// requests.
//
#[event(fetch)]
#[cfg(true)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {

    // Run the CloudFlare request via 'axum's pipe.
    //
    Ok(router(env).call(req).await?)
}

#[event(fetch)]
#[cfg(false)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    router(env).run(req, env).await
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



/***R
    /_*** tbd.
    let stub = {
        let _ns = env.durable_object("MY_DO")?;     // keep accessible, so DO API doesn't have a dangling ref
        let id = _ns.id_from_name(location)?;
        id.get_stub()?
        ***_/

    let resp = {
        let req = {
            // NOTE: 'serde_wasm_bindgen' IS THE WRONG TOOL to form a request to DO. Don't. Won't work.
            //let body = serde_wasm_bindgen::to_value(&o)?;   // JsValue

            Request::new_with_init("http://_/rpc", RequestInit::new()
                .with_method(Method::Post)
                .with_headers({
                    let hh = Headers::new();
                    hh.set("content-type", "application/json")?;
                    hh
                })
                .with_body(Some(rpc_body(&o)?.into()))
            )
        }?;

        console_debug!("C, req: {:?}", req);
        stub.fetch_with_request(req).await?
    };

    Ok(resp)

}
***/
