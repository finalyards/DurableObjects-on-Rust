//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

#[durable_object]
pub struct MyDurableObject {
    // tbd. own state fields here
    state: State,
    env: Env,
}

impl MyDurableObject {
    fn abc(name: String) -> String {
        format!("Hey: {name}")
    }
}

impl DurableObject for MyDurableObject {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    // So far, the only ENTRY POINT into a Durable Object (in Rust)
    //
    async fn fetch(&self, _req: Request) -> Result<Response> {
        //let result = self.state.storage().sql("SELECT * FROM items", ()).await?;

        //Response::ok(format!("Rows: {:?}", result))

        // from sample
        //Response::ok(&format!("{} active users.", self.users.len()))

        Response::ok("ok")
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

    // Cannot do this in Rust, yet.
    #[cfg(false)]
    let resp = stub.abc("def").await?;

    // Need to use an inner 'fetch'  Object ('fetch_with_str', 'fetch_with_request')
    // Note: host (and protocol) need to be given, but are not used!!!
    #[cfg(true)]
    stub.fetch_with_str("http://_/abc").await

    //R Response::ok("Hello World!")
}
