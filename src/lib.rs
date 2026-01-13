//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

use worker::web_sys::console;

#[durable_object]
pub struct MyDurableObject {
    // tbd. own state fields here
    state: State,
    env: Env,
}

impl MyDurableObject {
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
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    // All calls come through here (no RPC in Rust, yet).
    //
    #[cfg(feature = "_rpc_emul")]
    async fn fetch(&self, mut req: Request) -> Result<Response> {
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

#[event(fetch)]
async fn fetch(
    _req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let stub = {
        let _ns = env.durable_object("MY_DO")?;     // keep accessible, so DO API doesn't have a dangling ref
        let id = _ns.id_from_name("abc")?;
        id.get_stub()?
    };

    // Rust RPC is not there, yet
    #[cfg(not(feature = "_rpc_emul"))]
    let resp = stub.abc("def").await?;

    // WAS: just call inner 'fetch' (no serde)
    // Note: host (and protocol) need to be given, but are not used!!!
    #[cfg(false)]
    stub.fetch_with_str("http://_/abc").await?;

    #[cfg(feature = "_rpc_emul")]
    let resp = {
        let o = RpcCall::Abc { name: "xyz".into() };
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

            /*** #alternatively
            Request::new_with_init("http://_/rpc", &RequestInit {
                method: Method::Post,
                body: Some(rpc_body(&o)?.into()),
                headers: {
                    let hh = Headers::new();
                        hh.set("content-type", "application/json")?;
                        hh
                },
                ..Default::default()
            })***/
        }?;

        console_debug!("C, req: {:?}", req);
        stub.fetch_with_request(req).await?
    };

    Ok(resp)
}

use serde::Serialize;
use wasm_bindgen::JsValue;
use worker::Result;

#[cfg(feature = "_rpc_emul")]
pub fn rpc_body<T: Serialize>(value: &T) -> Result<JsValue> {
    let json = serde_json::to_string(value)
        .map_err(|e| worker::Error::RustError(format!("JSON serialization failed: {}", e)))?;
    Ok(JsValue::from_str(&json))
}
