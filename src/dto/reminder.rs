use crate::repository::reminder::{NewReminder, Reminder};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ReminderDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NewReminderDTO {
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

impl From<Reminder> for ReminderDTO {
    fn from(reminder: Reminder) -> Self {
        ReminderDTO {
            id: reminder.id, 
            title: reminder.title,
            description: reminder.description, 
            start_time: reminder.start_time, 
            end_time: reminder.end_time 
        }
    }
}

impl From<NewReminder> for NewReminderDTO {
    fn from(newreminder: NewReminder) -> Self {
        NewReminderDTO {
            title: newreminder.title,
            description: newreminder.description, 
            start_time: newreminder.start_time, 
            end_time: newreminder.end_time 
        }
    }
}

impl From<ReminderDTO> for Reminder{
    fn from(reminder: ReminderDTO) -> Self {
        Reminder {
            id: reminder.id, 
            title: reminder.title,
            description: reminder.description, 
            start_time: reminder.start_time, 
            end_time: reminder.end_time 
        }
    }
}

impl From<NewReminderDTO> for NewReminder{
    fn from(newreminder: NewReminderDTO) -> Self {
        NewReminder {
            title: newreminder.title,
            description: newreminder.description, 
            start_time: newreminder.start_time, 
            end_time: newreminder.end_time 
        }
    }
}