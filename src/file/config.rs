/*************************************************************************
    "task_server"
    file/config.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use std::{
    fs::{
        self,
        File
    },
    io::prelude::*,
};
use crate::file::options::Options;

pub fn config_load(filename: &str) -> Options {
    match fs::read_to_string(filename) {
        Ok(config_data) => {
            match Options::from_file_data(&config_data) {
                Ok(options) => {
                    return options;
                },
                Err(()) => {
                    eprintln!("Error parsing configuration file, using default configuration values");
                    return Options::default();
                },
            }
        },
        Err(e) => {
            eprintln!("{e} - Creating new default configuration file");
            let options = Options::default();
            config_write(filename, &options.to_string());
            return options;
        },
    }
}

fn config_write(filename: &str, data: &str) {
    match File::create(filename) {
        Ok(mut file) => {
            let _ = file.write_all(data.as_bytes());
        },
        Err(e) => {
            eprintln!("{e} - New configuration file not created");
        },
    }
}