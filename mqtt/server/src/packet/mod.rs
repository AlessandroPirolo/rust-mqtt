mod publish;
mod ping;
mod connect;
mod subscribe;

pub mod packet {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub enum PacketType {
        CONNECT,
        CONACK,
        PUBLISH,
        PUBACK,
        PUBREC,
        PUBREL,
        PUBCOMP,
        SUBSCRIBE,
        SUBACK,
        UNSUBSCRIBE,
        UNSUBACK,
        PINGREQ,
        PINGRESP,
        DISCONNECT
    }

    pub trait Packet {
        fn handle(&self) -> Option<impl Packet>;
        fn to_byte(&self) -> Vec<u8>;
        fn parse_form(bytes: Vec<u8>) -> Self;
    }

    #[derive(Serialize, Deserialize)]
    pub struct PacketImpl {
        packet_type: PacketType,
        payload: Vec<u8>
    }

    impl PacketImpl {
        pub fn new(packet_type: PacketType, payload: Vec<u8>) -> Self {
            Self { packet_type, payload }
        }
    }

    impl Packet for PacketImpl {
        fn handle(&self) -> Option<impl Packet> {
            None::<PacketImpl>
        }

        fn to_byte(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }

        fn parse_form(bytes: Vec<u8>) -> Self {
            serde_json::from_slice(&bytes).unwrap()
        }
    }
}
