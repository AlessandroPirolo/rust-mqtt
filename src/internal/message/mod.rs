mod timestamp;

pub mod message { 
    use serde::ser::{Serialize, Serializer, SerializeStruct};
    use serde::de;
    use timestamp;

    struct Message {
        time: timestamp::Timestamp,
        sender: String,
        contents: String,
    }

    impl Serialize for Message {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Message", 3)?;
            state.serialize_field("sender", &self.sender)?;
            state.serialize_field("contents", &self.sender)?;
            state.serialize_field("time", &self.time.to_rfc2822())?;
            state.end()
        }
    }

    impl<'de> de::Deserialize<'de> for Message {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> 
        {
            enum Field { Time, Sender, Contents }

            impl<'de> de::Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> de::Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("`time`, `sender` or `contents`")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "time" => Ok(Field::Time),
                                "sender" => Ok(Field::Sender),
                                "contents" => Ok(Field::Contents),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }

                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }
            struct MessageVisitor;

            impl<'de> de::Visitor<'de> for MessageVisitor {
                type Value = Message;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("struct Message")
                }

                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>, 
                {
                    let mut time: Option<DateTime<Utc>> = None;
                    let mut sender: Option<String> = None;
                    let mut contents: Option<String> = None;

                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Time => {
                                if time.is_some() {
                                    return Err(de::Error::duplicate_field("time"))
                                }
                                time = Option::expect(DateTime::parse_from_rfc2822(map.next_key()?), "error in retrieving time value");
                            }

                            Field::Sender => {
                                if sender.is_some() {
                                    return Err(de::Error::duplicate_field("sender"))
                                }
                                sender = Some(map.next_key()?);
                            }
                            
                            Field::Contents => {
                                if contents.is_some() {
                                    return Err(de::Error::duplicate_field("contents"))
                                }
                                contents = Some(map.next_key()?);
                            }
                        }
                    }
                    let time = time.ok_or_else(|| de::Error::missing_field("time"))?;
                    let sender = sender.ok_or_else(|| de::Error::missing_field("sender"))?;
                    let contents = contents.ok_or_else(|| de::Error::missing_field("contents"))?;
                    Ok(Message{sender, contents, time})

                }
            }
            const FIELDS: &[&str] = &["time", "sender", "contents"];
            deserializer.deserialize_struct("Message", FIELDS, MessageVisitor)
        }
    }

    impl Message {
        pub fn new(_sender: String, _contents: String) -> Self {
            Self {
                time: Utc::now(),
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
