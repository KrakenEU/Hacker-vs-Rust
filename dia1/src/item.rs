use std::fmt::{write, Display};
use core::fmt;
use core::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub enum Status {
    NotStarted,
    Started,
    Completed
}
#[derive(Debug, PartialEq)]
pub enum Priority {
    low,
    normal,
    urgent
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::NotStarted => write!(f, "not started"),
            Status::Started => write!(f, "started"),
            Status::Completed => write!(f, "completed"),
        }
    }
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::low => write!(f, "low"),
            Priority::normal => write!(f, "normal"),
            Priority::urgent => write!(f, "urgent"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Added {
    NotNew,
    New
}
impl Added {
    pub fn from_str(s: &str) -> Option<Added> {
        match s {
            "1" => Some(Added::New),
            _ => Some(Added::NotNew),
        }
    }
}
pub struct Item {
    pub title: String,
    pub body: String,
    pub status: Status,
    pub priority: Priority,
    pub is_new: Added
}

impl Item {
    pub fn new(title:String, body: String, status: Status, priority: Priority, new: Added) -> Self {
        Self { title: title, body: body, status: status, priority: priority, is_new: new }
    }
    pub fn display_item(&self) {
        println!("Item name: {}\nDescription: {}\nStatus: {}\nPriority: {}", self.title, self.body, self.status, self.priority);
    }
}

impl Status {
    pub fn from_str(s: &str) -> Option<Status> {
        match s {
            "not started" => Some(Status::NotStarted),
            "started" => Some(Status::Started),
            "completed" => Some(Status::Completed),
            _ => None
        }
    }
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Priority> {
        match s {
            "low" => Some(Priority::low),
            "normal" => Some(Priority::normal),
            "urgent" => Some(Priority::urgent),
            _ => None
        }
    }
}