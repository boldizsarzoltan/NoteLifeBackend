use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::repository::events::Event;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct EventDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date_time: NaiveDateTime,
 }

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NewEventDTO {
    pub title: String,
    pub description: Option<String>,
    pub date_time: NaiveDateTime,
}

impl From<Event> for EventDTO {
    fn from(event: Event) -> Self {
        EventDTO {
            id: event.id,
            title: event.title,
            description: event.description,
            date_time: event.date_time,
        }
    }
}