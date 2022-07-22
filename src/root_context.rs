use std::{error::Error, rc::Rc};

use log::{error, info, trace};
use proxy_wasm::traits::{Context, HttpContext, RootContext};
use proxy_wasm::types::ContextType;

use crate::config::Config;
use crate::http_context::HeaderSet;

const NO_CONFIGURATION_ERROR: &str = "No configuration provided";

pub struct HeaderSetRoot {
    config: Rc<Config>,
}

impl HeaderSetRoot {
    pub fn new() -> Self {
        HeaderSetRoot {
            config: Default::default(),
        }
    }

    fn configure(&mut self, bytes: Option<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        let bytes = bytes.ok_or(NO_CONFIGURATION_ERROR)?;
        let json = String::from_utf8(bytes)?;
        self.config = Rc::new(Config::from_json(json.as_str())?);
        Ok(())
    }
}

impl Context for HeaderSetRoot {}

impl RootContext for HeaderSetRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Err(err) = self.configure(self.get_plugin_configuration()) {
            error!("Error: {}", err);
            info!("Default configuration will be used");
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        trace!("Header set filter created");
        Some(Box::new(HeaderSet::new(self.config.clone())))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}