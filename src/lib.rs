use std::str::from_utf8;

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> { Box::new(Rhis::from(context_id)) });
}

struct Rhis {
    pub context_id: u32,
}

impl From<u32> for Rhis {
    fn from(context_id: u32) -> Self { Self { context_id } }
}

impl Context for Rhis {}

impl HttpContext for Rhis {
    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if end_of_stream {
            if let Some(body) = self.get_http_request_body(0, body_size) {
                if let Ok(utf8_body) = from_utf8(body.as_slice()) {
                    info!("-> {}", utf8_body);
                }
            }
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        // self.set_http_response_header("Powered-By", Some("SBCP"));
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if end_of_stream {
            if let Some(body) = self.get_http_response_body(0, body_size) {
                if let Ok(utf8_body) = from_utf8(body.as_slice()) {
                    info!("<- {}", utf8_body);
                }
            }
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        self.log();
    }
}

impl Rhis {
    fn log(&mut self) {
        let req_headers = self.get_http_request_headers().iter()
            .map(|(k, v)| format!("-> {}::{}", k, v))
            .collect::<Vec<String>>()
            .join("\n");
        let resp_headers = self.get_http_response_headers().iter()
            .map(|(k, v)| format!("<- {}::{}", k, v))
            .collect::<Vec<String>>()
            .join("\n");
        // info!("#{}\n{}{}", self.context_id, req_headers, resp_headers);
    }
}