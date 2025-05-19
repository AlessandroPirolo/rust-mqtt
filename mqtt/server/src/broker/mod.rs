mod session;

pub mod broker {
    use std::error::Error;
    use std::net::TcpListener;
    use crate::broker::session::session::Session;
    use crate::packet::packet::Packet;

    pub struct Broker {
        subs: Vec<Session>,
    }

    impl Broker {
        fn start_broker(&self) {
            let _ = TcpListener::bind("127.0.0.1:1883");
        }

        fn accept_packet(&self) { 

        }


        pub fn run(&self) -> Result<(),()> {
            self.start_broker();

            self.accept_packet();

            Err(())
        }
    }
}
