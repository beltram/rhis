#[macro_use]
extern crate serde_derive;

use std::str::from_utf8;

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use crate::config::RhisConfig;
use wasm_bindgen::__rt::core::fmt::{Display, Formatter, Error};

mod config;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|context_id| -> Box<dyn RootContext> { Box::new(Rhis::from(context_id)) });
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> { Box::new(Rhis::from(context_id)) });
}

struct Rhis {
    context_id: u32,
    request_body: Option<Bytes>,
    request_headers: Option<Vec<(String, String)>>,
    response_body: Option<Bytes>,
    response_headers: Option<Vec<(String, String)>>,
}

impl From<u32> for Rhis {
    fn from(context_id: u32) -> Self {
        Self {
            context_id,
            request_body: None,
            request_headers: None,
            response_body: None,
            response_headers: None,
        }
    }
}

impl Context for Rhis {}

impl RootContext for Rhis {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        info!("Rhis started");
        true
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        self.get_configuration().as_ref()
            .and_then(|it| from_utf8(it.as_ref()).ok())
            .and_then(|it| serde_json::from_str::<RhisConfig>(it).ok())
            .map(|it| info!("Config: {}", it.response_body));
        true
    }
}

impl HttpContext for Rhis {

    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        self.request_headers = Some(self.get_http_request_headers());
        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, _end_of_stream: bool) -> Action {
        self.request_body = self.get_http_request_body(0, body_size);
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.response_headers = Some(self.get_http_response_headers());
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, _end_of_stream: bool) -> Action {
        self.response_body = self.get_http_response_body(0, body_size);
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("{}", self);
    }
}

impl Display for Rhis {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let req_body = self.request_body.as_ref()
            .and_then(|it| from_utf8(it.as_slice()).ok())
            .map(|it| format!("-> {}\n", it))
            .unwrap_or_else(|| "".to_string());
        let resp_body = self.response_body.as_ref()
            .and_then(|it| from_utf8(it.as_slice()).ok())
            .map(|it| format!("<- {}\n", it))
            .unwrap_or_else(|| "".to_string());
        write!(f, "{}{}", req_body, resp_body)
    }
}
