#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate Cardano Node API;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate tokio_core;

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use Cardano Node API::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      ApiV1NodeInfoGetResponse,
                      ApiV1NodeSettingsGetResponse,
                      ApiV1NextUpdateGetResponse,
                      ApiV1RestartNodePostResponse
                     };
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "ApiV1NodeInfoGet",
    "ApiV1NodeSettingsGet",
    "ApiV1NextUpdateGet",
    "ApiV1RestartNodePost",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("127.0.0.1")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        Cardano Node API::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        Cardano Node API::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    // Using a non-default `Context` is not required; this is just an example!
    let client = client.with_context(Cardano Node API::Context::new_with_span_id(self::uuid::Uuid::new_v4().to_string()));

    match matches.value_of("operation") {

        Some("ApiV1NodeInfoGet") => {
            let result = core.run(client.api_v1_node_info_get(Some(true)));
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        Some("ApiV1NodeSettingsGet") => {
            let result = core.run(client.api_v1_node_settings_get());
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        Some("ApiV1NextUpdateGet") => {
            let result = core.run(client.api_v1_next_update_get());
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        Some("ApiV1RestartNodePost") => {
            let result = core.run(client.api_v1_restart_node_post());
            println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
         },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

