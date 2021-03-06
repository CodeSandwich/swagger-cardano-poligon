extern crate cardano_node_api;
extern crate futures;
extern crate hyper;
extern crate swagger;
extern crate tokio_proto;

use cardano_node_api::{Api, ApiError, ApiV1NodeInfoGetResponse, ApiV1NodeSettingsGetResponse,
                       ApiV1NextUpdateGetResponse, ApiV1RestartNodePostResponse, Context};
use cardano_node_api::server::Service;
use futures::Future;
use hyper::{Request, Response};
use hyper::server::Http;
use tokio_proto::TcpServer;

fn main() {
    println!("Hello world!");
    let addr = "127.0.0.1:8080".parse().expect("Failed to parse bind address");
    TcpServer::new(Http::new(), addr)
        .serve(|| Ok(MyService::new()));
}

struct MyService {
    service: Service<MyApi>,
}

impl MyService {
    fn new() -> Self {
        MyService {
            service: Service::new(MyApi),
        }
    }
}

impl hyper::server::Service for MyService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let context = Context::default();
        self.service.call((req, context))
    }
}

#[derive(Clone)]
pub struct MyApi;

impl Api for MyApi {
    fn api_v1_node_info_get(&self, _force_ntp_check: Option<bool>, _context: &Context) -> Box<Future<Item=ApiV1NodeInfoGetResponse, Error=ApiError>> {
        println!("Hello api_v1_node_info_get!");
        Box::new(futures::failed(ApiError("Hello api_v1_node_info_get!".to_string())))
    }

    fn api_v1_node_settings_get(&self, _context: &Context) -> Box<Future<Item=ApiV1NodeSettingsGetResponse, Error=ApiError>> {
        println!("Hello api_v1_node_settings_get!");
        Box::new(futures::failed(ApiError("Hello api_v1_node_settings_get!".to_string())))
    }

    fn api_v1_next_update_get(&self, _context: &Context) -> Box<Future<Item=ApiV1NextUpdateGetResponse, Error=ApiError>> {
        println!("Hello api_v1_next_update_get!");
        Box::new(futures::failed(ApiError("Hello api_v1_next_update_get!".to_string())))
    }

    fn api_v1_restart_node_post(&self, _context: &Context) -> Box<Future<Item=ApiV1RestartNodePostResponse, Error=ApiError>> {
        println!("Hello api_v1_restart_node_post!");
        Box::new(futures::failed(ApiError("Hello api_v1_restart_node_post!".to_string())))
    }
}
