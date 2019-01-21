/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for ApiV1NodeInfoGet
    lazy_static! {
        pub static ref API_V1_NODE_INFO_GET_: Mime = "application/json;charset=utf-8".parse().unwrap();
    }
    /// Create Mime objects for the response content types for ApiV1NodeSettingsGet
    lazy_static! {
        pub static ref API_V1_NODE_SETTINGS_GET_: Mime = "application/json;charset=utf-8".parse().unwrap();
    }
    /// Create Mime objects for the response content types for ApiV1NextUpdateGet
    lazy_static! {
        pub static ref API_V1_NEXT_UPDATE_GET_: Mime = "application/json;charset=utf-8".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
