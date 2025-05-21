pub mod publish {
    use crate::packet::packet::Packet;   
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct Publish {
        qos: String, 
        payload: Option<Vec<u8>>
    }

    impl Publish {
        pub fn new(qos: String, payload: Option<Vec<u8>>) -> Self {
            Self { qos, payload }
        }
    }

    impl Packet for Publish {
        fn handle(&self) -> Option<impl Packet> {
            None::<Publish>
        }

        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Puback;

    impl Puback {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Puback {
        fn handle(&self) -> Option<impl Packet> {
            None::<Puback>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Pubrec;

    impl Pubrec {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Pubrec {
        fn handle(&self) -> Option<impl Packet> {
            None::<Pubrec>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Pubrel;

    impl Pubrel {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Pubrel {
        fn handle(&self) -> Option<impl Packet> {
            None::<Pubrel>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Pubcomp;

    impl Pubcomp {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Pubcomp {
        fn handle(&self) -> Option<impl Packet> {
            None::<Pubcomp>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
} 
