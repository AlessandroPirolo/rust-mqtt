mod publish;
mod ping;
mod connect;
mod subscribe;

pub mod packet {
    pub trait Packet {
        fn handle(&self) -> ();
        fn to_byte(&self) -> Vec<u8>;
        fn parse_form(bytes: Vec<u8>) -> Self;
    }
}
