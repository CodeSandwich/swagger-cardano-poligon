#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, Context, ContextWrapper};

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "cardano-sl:0.0.0";


#[derive(Debug, PartialEq)]
pub enum ApiV1NodeInfoGetResponse {
    /// 
     ( models::ApiResponseNodeInfo ) ,
    /// Invalid &#x60;force_ntp_check&#x60;
    Invalid_ ,
}

#[derive(Debug, PartialEq)]
pub enum ApiV1NodeSettingsGetResponse {
    /// 
     ( models::ApiResponseNodeSettings ) ,
}

#[derive(Debug, PartialEq)]
pub enum ApiV1NextUpdateGetResponse {
    /// 
     ( models::ApiResponseV1SoftwareVersion ) ,
}

#[derive(Debug, PartialEq)]
pub enum ApiV1RestartNodePostResponse {
    /// 
     ,
}


/// API
pub trait Api {

    /// Retrieves the dynamic information for this node.
    fn api_v1_node_info_get(&self, force_ntp_check: Option<bool>, context: &Context) -> Box<Future<Item=ApiV1NodeInfoGetResponse, Error=ApiError>>;

    /// Retrieves the static settings for this node.
    fn api_v1_node_settings_get(&self, context: &Context) -> Box<Future<Item=ApiV1NodeSettingsGetResponse, Error=ApiError>>;

    /// Version of the next update
    fn api_v1_next_update_get(&self, context: &Context) -> Box<Future<Item=ApiV1NextUpdateGetResponse, Error=ApiError>>;

    /// Restart the node
    fn api_v1_restart_node_post(&self, context: &Context) -> Box<Future<Item=ApiV1RestartNodePostResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// Retrieves the dynamic information for this node.
    fn api_v1_node_info_get(&self, force_ntp_check: Option<bool>) -> Box<Future<Item=ApiV1NodeInfoGetResponse, Error=ApiError>>;

    /// Retrieves the static settings for this node.
    fn api_v1_node_settings_get(&self) -> Box<Future<Item=ApiV1NodeSettingsGetResponse, Error=ApiError>>;

    /// Version of the next update
    fn api_v1_next_update_get(&self) -> Box<Future<Item=ApiV1NextUpdateGetResponse, Error=ApiError>>;

    /// Restart the node
    fn api_v1_restart_node_post(&self) -> Box<Future<Item=ApiV1RestartNodePostResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: Context) -> ContextWrapper<'a, Self>;
}

impl<'a, T: Api + Sized> ContextWrapperExt<'a> for T {
    fn with_context(self: &'a T, context: Context) -> ContextWrapper<'a, T> {
         ContextWrapper::<T>::new(self, context)
    }
}

impl<'a, T: Api> ApiNoContext for ContextWrapper<'a, T> {

    /// Retrieves the dynamic information for this node.
    fn api_v1_node_info_get(&self, force_ntp_check: Option<bool>) -> Box<Future<Item=ApiV1NodeInfoGetResponse, Error=ApiError>> {
        self.api().api_v1_node_info_get(force_ntp_check, &self.context())
    }

    /// Retrieves the static settings for this node.
    fn api_v1_node_settings_get(&self) -> Box<Future<Item=ApiV1NodeSettingsGetResponse, Error=ApiError>> {
        self.api().api_v1_node_settings_get(&self.context())
    }

    /// Version of the next update
    fn api_v1_next_update_get(&self) -> Box<Future<Item=ApiV1NextUpdateGetResponse, Error=ApiError>> {
        self.api().api_v1_next_update_get(&self.context())
    }

    /// Restart the node
    fn api_v1_restart_node_post(&self) -> Box<Future<Item=ApiV1RestartNodePostResponse, Error=ApiError>> {
        self.api().api_v1_restart_node_post(&self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
