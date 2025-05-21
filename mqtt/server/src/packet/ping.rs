pub mod ping {
    use crate::packet::packet::Packet;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct Pingreq;

    impl Pingreq {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Pingreq {
        fn handle(&self) -> Option<impl Packet> {
           // reset some timer associated with the client
           Some(Pingresp::new())
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Pingresp;

    impl Pingresp {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Pingresp {
        fn handle(&self) -> Option<impl Packet> {
            None::<Pingresp>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
