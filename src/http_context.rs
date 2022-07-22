use std::rc::Rc;

use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;

use crate::config::Config;

pub struct HeaderSet {
    config: Rc<Config>,
}

impl HeaderSet {
    pub fn new(config: Rc<Config>) -> Self {
        HeaderSet { config }
    }
}

impl Context for HeaderSet {}

impl HttpContext for HeaderSet {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        if let Some(header_name) = self.config.header_name.clone() {
            self.set_http_request_header(
                header_name.as_str(),
                Some(self.config.header_value.clone().unwrap().as_str()),
            );
        }
        let header_list_opt = self.config.headers.clone();
        if let Some(header_list) = header_list_opt {
            header_list.into_iter().for_each(|header_config| {
                self.set_http_request_header(
                    header_config.header_name.as_str(),
                    Some(header_config.header_value.as_str()),
                );
            });
        }
        Action::Continue
    }
}