pub mod serializer {
    use crate::internal::message::message::Message;

    pub fn deserialize(payload: Vec<u8>) -> String {
        Message::stringify(payload)
    }

    pub fn serialize(who: String, contents: String) -> Vec<u8> {
        Message::new(who, contents).to_byte()
    }

}
