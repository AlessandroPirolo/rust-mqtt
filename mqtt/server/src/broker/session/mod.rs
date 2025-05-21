mod topic;

pub mod session {
    use std::net::{IpAddr, TcpStream};

    use crate::broker::session::topic::topic::Topic;

    #[derive(Debug)]
    pub struct Session{
        // info about connection
        subs: Vec<Topic>,
        socket_addr: IpAddr,
        conn_stream: TcpStream
    }

    impl Session {
        pub fn new(subs: Vec<Topic>, addr: IpAddr, conn: TcpStream) -> Self {
            Self {
                subs,
                socket_addr: addr,
                conn_stream: conn
            }
        }

        pub fn add(&mut self, new_sub: Topic) {
            self.subs.push(new_sub);
        }

        pub fn contains(&self, sub: Topic) -> bool {
            self.subs.contains(&sub)
        }
    }
}
