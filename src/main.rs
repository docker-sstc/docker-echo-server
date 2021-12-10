#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate rand;

use std::io::Write;
use std::path::Path;
use futures::{future};
// use http::{HeaderMap};

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
// use hyper::http::{
//     Result
// };
// use hyper::header::{Connection, Headers, UserAgent};
use chrono::prelude::Utc;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use env_logger::{Builder, Env};

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn gen_id (len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect()
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
    let id = gen_id(6);
    let mut builder = Response::builder();

    builder.status(StatusCode::OK); // default

    info!("[{}] incoming path: {}", id, req.uri().path());

    // handle content-type by extname
    let path = Path::new(req.uri().path());

    let ext = match path.extension() {
        None => "",
        Some(os_str) => {
            match os_str.to_str() {
                Some("json") => {
                    builder.header("content-type", "application/json");
                    "json"
                }
                _ => ""
            }
        }
    };

    // feature: overwrite http status
    let headers = req.headers();
    if headers.contains_key("x-echo-status") {
        let req_status = headers
            .get("x-echo-status").unwrap()
            .to_str().unwrap();
        match req_status.parse::<u16>() {
            Ok(code) => {
                match StatusCode::from_u16(code) {
                    Ok(status) => {
                        debug!("[{}] header x-echo-status: {} received, response with it.", id, req_status);
                        builder.status(status);
                    }
                    Err(e) => {
                        error!("[{}] header x-echo-status: {} received, but parse to http status failed: {}", id, req_status, e);
                        builder
                            .status(StatusCode::BAD_REQUEST)
                            .header("x-echo-status-error", format!("{}", e));
                    }
                }
            }
            Err(e) => {
                error!("[{}] header x-echo-status: {} received, but parse to u16 failed: {}", id, req_status, e);
                builder
                    .status(StatusCode::BAD_REQUEST)
                    .header("x-echo-status-error", format!("{}", e));
            },
        }
    }

    let body = match req.method() {
        // handle preflight
        &Method::OPTIONS => {
            let mut h_list = vec![];
            if headers.contains_key("Origin") {
                let k = "Access-Control-Allow-Origin";
                let v = headers.get("Origin").unwrap().to_str().unwrap();
                h_list.push(format!("{}: {}", k, v));
                builder.header(k, v);
            }
            if headers.contains_key("Access-Control-Request-Method") {
                let k = "Access-Control-Allow-Methods";
                let v = headers.get("Access-Control-Request-Method").unwrap().to_str().unwrap();
                h_list.push(format!("{}: {}", k, v));
                builder.header(k, v);
            }
            if headers.contains_key("Access-Control-Request-Headers") {
                let k = "Access-Control-Allow-Headers";
                let v = headers.get("Access-Control-Request-Headers").unwrap().to_str().unwrap();
                h_list.push(format!("{}: {}", k, v));
                builder.header(k, v);
            }
            debug!("[{}] Method {} received. Response with preflight headers: {:?}", id, req.method(), h_list.join(", "));
            Body::empty()
        }
        &Method::HEAD => {
            debug!("[{}] Method {} received. Response with empty body.", id, req.method());
            Body::empty()
        }
        // &Method::GET |
        // &Method::POST |
        // &Method::PUT |
        // &Method::DELETE |
        // &Method::PATCH
        _ => {
            let p = &req.uri().path();
            if p.len() >= 3 && &p[..3] == "/_/" {
                info!("[{}] Request path is prefix with `/_/`. It's system api!", id);
                // system api can't be overwritten
                match path.file_stem() {
                    None => {
                        builder.status(StatusCode::NOT_FOUND);
                        Body::empty()
                    },
                    Some(os_str) => {
                        match os_str.to_str() {
                            Some("version") => {
                                if ext == "json" {
                                    Body::from(format!("\"{}\"", VERSION))
                                } else {
                                    Body::from(VERSION)
                                }
                            }
                            _ => {
                                builder.status(StatusCode::NOT_FOUND);
                                Body::empty()
                            }
                        }
                    }
                }
            } else {
                let body = req.into_body();
                debug!("[{}] Incoming request body: {:?}", id, body);
                body
            }
        }
    };
    Box::new(future::ok(builder.body(body).unwrap()))
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{:?}] {}: {}",
                Utc::now(),
                record.level(),
                record.args()
            )
        })
        .init();

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| error!("server error: {}", e));

    info!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
