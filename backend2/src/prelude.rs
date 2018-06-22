pub use uuid::Uuid;

use chrono;
pub use chrono::NaiveDateTime;
pub use failure::Error;
pub type DateTime = chrono::DateTime<chrono::Utc>;

pub use app::App;
pub(super) use db;

pub fn now() -> DateTime {
    chrono::Utc::now()
}

pub fn utc_from_naive(d: NaiveDateTime) -> DateTime {
    DateTime::from_utc(d, chrono::Utc)
}

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}
