//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

#[cfg(feature = "_rpc_emul")]
use serde_wasm_bindgen;
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

        #[cfg(false)]
        let o: RpcCall = {
            let s = req.text().await?;
            console_debug!("TEXT: {}", s);  // "[object Object]"
            let js = wasm_bindgen::JsValue::from_str(&s);
            console_debug!("JS: {:?}", js);
            serde_wasm_bindgen::from_value(js)
                .map_err(|e| worker::Error::RustError(format!("RPC decode failed: {:?}", e)))?
        };

        // EI toimi: '.json()' ei toimi yhdessä 'serde_wasm_bindgen':n kanssa
        let o: RpcCall = {
            let x = req.json().await?;
            serde_wasm_bindgen::from_value(x)?
        };

        //console::debug_1("o: {:?}", o);
        console_log!("o: {:?}", o);

        match o {
            RpcCall::Abc { name } => Response::ok( self.abc(name.into()) ),
        }

        //Response::ok("ok")
    }
}

#[event(fetch)]
async fn fetch(
    _req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    //let id = env.durable_object("MY_DO")?         // Rust: "creates a temporary value which is freed while still in use"
    //    .id_from_name("abc")?;
    let _ns = env.durable_object("MY_DO")?;     // keep accessible, so DO API doesn't have a dangling ref
    let id = _ns.id_from_name("abc")?;
    let stub = id.get_stub()?;  // DurableObjectStub

    // Rust RPC is not there, yet
    #[cfg(not(feature = "_rpc_emul"))]
    let resp = stub.abc("def").await?;

    // WAS: just call inner 'fetch' (no serde)
    // Note: host (and protocol) need to be given, but are not used!!!
    #[cfg(false)]
    stub.fetch_with_str("http://_/abc").await?;

    //console_debug!("xxx: {:?}", "xxx");   // OK
    console_debug!("A");

    #[cfg(feature = "_rpc_emul")]
    let resp = {
        let o = RpcCall::Abc { name: "xyz".into() };
        let req = {
            let body = serde_wasm_bindgen::to_value(&o)?;   // JsValue

            console_debug!("B {:?}", body);

            Request::new_with_init("http://_/rpc", &RequestInit {
                method: Method::Post,
                body: Some(body),
                headers: {
                    let hh = Headers::new();
                        hh.set("content-type", "application/json")?;
                        hh
                },
                ..Default::default()
            })
        }?;

        console_debug!("C, req: {:?}", req);
        stub.fetch_with_request(req).await?
    };

    Ok(resp)
}
