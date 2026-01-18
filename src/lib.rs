mod axum;
mod weather_samples;

//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

use worker::web_sys::console;


/*** TEMP
#[durable_object]
pub struct MyDurableObject2 {   // measurements of a certain location (e.g. "Helsinki")

    // Note: We wouldn't _really_ need to keep this here, since things get persisted in the SQLite.
    data: Vec<(u64 /*Instant*/, TemperatureC)>,

    // common fields
    state: State,
    env: Env,
}

struct TemperatureC(f32);

impl MyDurableObject2 {
    fn abc(&self, name: String) -> String {
        format!("Hey: {name}")
    }
}

#[cfg(feature = "_rpc_emul")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
enum RpcCall {
    Abc { name: String },
}

impl DurableObject for MyDurableObject {
    fn new(state: State, _env: Env) -> Self {
        let sql = state.storage().sql();

        // Create table if it does not exist
        sql.exec("CREATE TABLE IF NOT EXISTS \
            samples ( timestamp_ms INTEGER PRIMARY KEY, temp_c REAL NOT NULL );", None)
            .expect("table created");

        Self { state }  // tbd. or 'sql'
    }

    // The DO (inner) 'fetch'
    //
    // These calls come only from the Worker. Thus, we know they are guarded.
    //
    async fn fetch(&self, mut req: Request) -> Result<Response> {

        // "/{location}
        //
        let p = req.path();
        console_debug!("Path: {}", p);

        match req.method() {
            // POST to '/{location}'
            //  - body: JSON
            Method::Post => {
                let location = p;
                let body = req.body();

                spawn_to(location, ); //..refined request
            },

            case Method::Get -> {

            },

            case _ -> {
                error
            }
        }

        // Jos 'POST', lisää mittaustiedot (voivat olla ei-järjestyksessä)
        self.sql( "INSERT INTO samples (timestamp_ms, temp_c) VALUES (?, ?)", (ts_ms, temp_c), ).await?;

        // Jos 'GET', palauta kaikki (aikajärjestyksessä)


        //let result = self.state.storage().sql("SELECT * FROM items", ()).await?;

        #[cfg(true)]
        let o: RpcCall = {
            let s = req.text().await?;
            console_debug!("TEXT: {}", s);  // "{"Abc":{"name":"xyz"}}"

            serde_json::from_str(&s)?
        };

        console_log!("o: {:?}", o);

        match o {
            RpcCall::Abc { name } => Response::ok( self.abc(name.into()) ),
        }
    }
}
***/

/***RRR
//--- tbd. outer.rs
//
// The Worker 'fetch'
//
// Check for correctness of the requests, authentication etc. - like a bouncer at a bar. Durable
// Object 'fetch' gets passed only well-formed requests.
//
#[event(fetch)]
async fn outer_fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {

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
}
***/
