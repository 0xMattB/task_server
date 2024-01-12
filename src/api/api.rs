/*************************************************************************
    "task_server"
    api/api.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use actix_web::{
    HttpRequest,
    HttpResponse,
    web::{
        self,
        Data,
        Json,
    }
};
use base64::{
    Engine as _,
    engine::general_purpose
};
use crate::repository::database::Database;
use crate::models::{
    user::{
        User,
        UserPatch,
    },
    entry::{
        Entry,
        EntryWrite,
        EntryPatch,
        EntryParams,
        entry_validate,
        entry_from_entry_write,
        entry_from_entry_write_edit,
    },
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/entries", web::post().to(create_entry))
            .route("/entries", web::get().to(get_entries))
            .route("/entries/{id}", web::get().to(get_entry_by_id))
            .route("/entries/{id}", web::put().to(update_entry_by_id))
            .route("/entries/{id}", web::patch().to(update_entry_partial_by_id))
            .route("/entries/{id}", web::delete().to(delete_entry_by_id))

            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::patch().to(update_user_partial_by_id))
            .route("/users/{id}", web::delete().to(delete_user_by_id))
    );
}

/**************************************************************************/
/* ENTRY actions */
/**************************************************************************/
pub async fn create_entry(request: HttpRequest, db: Data<Database>, new_entry: Json<EntryWrite>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    let id = match get_user_id_from_username(&db, &username) {
        Ok(id) => { id },
        Err(()) => { return HttpResponse::NotFound().body("Matching username not found"); },
    };
    
    let new_entry = entry_from_entry_write(&new_entry, &username);

    match validate_entry(&new_entry) {
        Ok(()) => {
            match db.create_entry(new_entry, &id) {
                Ok(entry) => { return HttpResponse::Ok().json(entry); },
                Err(err) => { return HttpResponse::BadRequest().body(err.to_string()); },
            }
        },
        Err(s) => {
            return HttpResponse::BadRequest().body(s);
        },
    }
}

pub async fn get_entries(request: HttpRequest, db: web::Data<Database>, info: web::Query<EntryParams>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    let username = if username == "admin" { 
        None
    } else {
        Some(username.clone())
    };
    
    let f = EntryParams {
        username: username,
        year: info.year.to_owned(),
        month: info.month.to_owned(),
        day: info.day.to_owned(),
        reminder: info.reminder.to_owned(),
        expired: info.expired.to_owned(),
    };
    
    let entries = db.get_entries_by_filter(&f);
    HttpResponse::Ok().json(entries)
}

pub async fn get_entry_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    let entry = db.get_entry_by_id(&id);
    
    match entry {
        Some(entry) => {
            if username == "admin" || username == entry.username {
                HttpResponse::Ok().json(entry)
            } else {
                HttpResponse::Unauthorized().body("Invalid access")
            }
        }
       None => HttpResponse::NotFound().body("Entry not found"),
    }
}

pub async fn update_entry_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>, updated_entry: web::Json<EntryWrite>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    match db.get_entry_by_id(&id) {
        Some(entry) => {
            if username == "admin" || username == entry.username {
                let updated_entry = entry_from_entry_write_edit(&updated_entry, &entry);

                match entry_validate(&updated_entry) {
                    Ok(()) => {
                        let entry = db.update_entry_by_id(&id, updated_entry);
                        return HttpResponse::Ok().json(entry);
                    },
                    Err(s) => {
                        return HttpResponse::BadRequest().body(s);
                    },
                }
            } else {
                return HttpResponse::Unauthorized().body("Invalid access");
            }
        },
        None => {
            return HttpResponse::NotFound().body("Entry not found");
        },
    }
}

pub async fn update_entry_partial_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>, partial_entry: web::Json<EntryPatch>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    match db.get_entry_by_id(&id) {
        Some(mut entry) => {
            if username == "admin" || username == entry.username {
                if let Some(year) = &partial_entry.year {
                    entry.year = year.clone();
                }
                if let Some(month) = &partial_entry.month {
                    entry.month = month.clone();
                }
                if let Some(day) = &partial_entry.day {
                    entry.day = day.clone();
                }
                if let Some(task) = &partial_entry.task {
                    entry.task = task.clone();
                }
                if let Some(reminder) = &partial_entry.reminder {
                    entry.reminder = Some(reminder.clone());
                }
                
                match entry_validate(&entry) {
                    Ok(()) => {
                        let entry = db.update_entry_by_id(&id, entry.clone());
                        return HttpResponse::Ok().json(entry);
                    },
                    Err(s) => {
                        return HttpResponse::BadRequest().body(s);
                    },
                }
            } else {
                return HttpResponse::Unauthorized().body("Invalid access");
            }
        },
        None => {
            return HttpResponse::NotFound().body("Entry not found");
        },
    }
}

pub async fn delete_entry_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    if id.clone() == "all" {
        if username == "admin" {
            let entries = db.get_entries();
            
            for entry in entries {
                db.delete_entry_by_id(&entry.id);
            }
            
            return HttpResponse::Ok().body("Entry database cleared");
        } else {
            return HttpResponse::Unauthorized().body("Invalid access");
        }
    }
    
    match db.get_entry_by_id(&id) {
        Some(entry) => {
            if username == "admin" || username == entry.username {
                let entry = db.delete_entry_by_id(&id);
                return HttpResponse::Ok().json(entry);
            } else {
                return HttpResponse::Unauthorized().body("Invalid access");
            }
        },
        None => {
            return HttpResponse::NotFound().body("Entry not found");
        },
    }
}

fn validate_entry(entry: &Entry) -> Result<(), String> {
    let entry_check = Entry {
        id: entry.id.clone(),
        username: entry.username.clone(),
        year: entry.year.clone(),
        month: entry.month.clone(),
        day: entry.day.clone(),
        task: entry.task.clone(),
        reminder: entry.reminder.clone(),
        expired: entry.expired.clone(),
        created: entry.created,
        updated: entry.updated,
        user_id: entry.user_id.clone(),
    };
    
    match entry_validate(&entry_check) {
        Ok(()) => Ok(()),
        Err(e) => {
            Err(String::from(e))
        }
    }
}

/**************************************************************************/
/* USER actions */
/**************************************************************************/
pub async fn create_user(db: Data<Database>, new_user: Json<User>) -> HttpResponse {
    match validate_user(&db, &new_user) {
        Ok(()) => {
            match db.create_user(new_user.into_inner()) {
                Ok(user) => { return HttpResponse::Ok().json(user); },
                Err(err) => { return HttpResponse::BadRequest().body(err.to_string()); },
            }
        },
        Err(s) => {
            return HttpResponse::BadRequest().body(s);
        },
    }
}

pub async fn get_users(request: HttpRequest, db: web::Data<Database>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    if username != "admin" {
        return HttpResponse::Unauthorized().body("Invalid access");
    }
    
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

pub async fn get_user_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    if username != "admin" {
        return HttpResponse::Unauthorized().body("Invalid access");
    }

    let user = db.get_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn update_user_partial_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>, partial_user: web::Json<UserPatch>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    if username != "admin" {
        return HttpResponse::Unauthorized().body("Invalid access");
    }

    let user = db.get_user_by_id(&id);
    match user {
        Some(mut user) => {
            if let Some(password) = &partial_user.password {
                user.password = password.clone();
            }
            if let Some(utc_offset) = &partial_user.utc_offset {
                user.utc_offset = Some(utc_offset.clone());
            }

            match validate_user_partial(&user) {
                Ok(()) => {
                    match db.update_user_by_id(&id, user.clone()) {
                        Some(user) => { return HttpResponse::Ok().json(user); },
                        None => { return HttpResponse::NotFound().body("User not found"); },
                    }
                },
                Err(s) => {
                    return HttpResponse::BadRequest().body(s);
                },
            }
        }
        
        None => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn delete_user_by_id(request: HttpRequest, db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let username = match get_username_from_base64(&db, &request) {
        Some(name) => { name },
        None => { return HttpResponse::NotFound().body("Username not found"); },
    };
    
    if username != "admin" {
        return HttpResponse::Unauthorized().body("Invalid access");
    }

    let user = db.delete_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

/**************************************************************************/
/* supporting functions */
/**************************************************************************/
fn validate_user(db: &Database, test_user: &User) -> Result<(), String> {
    let users = db.get_users();
    
    for user in users {
        if user.username == test_user.username {
            return Err("Username in use".to_string());
        } else if user.email == test_user.email {
            return Err("Email in use".to_string());
        }
    }
    
    if validate_email(&test_user.email) == false {
        return Err("Invalid email address format".to_string());
    }
    
    if test_user.utc_offset.is_some() && validate_utc_offset(&test_user.utc_offset.clone().unwrap()) == false {
        return Err("Invalid utc-offset".to_string());
    }
    
    Ok(())
}

fn validate_user_partial(test_user: &User) -> Result<(), String> {
    if validate_email(&test_user.email) == false {
        return Err("Invalid email address format".to_string());
    }
    
    if test_user.utc_offset.is_some() && validate_utc_offset(&test_user.utc_offset.clone().unwrap()) == false {
        return Err("Invalid utc-offset".to_string());
    }
    
    Ok(())
}

fn get_username_from_base64(db: &Database, request: &HttpRequest) -> Option<String> {
    let users = db.get_users();
    
    if let Some(user64) = get_header_base64(request) {
        for user in users {
            if check_user(&user64, &user.username, &user.password) {
                return Some(String::from(&user.username));
            }
        }
    }
    
    None
}

fn get_header_base64<'a>(request: &'a HttpRequest) -> Option<String> {
    let response = request.headers().get("authorization")?.to_str();
    
    if let Ok(r) = response {
        let r = String::from(r);
        let args: Vec<_> = r.split_whitespace().collect();
        
        if args.len() == 2 && args[0].to_lowercase() == "basic" {
            return Some(String::from(args[1]));
        }        
    }
    
    None
}

fn check_user(b64: &str, user_name: &str, user_password: &str) -> bool {
    let user_b64 = get_b64(user_name, user_password);
    
    b64 == user_b64
}

fn get_b64(user_name: &str, user_password: &str) -> String {
    let combined = format!["{}:{}", user_name, user_password];
    
    String::from(
        general_purpose::STANDARD.encode(combined.as_bytes())
    )
}

fn validate_email(email: &str) -> bool {
    if let Some(c_at) = email.find('@') {
        if let Some(c_dot) = email.find('.') {
            if c_at < c_dot {
                return true;
            }
        }
    }
    
    false
}

fn validate_utc_offset(utc_offset_str: &str) -> bool {
    match utc_offset_str.parse::<i32>() {
        Ok(utc) => {
            if utc < -12 || utc > 14 {
                false
            } else {
                true
            }
        },
        Err(_) => {
            false
        },
    }
}

fn get_user_id_from_username(db: &Database, username: &str) -> Result<String, ()> {
    let users = db.get_users();
    
    for user in users {
        if user.username == username {
            return Ok(user.id);
        }
    }
    
    Err(())
}