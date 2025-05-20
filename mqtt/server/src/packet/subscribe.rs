pub mod subscribe {
    use crate::packet::packet::Packet;
    use serde::{Serialize, Deserialize};

    // Subscription
    #[derive(Serialize, Deserialize)]
    struct Subscribe {
        payload: Vec<u8>
    }

    impl Subscribe {
        pub fn new(payload: Vec<u8>) -> Self {
            Self { payload }
        }
    }

    impl Packet for Subscribe {
        fn handle(&self) -> () {
            // add to the vec of sub of the 
            // corresponding entry the new topic
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Suback;
    
    impl Suback {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Suback {
        fn handle(&self) -> () {
            //nothing
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }

    // Unsubscribe 
    #[derive(Serialize, Deserialize)]
    struct Unsubscribe {
        payload: Vec<u8>
    }

    impl Unsubscribe {
        pub fn new(payload: Vec<u8>) -> Self {
            Self { payload }
        }
    }

    impl Packet for Unsubscribe {
        fn handle(&self) -> () {
            // remove the topic from the corresponding
            // entry's vector of sub
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Unsuback;
    
    impl Unsuback {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Unsuback {
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
