use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str;

const PEER_LOCALITY_EXCHANGE_HEADER: &str = "x-envoy-peer-zone";

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(PeerLocalityRoot) });
}

#[repr(u32)]
#[non_exhaustive]
pub enum TrafficDirection {
    Unspecified,
    Inbound,
    Outbound,
}

struct PeerLocalityRoot;

impl Context for PeerLocalityRoot {}

impl RootContext for PeerLocalityRoot {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(PeerLocality {}))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct PeerLocality;

impl Context for PeerLocality {}

impl HttpContext for PeerLocality {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        match self.get_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER) {
            Some(v) => {
                self.set_property(vec!["downstream_peer_zone"], Some(v.as_bytes()));
                self.set_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER, None);
            }
            None => {}
        }

        match self.get_property(vec!["listener_direction"]) {
            Some(d) => {
                let s = &d[0].to_string()[..];

                // skip inbound proxy
                if !s.eq("1") {
                    match self.get_property(vec!["node", "locality", "zone"]) {
                        Some(v) => {
                            self.set_http_request_header(
                                PEER_LOCALITY_EXCHANGE_HEADER,
                                Some(str::from_utf8(&v).unwrap()),
                            );
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        // TODO
        Action::Continue
    }
}
