use axum::{
    Router,
    extract::Path,
    response::Json,
    routing::{get, post}
};
//R? use tower_service::Service;
use worker::*;

// our router
//
// tbd. is there any way we could 'let' (initialize only once), instead of having it a function?
//      Lazy init???
//
fn router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/:location", get(listSamples))
        .route("/:location", post(saveSamples))
}

// The Worker 'fetch'
//
// We use 'axum' library for parsing the incoming requests. Check for correctness of the requests,
// authentication etc. - like a bouncer at a bar. Durable Object 'fetch' gets passed only well-formed
// requests.
//
#[event(fetch)]
async fn outer_fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

async fn hello() ->&'static str {
    "Hello from Worker!"
}

async fn listSamples(Path(loc): Path<String>) -> axum::http::Response<axum::body::Body> {

    router.call
}

async fn saveSamples(loc: String) -> axum::http::Response<axum::body::Body> {

}

    // tbd. 'get_location()'
    let p = req.path();
    console_debug!("Path: {}", p);
    // "/apple-touch-icon.png" | "/apple-touch-icon-{...}.png"
    // "/favicon.ico"


    /*** tbd.
    let stub = {
        let _ns = env.durable_object("MY_DO")?;     // keep accessible, so DO API doesn't have a dangling ref
        let id = _ns.id_from_name(location)?;
        id.get_stub()?
        ***/

    let pat = Regex::new(r"^/([a-zA-Z_\-]+)$");
    let location = pat.captures(req.path());

    let (do_id, inner_req) = match (req.method(), req.path()) {
        // POST to '/{location}'
        //  - body: JSON
        (Method::Post, "/{location}") => {
            let location = p;
            let body = req.body();

            req;
        },

        // GET to '/{location}'
        //  -> body: JSON
        Method::Get => {
            let location = p;
            req
        },

        _ => {
            return Error:: 404
        }
    };

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
        ***/
}
