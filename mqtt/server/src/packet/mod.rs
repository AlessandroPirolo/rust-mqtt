pub mod packet {
    pub trait Packet {
        fn handle(&self) -> (); 
    }
}
