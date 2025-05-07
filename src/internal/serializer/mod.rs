mod message;

pub mod serializer {
    use crate::internal::serializer::message::message::Message;

    pub fn deserialize(payload: Vec<u8>) -> String {
        Message::stringify(payload)
    }

    pub fn serialize(who: String, contents: String) -> Vec<u8> {
        Message::new(who, contents).to_byte()
    }

}
