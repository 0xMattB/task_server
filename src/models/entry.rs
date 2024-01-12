/*************************************************************************
    "task_server"
    models/entry.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use chrono::NaiveDateTime;
use serde::{
    Deserialize,
    Serialize
};
use diesel::{
    Queryable,
    Insertable,
    AsChangeset
};
use crate::date::date::Date;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::models::schema::entries)]
pub struct Entry {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub year: String,
    pub month: String,
    pub day: String,
    pub task: String,
    pub reminder: Option<String>,
    pub expired: String,
    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EntryWrite {
    pub year: String,
    pub month: String,
    pub day: String,
    pub task: String,
    pub reminder: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EntryPatch {
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub task: Option<String>,
    pub reminder: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EntryParams {
    pub username: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub reminder: Option<String>,
    pub expired: Option<String>,
}

pub fn entry_validate(entry: &Entry) -> Result<(), String> {
    /* validate date */
    let year = convert_str_to_t::<i32>(&entry.year, "Invalid year")?;
    let month = convert_str_to_t::<u32>(&entry.month, "Invalid month")?;
    let day = convert_str_to_t::<u32>(&entry.day, "Invalid day")?;
    
    let test_date = match Date::new(year, month, day) {
        Ok(td) => td,
        Err(_) => { return Err("Invalid date".to_string()); },
    };
    
    if Date::is_today_or_later(&test_date) == false {
        return Err("Date has passed".to_string());
    }
    
    /* validate task */
    if entry.task.len() < 1 {
        return Err("No task present".to_string());
    }
    
    /* validate reminder */
    if let Some(reminder) = &entry.reminder
    {
        let reminder = match reminder.parse::<i64>() {
            Ok(r) => r,
            Err(_) => { return Err("Invalid reminder".to_string()); },
        };
        
        let today = Date::today();
        let diff = Date::difference(&test_date, &today);
        
        if reminder > 0 && reminder > diff {
            return Err("Reminder too great for due date".to_string());
        }
    }
    
    Ok(())
}

pub fn entry_from_entry_write(src: &EntryWrite, username: &str) -> Entry {
    Entry {
        id: String::new(),
        username: String::from(username),
        year: src.year.clone(),
        month: src.month.clone(),
        day: src.day.clone(),
        task: src.task.clone(),
        reminder: src.reminder.clone(),
        expired: false.to_string(),
        created: NaiveDateTime::MIN,
        updated: NaiveDateTime::MIN,
        user_id: String::new(),
    }
}

pub fn entry_from_entry_write_edit(write: &EntryWrite, orig: &Entry) -> Entry {
    Entry {
        id: orig.id.clone(),
        username: orig.username.clone(),
        year: write.year.clone(),
        month: write.month.clone(),
        day: write.day.clone(),
        task: write.task.clone(),
        reminder: write.reminder.clone(),
        expired: orig.expired.clone(),
        created: orig.created.clone(),
        updated: orig.updated.clone(),
        user_id: orig.user_id.clone(),
    }
}

fn convert_str_to_t<'a, T: std::str::FromStr>(s: &'a str, err: &'a str) -> Result<T, &'a str> {
    match s.parse::<T>() {
        Ok(u) => Ok(u),
        Err(_) => Err(err),
    }
}