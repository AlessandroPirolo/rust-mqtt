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
        fn handle(&self) -> Option<impl Packet> {
            // add to the vec of sub of the 
            // corresponding entry the new topic
            Some(Suback::new())
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
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
        fn handle(&self) -> Option<impl Packet> {
            None::<Suback>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
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
        fn handle(&self) -> Option<impl Packet> {
            // remove the topic from the corresponding
            // entry's vector of sub
            Some(Unsuback::new())
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
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
        fn handle(&self) -> Option<impl Packet> {
            None::<Unsuback>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
