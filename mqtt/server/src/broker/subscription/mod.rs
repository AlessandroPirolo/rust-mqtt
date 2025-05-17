mod topic;

pub mod subscription {
    use crate::broker::subscription::topic::topic::Topic;
    use crate::broker::session::session::Session;

    pub struct Sub {
        topic: Topic,
        subs: Vec<Session>
    }

    impl Sub {
        pub fn new(topic: Topic, subs: Vec<Session>) -> Self {
            Self {
                topic,
                subs
            }
        }

        pub fn add(&mut self, new_sub: Session) {
            self.subs.push(new_sub);
        }

        pub fn contains(&self, sub: Session) -> bool {
            self.subs.contains(&sub)
        }
    }
}
