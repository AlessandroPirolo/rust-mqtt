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
        fn handle(&self) -> () {
            
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
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
        fn handle(&self) -> () {
            
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
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
        fn handle(&self) -> () {
            
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
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
        fn handle(&self) -> () {
            
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
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
        fn handle(&self) -> () {
            
        }
        fn to_byte(&self) -> Vec<u8> {
            Vec::new()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            Self {}
        }
    }
} 
