//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

#[durable_object]
pub struct MyDurableObject {
    // tbd. own state fields here
    state: State,
    env: Env,
}

impl DurableObject for MyDurableObject {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

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
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    Response::ok("Hello World!")
}
