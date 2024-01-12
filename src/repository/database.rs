/*************************************************************************
    "task_server"
    repository/database.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use std::fmt::Error;
use chrono::prelude::*;
use dotenv::dotenv;
use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager
    },
};
use crate::models::{
    user::User,
    entry::{
        Entry,
        EntryParams,
    },
    schema::{
        entries::{
            self,
            dsl::*
        },
        users::dsl::*,
    }
};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }
    
    /**************************************************************************/
    /* ENTRY actions*/
    /**************************************************************************/
    pub fn create_entry(&self, entry: Entry, user_id_str: &str) -> Result<Entry, Error> {
        let entry = Entry {
            id: uuid::Uuid::new_v4().to_string(),
            expired: "false".to_string(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            user_id: String::from(user_id_str),
            ..entry
        };
        diesel::insert_into(entries)
            .values(&entry)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new entry");
        Ok(entry)
    }

    pub fn get_entries(&self) -> Vec<Entry> {
        entries
            .load::<Entry>(&mut self.pool.get().unwrap())
            .expect("Error loading all entries")
    }
    
    pub fn get_entries_by_filter(&self, params: &EntryParams) -> Vec<Entry> {
        let mut query = entries::table.into_boxed();

        if let Some(n) = &params.username {
            query = query.filter(entries::username.eq(n));
        }
        if let Some(n) = &params.year {
            query = query.filter(entries::year.eq(n));
        }
        if let Some(n) = &params.month {
            query = query.filter(entries::month.eq(n));
        }
        if let Some(n) = &params.day {
            query = query.filter(entries::day.eq(n));
        }
        if let Some(n) = &params.reminder {
            query = query.filter(entries::reminder.eq(n));
        }
        if let Some(n) = &params.expired {
            query = query.filter(entries::expired.eq(n));
        }
    
        let list = query.load::<Entry>(&mut self.pool.get().unwrap());
        
        match list {
            Ok(e) => e,
            Err(_) => vec![],
        }
    }
    
    pub fn get_entry_by_id(&self, entry_id: &str) -> Option<Entry> {
        let entry = entries
            .find(entry_id)
            .get_result::<Entry>(&mut self.pool.get().unwrap());

        match entry {
            Ok(_) => Some(entry.unwrap()),
            Err(_) => None,
        }
    }
    
    pub fn update_entry_by_id(&self, entry_id: &str, mut entry: Entry) -> Option<Entry> {
        entry.updated = Utc::now().naive_utc();
        let entry = diesel::update(entries.find(entry_id))
            .set(&entry)
            .get_result::<Entry>(&mut self.pool.get().unwrap())
            .expect("Error updating entry by id");
        Some(entry)
    }
    
    pub fn delete_entry_by_id(&self, entry_id: &str) -> Option<usize> {
        let count = diesel::delete(entries.find(entry_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting entry by id");
        Some(count)
    }

    /**************************************************************************/
    /* USER actions*/
    /**************************************************************************/
    pub fn create_user(&self, user: User) -> Result<User, Error> {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            ..user
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new user");
        Ok(user)
    }

    pub fn get_users(&self) -> Vec<User> {
        users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users")
    }
    
    pub fn get_user_by_id(&self, user_id_str: &str) -> Option<User> {
        let user = users
            .find(user_id_str)
            .get_result::<User>(&mut self.pool.get().unwrap());

        match user {
            Ok(_) => Some(user.unwrap()),
            Err(_) => None,
        }
    }
    
    pub fn update_user_by_id(&self, user_id_str: &str, user: User) -> Option<User> {
        let user = diesel::update(users.find(user_id_str))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error updating user by id");
        Some(user)
    }

    pub fn delete_user_by_id(&self, user_id_str: &str) -> Option<usize> {
        let count = diesel::delete(users.find(user_id_str))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting user by id");
        Some(count)
    }
}