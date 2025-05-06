mod timestamp;

pub mod message { 
    use crate::internal::message::timestamp::timestamp;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        time: timestamp::Timestamp,
        sender: String,
        contents: String,
    }

    impl Message {
        pub fn new(_sender: String, _contents: String) -> Self {
            Self {
                time: timestamp::Timestamp::now(),
                sender: _sender,
                contents: _contents,
            }
        }
    
        pub fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        pub fn parse_from(bytes: Vec<u8>) -> Message {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
