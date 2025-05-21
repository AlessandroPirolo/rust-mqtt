pub mod connect {
    use std::net::IpAddr;

    use crate::packet::packet::Packet;
    use serde::Deserialize;
    use serde::Serialize;

    // Connect
    #[derive(Serialize, Deserialize)]
    struct Connect {
        ip: IpAddr,
    }

    impl Connect {
        pub fn new(ip: IpAddr) -> Self {
            Self {ip}
        }
    }

    impl Packet for Connect {
        fn handle(&self) -> Option<impl Packet> {
           // add into some list, open a session 
           // with a vec of subscription empty, 
           // open a thread to handle it
           Some(Conack::new())
        }

        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    // Conack 
    #[derive(Serialize, Deserialize)]
    struct Conack;

    impl Conack {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Packet for Conack {
        fn handle(&self) -> Option<impl Packet> {
            // nothing
            None::<Conack>
        }

        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Disconnect {
        ip: IpAddr
    }

    impl Disconnect {
        pub fn new(ip: IpAddr) -> Self {
            Self {ip}
        }
    }

    impl Packet for Disconnect {
        fn handle(&self) -> Option<impl Packet> {
            // remove from some list and all entry regarding this client
            None::<Disconnect>
        }
        
        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
