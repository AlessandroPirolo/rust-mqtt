pub mod topic {

    pub struct Topic {
        topic: String
    }

    impl Topic {
        pub fn new(topic: String) -> Self {
            Self{ topic }
        }

        pub fn contains(&self, pattern: String) -> bool {
            self.topic.contains(&pattern)
        }
    }
}
