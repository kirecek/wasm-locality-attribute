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
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
enum TrafficDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}

impl TrafficDirection {
    fn from_utf8(input: &Vec<u8>) -> TrafficDirection {
        match &input[0].to_string()[..] {
            "1" => TrafficDirection::Inbound,
            "2" => TrafficDirection::Outbound,
            _ => TrafficDirection::Unspecified,
        }
    }
}

struct PeerLocalityRoot;

impl Context for PeerLocalityRoot {}

impl RootContext for PeerLocalityRoot {
    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(PeerLocality {
            locality_received: true,
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct PeerLocality {
    locality_received: bool,
}

impl Context for PeerLocality {}

impl HttpContext for PeerLocality {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        match self.get_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER) {
            Some(v) => {
                self.set_http_request_header(PEER_LOCALITY_EXCHANGE_HEADER, None);
                self.set_property(vec!["downstream_peer_zone"], Some(v.as_bytes()));
            }
            None => self.locality_received = false,
        }

        let direction =
            TrafficDirection::from_utf8(&self.get_property(vec!["listener_direction"]).unwrap());

        if direction != TrafficDirection::Inbound {
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

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        match self.get_http_response_header(PEER_LOCALITY_EXCHANGE_HEADER) {
            Some(v) => {
                self.set_http_response_header(PEER_LOCALITY_EXCHANGE_HEADER, None);
                self.set_property(vec!["upstream_peer_zone"], Some(v.as_bytes()));
            }
            None => {}
        }

        let direction =
            TrafficDirection::from_utf8(&self.get_property(vec!["listener_direction"]).unwrap());

        if direction != TrafficDirection::Outbound && self.locality_received {
            match self.get_property(vec!["node", "locality", "zone"]) {
                Some(v) => {
                    self.set_http_response_header(
                        PEER_LOCALITY_EXCHANGE_HEADER,
                        Some(str::from_utf8(&v).unwrap()),
                    );
                }
                None => {}
            }
        }
        Action::Continue
    }
}
