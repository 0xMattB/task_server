/*************************************************************************
    "task_server"
    timer/timer.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use std::{
    thread,
    time::Duration,
    sync::Arc,
    str::FromStr,
};
use chrono::Timelike;
use actix_web::web::Data;
use crate::date::date::Date;
use crate::timer::email::Email;
use crate::file::options::Options;
use crate::models::user::User;
use crate::repository::database::Database;

pub fn run(db: Data<Database>, options: Arc<Options>) {
    let email = Arc::new(
        Email::new(
            options.sender_email_address(),
            options.sender_email_password(),
            options.sender_email_smtp(),
        )
    );
    
    thread::spawn(move || {
        loop {
            unsafe {
                let db_arc = Arc::clone(&db);
                let options_arc = Arc::clone(&options);
                let email_arc = Arc::clone(&email);
                
                static mut LAST_HOUR: i32 = 0;
                let current_hour = get_current_hour();
                
                if current_hour != LAST_HOUR {
                    LAST_HOUR = current_hour;
                    
                    match check_entries(&db_arc, &options_arc, &email_arc, current_hour) {
                        Ok(()) => {},
                        Err((e, id)) => { eprintln!("Error: {} (id: {})", e, id) },
                    }
                }

                thread::sleep(Duration::from_secs(60));
            }
        }
    });
}

fn get_current_hour() -> i32 {
    let current_date = chrono::Utc::now();
    current_date.hour() as i32
    
    /* use the following line instead to check entries once every 24 minutes (for testing) */
    //(current_date.minute() % 24) as i32
}

fn check_entries(db: &Database, options: &Options, email: &Email, current_hour: i32) -> Result<(), (String, String)> {
    let current_date = Date::today();
    let entries = db.get_entries();
    
    for mut entry in entries {
        /* get user for current entry */
        let user = match db.get_user_by_id(&entry.user_id) {
            Some(user) => user,
            None => { return Err((String::from("Cannot find user for entry"), String::from(entry.id))) }
        };
        
        /* get UTC offset for current user */
        let utc_offset = match user.utc_offset {
            Some(ref offset) => {
                match offset.parse::<i32>() {
                    Ok(offset) => offset,
                    Err(_) => { return Err((String::from("Invalid UTC offset for user"), String::from(user.id))) }
                }
            },
            None => 0,
        };
        
        let adjusted_hour = current_hour + utc_offset;
        
        if adjusted_hour == 0 {  /* midnight */
            let due_date = match Date::from_str(&format!["{}/{}/{}", entry.month, entry.day, entry.year]) {
                Ok(date) => { date },
                Err(_) => { return Err((String::from("Cannot parse date for user entry"), String::from(entry.id))) }
            };

            let date_diff = Date::difference(&due_date, &current_date) + 1;

            /* check for reminder */
            if options.enable_reminder_emails() {
                match get_reminder(&entry.reminder) {
                    Some(reminder) => {
                        if date_diff == reminder {
                            send_email(&user, &entry.task, &due_date, false, email);
                        }
                    },
                    _ => {},
                }
            }

            /* check for expired */
            if entry.expired == "false" && date_diff <= 0 {
                if options.enable_expired_emails() {
                    send_email(&user, &entry.task, &due_date, true, email);
                }

                entry.expired = String::from("true");
                db.update_entry_by_id(&entry.id.clone(), entry);
            }
        }
    }
    
    Ok(())
}

fn send_email(user: &User, task: &str, date: &Date, is_expired: bool, email: &Email) {
    match email.send(&user.username, &user.email, task, date, is_expired) {
        Ok(()) => {},
        Err(()) => { eprintln!("error sending email"); },
    }
}

fn get_reminder(reminder: &Option<String>) -> Option<i64> {
    if let Some(reminder) = reminder {
        match reminder.parse::<i64>() {
            Ok(reminder) => {
                return Some(reminder);
            },
            Err(_) => {},
        }
    }
    
    None
}