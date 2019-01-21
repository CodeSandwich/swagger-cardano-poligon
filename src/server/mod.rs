#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;

extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;


use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, Context, XSpanId};
use swagger::auth::Scopes;

use {Api,
     ApiV1NodeInfoGetResponse,
     ApiV1NodeSettingsGetResponse,
     ApiV1NextUpdateGetResponse,
     ApiV1RestartNodePostResponse
     };
#[allow(unused_imports)]
use models;

pub mod auth;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/api/v1/next-update$",
            r"^/api/v1/node-info$",
            r"^/api/v1/node-settings$",
            r"^/api/v1/restart-node$"
        ]).unwrap();
    }
    pub static ID_API_V1_NEXT_UPDATE: usize = 0;
    pub static ID_API_V1_NODE_INFO: usize = 1;
    pub static ID_API_V1_NODE_SETTINGS: usize = 2;
    pub static ID_API_V1_RESTART_NODE: usize = 3;
}

pub struct NewService<T> {
    api_impl: Arc<T>,
}

impl<T> NewService<T> where T: Api + Clone + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T> {
        NewService{api_impl: api_impl.into()}
    }
}

impl<T> hyper::server::NewService for NewService<T> where T: Api + Clone + 'static {
    type Request = (Request, Context);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T> {
    api_impl: Arc<T>,
}

impl<T> Service<T> where T: Api + Clone + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T> {
        Service{api_impl: api_impl.into()}
    }
}

impl<T> hyper::server::Service for Service<T> where T: Api + Clone + 'static {
    type Request = (Request, Context);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        match &method {

            // ApiV1NodeInfoGet - GET /api/v1/node-info
            &hyper::Method::Get if path.matched(paths::ID_API_V1_NODE_INFO) => {
                if context.x_span_id.is_none() {
                    context.x_span_id = Some(headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                }





                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_force_ntp_check = query_params.iter().filter(|e| e.0 == "force_ntp_check").map(|e| e.1.to_owned())

                    .nth(0);

                let param_force_ntp_check = param_force_ntp_check.and_then(|param_force_ntp_check| param_force_ntp_check.parse::<>().ok());



                Box::new(({
                        {{

                                Box::new(api_impl.api_v1_node_info_get(param_force_ntp_check, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        context.x_span_id.as_ref().map(|header| response.headers_mut().set(XSpanId(header.clone())));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiV1NodeInfoGetResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::API_V1_NODE_INFO_GET_.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                ApiV1NodeInfoGetResponse::Invalid_


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                })) as Box<Future<Item=Response, Error=Error>>


            },


            // ApiV1NodeSettingsGet - GET /api/v1/node-settings
            &hyper::Method::Get if path.matched(paths::ID_API_V1_NODE_SETTINGS) => {
                if context.x_span_id.is_none() {
                    context.x_span_id = Some(headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                }







                Box::new(({
                        {{

                                Box::new(api_impl.api_v1_node_settings_get(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        context.x_span_id.as_ref().map(|header| response.headers_mut().set(XSpanId(header.clone())));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiV1NodeSettingsGetResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::API_V1_NODE_SETTINGS_GET_.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                })) as Box<Future<Item=Response, Error=Error>>


            },


            // ApiV1NextUpdateGet - GET /api/v1/next-update
            &hyper::Method::Get if path.matched(paths::ID_API_V1_NEXT_UPDATE) => {
                if context.x_span_id.is_none() {
                    context.x_span_id = Some(headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                }







                Box::new(({
                        {{

                                Box::new(api_impl.api_v1_next_update_get(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        context.x_span_id.as_ref().map(|header| response.headers_mut().set(XSpanId(header.clone())));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiV1NextUpdateGetResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::API_V1_NEXT_UPDATE_GET_.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                })) as Box<Future<Item=Response, Error=Error>>


            },


            // ApiV1RestartNodePost - POST /api/v1/restart-node
            &hyper::Method::Post if path.matched(paths::ID_API_V1_RESTART_NODE) => {
                if context.x_span_id.is_none() {
                    context.x_span_id = Some(headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                }







                Box::new(({
                        {{

                                Box::new(api_impl.api_v1_restart_node_post(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        context.x_span_id.as_ref().map(|header| response.headers_mut().set(XSpanId(header.clone())));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiV1RestartNodePostResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(202).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                })) as Box<Future<Item=Response, Error=Error>>


            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

