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
        fn handle(&self) -> () {
           // reset some timer associated with the client 
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
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
        fn handle(&self) -> () {
            // nothing
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }
}
