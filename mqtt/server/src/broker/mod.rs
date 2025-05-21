mod session;

pub mod broker {
    use std::error::Error;
    use std::net::TcpListener;
    use std::process::ExitStatus;
    use crate::broker::session::session::Session;
    use crate::packet::packet::Packet;

    pub struct Broker {
        subs: Vec<Session>,
        listener: TcpListener,
    }

    impl Broker {
        fn start_broker(&mut self) {
            self.listener = TcpListener::bind("127.0.0.1:1883").unwrap();
        }

        fn accept_packet(&self) {
                
        }


        pub fn run(&mut self) -> Result<(),()> {
            self.start_broker();

            self.accept_packet();

            Err(())
        }
    }
}
