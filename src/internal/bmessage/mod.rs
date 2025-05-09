pub mod bmessage { 
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BMessage {
        sender: String,
        contents: String,
    }

    impl BMessage {
        pub fn new(sender: String, contents: String) -> Self {
            Self {sender, contents}
        }
    
        pub fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_from(bytes: Vec<u8>) -> BMessage {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
