mod topic;

pub mod session {
    use crate::broker::session::topic::topic::Topic;

    #[derive(Debug, PartialEq)]
    pub struct Session{
        // info about connection
        subs: Vec<Topic>
    }

    impl Session {
        pub fn new(subs: Vec<Topic>) -> Self {
            Self {
                subs
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
