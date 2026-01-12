# Track


## Ability to do RPC, within Rust


- [(no tracking entry)](...) ..for Rust RPC API, similar to JS/TS.
	

The JavaScript/Typescript API has [RPC's for Durable Objects](https://developers.cloudflare.com/durable-objects/best-practices/create-durable-object-stubs-and-send-requests/#invoking-methods-on-a-durable-object). This is sweet.

Unfortunately, `worker` doesn't, at the moment (Jan'26), so the author needed to create their own (likely sub-optimal) RPC, between two `fetch` sections.

### If we had RPC's

DO side: 

```
impl MyDurableObject {
    fn abc(&self, name: String) -> String {
        format!("Hey: {name}")
    }
}
```

Worker side:

```
let resp = stub.abc("def").await?;
```

..i.e. we can *directly* call methods within the `MyDurableObject` implementation.

- No need for an inner `fetch` within the DO code
- No need to use dummy URL: `http://_/{...}`, nor build a request worker -> DO
- No need for an `RpcCall` enum
- No need for `serde`, `serde_wasm_bindgen`

### Now

DO side:

```
impl DurableObject for MyDurableObject {
	// ...

    // All calls come through here (no RPC in Rust, yet).
    //
    async fn fetch(&self, mut req: Request) -> Result<Response> {
        let o = {
            let s = req.text().await?;
            let js = wasm_bindgen::JsValue::from_str(&s);
            serde_wasm_bindgen::from_value(js)
                .map_err(|e| worker::Error::RustError(format!("RPC decode failed: {:?}", e)))?
        };

        // EI toimi: '.json()' ei toimi yhdessä 'serde_wasm_bindgen':n kanssa
        //let x: JsValue = req.json().await?;
        //let o = serde_wasm_bindgen::from_value(x)?;

        match o {
            RpcCall::Abc { name } => Response::ok( self.abc(name) ),
        }
    }
}
```

Worker side:

```
    let resp = {
        let o = RpcCall::Abc { name: "xyz".into() };
        let req = {
            let body = serde_wasm_bindgen::to_value(&o)?;   // JsValue

            Request::new_with_init("http:://_/abc", &RequestInit {
                method: Method::Post,
                body: Some(body),
                ..Default::default()
            })
        }?;

        stub.fetch_with_request(req).await?
    };
```

- Need for an inner `fetch` within the DO code
- Need to use dummy URL: `http://_/{...}`, nor build a request worker -> DO
- Need for an `RpcCall` enum
- Need for `serde`, `serde_wasm_bindgen`

