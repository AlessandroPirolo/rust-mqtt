pub mod connect {
    use crate::packet::packet::Packet;
    use serde::Deserialize;
    use serde::Serialize;

    // Connect
    #[derive(Serialize, Deserialize)]
    struct Connect {
        payload: String,
    }

    impl Connect {
        pub fn new(payload: String) -> Self {
            Self {payload}
        }
    }

    impl Packet for Connect {
        fn handle(&self) -> () {
           // add into some list, open a session 
           // with a vec of subscription empty, 
           // open a thread to handle it
        }

        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }

    // Conack 
    #[derive(Serialize, Deserialize)]
    struct Conack;

    impl Packet for Conack {
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

    #[derive(Serialize, Deserialize)]
    struct Disconnect;

    impl Packet for Disconnect {
        fn handle(&self) -> () {
            // remove from some list and all entry regarding this client
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }
}
