pub mod timestamp {
    use chrono::{DateTime, Datelike, Timelike, Utc};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Date {
        day: u32,
        month: u32,
        year: i32,
    }

    impl Date {
        fn new(day: u32, month: u32, year: i32) -> Self {
            Self {day, month, year}
        }
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Hour {
        hour: u32,
        min: u32,
    }

    impl Hour {
        fn new(hour: u32, min: u32) -> Self {
            Self {hour, min}
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Timestamp {
        date: Date,
        hour: Hour
    }

    impl Timestamp {
        pub fn now() -> Self {
            let _now: DateTime<Utc> = Utc::now();
            Self {
                date: Date::new(_now.day(), _now.month(), _now.year()),
                hour: Hour::new(_now.hour(), _now.minute())
            }
        }
    }
}
