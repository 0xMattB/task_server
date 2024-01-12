/*************************************************************************
    "task_server"
    file/options.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use std::fmt;

pub struct Options {
    sender_email_address: String,
    sender_email_password: String,
    sender_email_smtp: String,
    enable_reminder_emails: bool,
    enable_expired_emails: bool,
    server_ip: String,
    server_port: u16,
}

impl Options {
    const OCTET_MAX: u32 = 255;
    
    const PREFIX_SENDER_EMAIL_ADDRESS: &str = "sender_email_address";
    const PREFIX_SENDER_EMAIL_PASSWORD: &str = "sender_email_password";
    const PREFIX_SENDER_EMAIL_SMTP: &str = "sender_email_smtp";
    const PREFIX_ENABLE_REMINDER_EMAILS: &str = "enable_reminder_emails";
    const PREFIX_ENABLE_EXPIRED_EMAILS: &str = "enable_expired_emails";
    const PREFIX_SERVER_IP: &str = "server_ip";
    const PREFIX_SERVER_PORT: &str = "server_port";
    
    const DEFAULT_SENDER_EMAIL_ADDRESS: &str = "username@domain.com";
    const DEFAULT_SENDER_EMAIL_PASSWORD: &str = "password123";
    const DEFAULT_SENDER_EMAIL_SMTP: &str = "smtp.domain.com";
    const DEFAULT_ENABLE_REMINDER_EMAILS: bool = false;
    const DEFAULT_ENABLE_EXPIRED_EMAILS: bool = false;
    const DEFAULT_SERVER_IP: &str = "127.0.0.1";
    const DEFAULT_SERVER_PORT: u16 = 8085;
    
    pub fn from_file_data(file_data: &str) -> Result<Options, ()> {
        let lines: Vec<_> = file_data.trim().lines().collect();
        
        if lines.len() != 7 {
            return Err(());
        }
        
        let sender_email_address = Self::parse_string_argument(lines[0], Self::PREFIX_SENDER_EMAIL_ADDRESS)?;
        let sender_email_password = Self::parse_string_argument(lines[1], Self::PREFIX_SENDER_EMAIL_PASSWORD)?;
        let sender_email_smtp = Self::parse_string_argument(lines[2], Self::PREFIX_SENDER_EMAIL_SMTP)?;
        let enable_reminder_emails = Self::parse_bool_argument(lines[3], Self::PREFIX_ENABLE_REMINDER_EMAILS)?;
        let enable_expired_emails = Self::parse_bool_argument(lines[4], Self::PREFIX_ENABLE_EXPIRED_EMAILS)?;
        let server_ip = Self::parse_ip_argument(lines[5], Self::PREFIX_SERVER_IP)?;
        let server_port = Self::parse_u16_argument(lines[6], Self::PREFIX_SERVER_PORT)?;
        
        Ok(
            Options {
                sender_email_address: sender_email_address.to_owned(),
                sender_email_password: sender_email_password.to_owned(),
                sender_email_smtp: sender_email_smtp.to_owned(),
                enable_reminder_emails,
                enable_expired_emails,
                server_ip: server_ip.to_owned(),
                server_port: server_port,
            }
        )
    }
    
    pub fn sender_email_address(&self) -> &String {
        &self.sender_email_address
    }
    
    pub fn sender_email_password(&self) -> &String {
        &self.sender_email_password
    }
    
    pub fn sender_email_smtp(&self) -> &String {
        &self.sender_email_smtp
    }
    
    pub fn enable_reminder_emails(&self) -> bool {
        self.enable_reminder_emails
    }
    
    pub fn enable_expired_emails(&self) -> bool {
        self.enable_expired_emails
    }
    
    pub fn server_ip(&self) -> &String {
        &self.server_ip
    }
    
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    
    fn parse_string_argument(line: &str, prefix: &str) -> Result<String, ()> {
        let fields: Vec<_> = line.split('=').collect();
        
        if !Self::field_check_prelim(&fields, prefix, true) {
            return Err(());
        }
        
        Ok(String::from(fields[1]))
    }
    
    fn parse_bool_argument(line: &str, prefix: &str) -> Result<bool, ()> {
        let fields: Vec<_> = line.split('=').collect();
        
        if !Self::field_check_prelim(&fields, prefix, false) {
            return Err(());
        }
        
        match fields[1].to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(()),
        }
    }
    
    fn parse_ip_argument(line: &str, prefix: &str) -> Result<String, ()> {
        let fields: Vec<_> = line.split('=').collect();
        
        if !Self::field_check_prelim(&fields, prefix, true) {
            return Err(());
        }
        
        if !Self::validate_ip(fields[1]) {
            return Err(());
        }
        
        Ok(String::from(fields[1]))
    }
    
    fn parse_u16_argument(line: &str, prefix: &str) -> Result<u16, ()> {
        let fields: Vec<_> = line.split('=').collect();
        
        if !Self::field_check_prelim(&fields, prefix, false) {
            return Err(());
        }
        
        let port = fields[1].parse::<u16>();
        
        if port.is_err() {
            return Err(());
        }
        
        Ok(port.unwrap())
    }
    
    fn validate_ip(ip: &str) -> bool {
        let fields: Vec<_> = ip.split(".").collect();
        
        if fields.len() != 4 {
            return false;
        }
        
        if Self::validate_octet(fields[0], Self::OCTET_MAX) == false {
            return false;
        }
        if Self::validate_octet(fields[1], Self::OCTET_MAX) == false {
            return false;
        }
        if Self::validate_octet(fields[2], Self::OCTET_MAX) == false {
            return false;
        }
        if Self::validate_octet(fields[3], Self::OCTET_MAX) == false {
            return false;
        }

        true
    }
    
    fn validate_octet(value: &str, max: u32) -> bool {
        if let Ok(v) = value.parse::<u32>() {
            if v > max {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
    
    fn field_check_prelim(fields: &Vec<&str>, prefix: &str, is_string: bool) -> bool {
        if fields.len() != 2 {
            return false;
        }
            
        if fields[0] != prefix {
            return false;
        }
            
        if is_string && fields[1].len() == 0 {
            return false;
        }

        true
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}\n{}={}\n{}={}\n{}={}\n{}={}\n{}={}\n{}={}\n",
            Options::PREFIX_SENDER_EMAIL_ADDRESS, self.sender_email_address,
            Options::PREFIX_SENDER_EMAIL_PASSWORD, self.sender_email_password,
            Options::PREFIX_SENDER_EMAIL_SMTP, self.sender_email_smtp,
            Options::PREFIX_ENABLE_REMINDER_EMAILS, self.enable_reminder_emails,
            Options::PREFIX_ENABLE_EXPIRED_EMAILS, self.enable_expired_emails,
            Options::PREFIX_SERVER_IP, self.server_ip,
            Options::PREFIX_SERVER_PORT, self.server_port,
        )
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            sender_email_address: String::from(Self::DEFAULT_SENDER_EMAIL_ADDRESS),
            sender_email_password: String::from(Self::DEFAULT_SENDER_EMAIL_PASSWORD),
            sender_email_smtp: String::from(Self::DEFAULT_SENDER_EMAIL_SMTP),
            enable_reminder_emails: Self::DEFAULT_ENABLE_REMINDER_EMAILS,
            enable_expired_emails: Self::DEFAULT_ENABLE_EXPIRED_EMAILS,
            server_ip: String::from(Self::DEFAULT_SERVER_IP),
            server_port: Self::DEFAULT_SERVER_PORT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn valid() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(true),
            Err(()) => assert!(false),
        }
    }
    
    #[test]
    fn invalid_empty() {
        match Options::from_file_data("") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_address() {
        match Options::from_file_data("sender_email_address=\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_password() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_stmp() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_reminder() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_expired() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_ip() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_empty_port() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_reminder() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=nope\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_expired() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=nope\nserver_ip=127.0.0.1\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_ip() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.300\nserver_port=8085\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn invalid_port() {
        match Options::from_file_data("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                                       enable_reminder_emails=false\nenable_expired_emails=true\nserver_ip=127.0.0.1\nserver_port=aaa\n") {
            Ok(_) => assert!(false),
            Err(()) => assert!(true),
        }
    }
    
    #[test]
    fn default_as_string() {
        let d = Options::default();
        
        assert_eq!(
            d.to_string(),
            String::from("sender_email_address=username@domain.com\nsender_email_password=password123\nsender_email_smtp=smtp.domain.com\n\
                          enable_reminder_emails=false\nenable_expired_emails=false\nserver_ip=127.0.0.1\nserver_port=8085\n")
        );
    }
}