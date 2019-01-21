//! Main library entry point for Cardano Node API implementation.

mod server;

mod errors {
    error_chain!{}
}

pub use self::errors::*;
use std::io;
use hyper;
use Cardano Node API;

pub struct NewService;

impl hyper::server::NewService for NewService {
    type Request = (hyper::Request, Cardano Node API::Context);
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Instance = Cardano Node API::server::Service<server::Server>;

    /// Instantiate a new server.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Cardano Node API::server::Service::new(server::Server))
    }
}
