use log::error;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str;

const PEER_LOCALITY_EXCHANGE_HEADER: &str = "x-envoy-peer-zone";

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(PeerLocalityRoot) });
}}

struct PeerLocalityRoot;

impl Context for PeerLocalityRoot {}

impl RootContext for PeerLocalityRoot {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(PeerLocality))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct PeerLocality;

impl Context for PeerLocality {}

impl HttpContext for PeerLocality {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER) {
            Some(v) => {
                self.set_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER, None);
                self.set_property(vec!["downstream_peer_zone"], Some(v.as_bytes()));
            }
            None => match self.get_property(vec!["node", "locality", "zone"]) {
                Some(v) => {
                    self.set_http_request_header(
                        PEER_LOCALITY_EXCHANGE_HEADER,
                        Some(str::from_utf8(&v).unwrap()),
                    );
                }
                None => error!("enable to set locality attribute for downstream peer"),
            },
        }

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_response_header(PEER_LOCALITY_EXCHANGE_HEADER) {
            Some(v) => {
                self.set_http_response_header(PEER_LOCALITY_EXCHANGE_HEADER, None);
                self.set_property(vec!["upstream_peer_zone"], Some(v.as_bytes()));
            }
            None => match self.get_property(vec!["node", "locality", "zone"]) {
                Some(v) => {
                    self.set_http_response_header(
                        PEER_LOCALITY_EXCHANGE_HEADER,
                        Some(str::from_utf8(&v).unwrap()),
                    );
                }
                None => error!("enable to set locality attribute for upstream peer"),
            },
        }

        Action::Continue
    }
}
