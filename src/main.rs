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
use hyper::server::{Http, NewService};
use std::io;
use swagger::auth::AllowAllAuthenticator;
use tokio_proto::TcpServer;

fn main() {
    println!("Hello world!");

    let service_fn =
        cardano_node_api::server::auth::NewService::new(
            AllowAllAuthenticator::new(
                MyService,
                "cosmo"
        )
    );

    let addr = "127.0.0.1:8080".parse().expect("Failed to parse bind address");
    TcpServer::new(Http::new(), addr)
        .serve(service_fn);
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

pub struct MyService;

impl NewService for MyService {
    type Request = (Request, Context);
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Service<MyApi>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Service::new(MyApi))
    }
}
