/*************************************************************************
    "task_server"
    timer/email.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use lettre::{
    Message,
    Transport,
    SmtpTransport,
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use crate::date::date::Date;

pub struct Email {
    cred_user: String,
    cred_pass: String,
    smtp: String,
}

impl Email {
    pub fn new(email: &str, password: &str, smtp: &str) -> Self {
        Self {
            cred_user: String::from(email),
            cred_pass: String::from(password),
            smtp: String::from(smtp),
        }
    }
    
    pub fn send(&self, username: &str, email: &str, task: &str, due_date: &Date, expired: bool) -> Result<(), ()> {
        let from_field = format!["Task-Server <{}>", self.cred_user];
        let to_field = format!["{username} <{email}>"];
        let body_field = if expired {
            format!["The following task has expired ({})\n\n{}", due_date.to_string(), task]
        } else {
            format!["The following task is due on: {}\n\n{}", due_date.to_string(), task]
        };
        let subject_field = if expired {
            String::from("Task Due Today!")
        } else {
            String::from("Task Reminder!")
        };
        
        let email = Message::builder()
            .from(from_field.parse().unwrap())
            .to(to_field.parse().unwrap())
            .subject(subject_field)
            .header(ContentType::TEXT_PLAIN)
            .body(body_field)
            .unwrap();

        let cred_user = self.cred_user.clone();
        let cred_pass = self.cred_pass.clone();
        let creds = Credentials::new(cred_user.to_owned(), cred_pass.to_owned());

        let mailer = SmtpTransport::relay(&self.smtp)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}