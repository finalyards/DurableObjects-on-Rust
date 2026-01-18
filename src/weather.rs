//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;

use worker::web_sys::console;

#[durable_object]
pub struct WeatherSamples<'a> {   // measurements at a certain location (e.g. "Helsinki")
    //location: String,   // e.g. "Helsinki" (the Durable Object id; perhaps reachable via 'env'???)
    my_id: ObjectId<'a>,

    // Note: We wouldn't _really_ need to keep this here, since things get persisted in the SQLite.
    //data: Vec<(u64 /*instant*/, TemperatureC)>,

    // common fields
    sql: SqlStorage,

    // Some samples keep these around. But don't use them.
    //state: State,   // needed? tbd. what are the fields?
    //env: Env,       // needed? tbd. what are the fields?
}

struct TemperatureC(f32);

impl WeatherSamples {
    // We don't have RPC (worker being able to call the methods of the class), yet, in Rust (Jan'26);
    // JS/TS has proceeded to that. Keeping the methods separate (and unaware of "inner fetch") in
    // case the RPC ability becomes available in Rust, too, one day. 🌞

    // If writing to SQLite fails, the DO system will anyways restart the DO, right(?).
    //
    fn save(&mut self, vv: Vec<(u64 /*instant*/, TemperatureC)>) {

        // DO magic allows us to make a loop; it will handle *everything* as a transaction. Yay!
        for (a,b) in vv {
            self.sql.exec("INSERT INTO samples(timestamp_ms, temp_c) VALUES (?,?);",
                          &[a,b.into()]
            ).expect("write to SQLite");
        }
    }

    fn list(&self) -> Result<Vec<(u64 /*instant*/, TemperatureC)>> {

        let rows: Vec<(u64 /*instant*/, f32)> = self.sql
            .exec("SELECT timestamp_ms, temp_c FROM samples ORDER BY timestamp_ms ASC;", None)?
            .to_array()?;

        rows.map(|(a,b)| { (a, TemperatureC(b))})
    }
}

impl DurableObject for WeatherSamples {
    fn new(state: State, _env: Env) -> Self {
        let sql = state.storage().sql();

        sql.exec("CREATE TABLE IF NOT EXISTS \
            samples ( timestamp_ms INTEGER PRIMARY KEY, temp_c REAL NOT NULL );", None)
            .expect("table created");

        let id = state.id();
        console_debug!("Created DO: id={}", id);

        Self { my_id: id, sql }
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
