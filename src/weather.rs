//use worker::{durable_object, DurableObject, State, Env, Result, Request, Response};
use worker::*;
use serde::{Deserialize, Serialize};

#[durable_object]
#[derive(Clone)]
pub struct Weather {   // measurements at a certain location (e.g. "Helsinki")
    location: String,   // e.g. "Helsinki" (the Durable Object's id)

    // common fields
    sql: SqlStorage,
}

struct TemperatureC(f32);

/***Rimpl From<f32> for TemperatureC {
    fn from(v: f32) -> Self { Self(v) }
}***/

/*** tbd. maybe???
use serde::{Deserializer, Serializer};

impl Serialize for TemperatureC {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        //serializer.serialize_newtype_struct("TemperatureC", &self.0)
        serializer.serialize_f32(self.0)        // as a plain number in JSON
    }
}

impl<'de> Deserialize<'de> for TemperatureC {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        let v = deserializer.deserialize_f32()?;
        Ok(TemperatureC(v))
    }
}
***/

// We ship JSON in/out just using an array (no enclosing object). If you want one, just make it,
// e.g. '{ samples: [ ... ] }'.
//
// tbd. API classes to their own mod
#[derive(Deserialize, Serialize)]
pub(crate) struct Sample {
    pub(crate) when: u64,            // outer 'fetch' converts ISO8601 <-> u64
    pub(crate) temperature_c: f32,
}

impl Sample {
    pub(crate) fn new(when: u64, temperature_c: f32) -> Self { Self { when, temperature_c } }
}

impl Weather {
    // On Rust side (as of Jan'26), 'workers-rs' does not provide RPC's between worker->DO. These
    // methods are in the anticipation, that might change. I.e. we might not need the "inner fetch
    // handler".

    // If writing to SQLite fails, the DO system will anyways restart the DO, right(?).
    //
    // Inner mutability, in the way that calling this changes the *persisted* data in SQLite!!
    //
    fn save(&self, vv: Vec<(u64 /*instant*/, TemperatureC)>) -> Result<()> {

        // DO magic allows us to make a loop; it will handle *everything* as a transaction. Yay!
        for (a,b) in vv {
            let ab: Vec<SqlStorageValue> = vec![
                SqlStorageValue::Integer(a as i64),
                SqlStorageValue::Float(b.0 as f64)
            ];

            self.sql.exec("INSERT INTO samples(timestamp_s, temp_c) VALUES (?,?);", ab)
                .expect("write to SQLite");
        }

        Ok(())
    }

    fn list(&self) -> Result<Vec<(u64 /*instant*/, TemperatureC)>> {

        let rows: Vec<(u64 /*instant*/, f32)> = self.sql
            .exec("SELECT timestamp_s, temp_c FROM samples ORDER BY timestamp_ms ASC;", None)?
            .to_array()?;

        let a: Vec<(u64, TemperatureC)> = rows.iter().map(|(a,b)| { (*a, TemperatureC(*b)) })
            .collect();
        Ok(a)
    }
}

impl DurableObject for Weather {
    fn new(state: State, _env: Env) -> Self {
        let sql = state.storage().sql();

        sql.exec("CREATE TABLE IF NOT EXISTS \
            samples ( timestamp_s INTEGER PRIMARY KEY, temp_c REAL NOT NULL );", None /*no parameters*/)
            .expect("table created");

        let location: String = state.id().to_string();
        console_debug!("Created DO: location={}", location);

        Self { location, sql }
    }

    // The DO (inner) 'fetch'
    //
    // These calls come only from the Worker. They should be well structured (or at the least,
    // we don't need to protect against malicious use).
    //
    async fn fetch(&self, mut req: Request) -> Result<Response> {
        let path = req.path();
        let method = req.method();

        console_debug!("Path: {}", path);  // "/helsinki"

        let rr: Result<Response> = match (method, path.as_str()) {
            //---
            // POST '/save'
            //      body: JSON: '[ { when: <uint_sec>, temperature_c: <float> }, ... ]'
            //
            (Method::Post, "/save") => {
                console_debug!("Got POST: {}", self.location);

                let body: Vec<Sample> = req.json().await?;

                let vv: Vec<(u64, TemperatureC)> = body.into_iter()
                    .map(|Sample{ when, temperature_c: b }| (when,TemperatureC(b)))
                    .collect();

                self.save(vv)?;
                Response::empty()
            },

            //---
            // GET '/list'
            //  -> body: JSON: '[ { when: <uint_sec>, temperature_c: <float> }, ... ]'
            //
            (Method::Get, "/list") => {
                console_debug!("Got GET: {}", self.location);

                let out: Vec<Sample> = self.list()?.into_iter()
                    .map(|(ts_s, TemperatureC(b))| Sample {
                        when: ts_s,
                        temperature_c: b,
                    })
                    .collect();

                Response::from_json(&out)
            },

            _ => {
                Response::error("not found", 404)
            }
        };
        rr
    }
}
