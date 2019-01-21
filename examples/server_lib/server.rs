//! Server implementation of Cardano Node API.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;


use swagger;

use Cardano Node API::{Api, ApiError, Context,
                      ApiV1NodeInfoGetResponse,
                      ApiV1NodeSettingsGetResponse,
                      ApiV1NextUpdateGetResponse,
                      ApiV1RestartNodePostResponse
};
use Cardano Node API::models;

#[derive(Copy, Clone)]
pub struct Server;

impl Api for Server {

    /// Retrieves the dynamic information for this node.
    fn api_v1_node_info_get(&self, force_ntp_check: Option<bool>, context: &Context) -> Box<Future<Item=ApiV1NodeInfoGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("api_v1_node_info_get({:?}) - X-Span-ID: {:?}", force_ntp_check, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Retrieves the static settings for this node.
    fn api_v1_node_settings_get(&self, context: &Context) -> Box<Future<Item=ApiV1NodeSettingsGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("api_v1_node_settings_get() - X-Span-ID: {:?}", context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Version of the next update
    fn api_v1_next_update_get(&self, context: &Context) -> Box<Future<Item=ApiV1NextUpdateGetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("api_v1_next_update_get() - X-Span-ID: {:?}", context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Restart the node
    fn api_v1_restart_node_post(&self, context: &Context) -> Box<Future<Item=ApiV1RestartNodePostResponse, Error=ApiError>> {
        let context = context.clone();
        println!("api_v1_restart_node_post() - X-Span-ID: {:?}", context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
