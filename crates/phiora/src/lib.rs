//! # phiora
//!
//! The Rust client of the Phiora service (`phiadk/phiora`, FastAPI over
//! Postgres): "the first ECS database", where kinds, items and links are the
//! entities, components and systems that worlds, agents and assets query the
//! way they run.
//!
//! This crate does one thing: it asks the service what something is. It reads
//! metadata, the way Vision reads a Core ML model's description before it calls
//! the model (`VNCoreMLRequest.h`). It holds no HTTP code: a [`Transport`]
//! carries the request, so the crate depends on nothing above `serde` and
//! stays below every crate that uses it. A caller supplies the transport.
//!
//! Whatever an answer says is a proposal for the caller to check. Nothing here
//! decides anything for the Reader.
//!
//! The routes and shapes follow the service as read from
//! `phiadk/phiora/src/api/v1` and `src/schemas` (kinds, items, `ora/query`).
//! They were not run against a live service here.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// A failure to ask or to understand the answer.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The transport could not carry the request.
    Transport(String),
    /// The answer was not the shape the service documents.
    Shape(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Transport(m) => write!(f, "transport: {m}"),
            Error::Shape(m) => write!(f, "shape: {m}"),
        }
    }
}

impl std::error::Error for Error {}

/// Carries one request to the service and returns the response body.
pub trait Transport {
    /// `GET` a path under `/api/v1`, for example `/kinds`.
    fn get(&self, path: &str) -> Result<String, Error>;
    /// `POST` a JSON body to a path under `/api/v1`.
    fn post(&self, path: &str, body: &str) -> Result<String, Error>;
}

/// A kind: what a thing is, with the spec that says what it may hold.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Kind {
    pub name: String,
    #[serde(default)]
    pub app: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub spec: Value,
}

/// One PhiSQL chain as JSON, the body of `POST /ora/query`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Query {
    /// The kind to look in.
    pub map: String,
    /// Conditions, for example `{"status": "live", "usd >": 10}`.
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub has: Map<String, Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "with")]
    pub with: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderBy")]
    pub order_by: Option<String>,
    pub limit: u32,
    pub offset: u32,
}

impl Query {
    /// A query for a kind with the service's default page of 20.
    pub fn kind(name: &str) -> Self {
        Query { map: name.to_string(), has: Map::new(), with: Vec::new(), order_by: None, limit: 20, offset: 0 }
    }
}

/// The answer to a query.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Rows {
    pub items: Vec<Map<String, Value>>,
    pub total: u64,
}

/// The client. It owns a transport and nothing else.
pub struct Phiora<T: Transport> {
    transport: T,
}

impl<T: Transport> Phiora<T> {
    pub fn new(transport: T) -> Self {
        Phiora { transport }
    }

    /// Every kind the service holds (`GET /kinds`).
    pub fn kinds(&self) -> Result<Vec<Kind>, Error> {
        let body = self.transport.get("/kinds")?;
        serde_json::from_str(&body).map_err(|e| Error::Shape(e.to_string()))
    }

    /// One kind by name (`GET /kinds/{name}`): the description of a thing,
    /// asked for before acting on it.
    pub fn describe(&self, name: &str) -> Result<Kind, Error> {
        let body = self.transport.get(&format!("/kinds/{name}"))?;
        serde_json::from_str(&body).map_err(|e| Error::Shape(e.to_string()))
    }

    /// Run a PhiSQL chain (`POST /ora/query`).
    pub fn query(&self, q: &Query) -> Result<Rows, Error> {
        let body = serde_json::to_string(q).map_err(|e| Error::Shape(e.to_string()))?;
        let answer = self.transport.post("/ora/query", &body)?;
        serde_json::from_str(&answer).map_err(|e| Error::Shape(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// A transport that answers from a fixed script and records what it was asked.
    struct Script {
        reply: String,
        seen: RefCell<Vec<(String, String)>>,
    }

    impl Transport for Script {
        fn get(&self, path: &str) -> Result<String, Error> {
            self.seen.borrow_mut().push((path.to_string(), String::new()));
            Ok(self.reply.clone())
        }
        fn post(&self, path: &str, body: &str) -> Result<String, Error> {
            self.seen.borrow_mut().push((path.to_string(), body.to_string()));
            Ok(self.reply.clone())
        }
    }

    fn script(reply: &str) -> Script {
        Script { reply: reply.to_string(), seen: RefCell::new(Vec::new()) }
    }

    #[test]
    fn describe_asks_for_the_kind_by_name_and_reads_it() {
        let s = script(r#"{"name":"club","app":"phiegg","label":"Club","spec":{"fields":["usd"]}}"#);
        let p = Phiora::new(s);
        let k = p.describe("club").unwrap();
        assert_eq!(k.name, "club");
        assert_eq!(k.app.as_deref(), Some("phiegg"));
        assert_eq!(p.transport.seen.borrow()[0].0, "/kinds/club");
    }

    #[test]
    fn a_kind_with_no_optional_fields_still_reads() {
        let p = Phiora::new(script(r#"[{"name":"look"}]"#));
        let ks = p.kinds().unwrap();
        assert_eq!(ks.len(), 1);
        assert_eq!(ks[0].app, None);
        assert_eq!(ks[0].spec, Value::Null);
    }

    #[test]
    fn a_query_is_sent_in_the_services_own_spelling() {
        let p = Phiora::new(script(r#"{"items":[{"id":"a"}],"total":1}"#));
        let mut q = Query::kind("look");
        q.has.insert("status".into(), Value::from("live"));
        q.with.push("clubId".into());
        q.order_by = Some("usd".into());
        let rows = p.query(&q).unwrap();
        assert_eq!(rows.total, 1);
        let (path, body) = p.transport.seen.borrow()[0].clone();
        assert_eq!(path, "/ora/query");
        let sent: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(sent["map"], "look");
        assert_eq!(sent["orderBy"], "usd");
        assert_eq!(sent["with"][0], "clubId");
        assert_eq!(sent["limit"], 20);
    }

    #[test]
    fn an_answer_in_the_wrong_shape_is_an_error_not_a_guess() {
        let p = Phiora::new(script("not json"));
        assert!(matches!(p.describe("club"), Err(Error::Shape(_))));
    }
}
