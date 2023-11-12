use std::{env, path};

use crate::base::log_level::LogLevel;
use chrono::Local;

pub struct Logger {
    pub log_file_path: String,
    pub write_file_enabled: bool,
}

impl Logger {
    pub fn new() -> Logger {
        let mut p = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        let s = format!("./log/{}.log", chrono::Local::now().format("%Y%m%d"));
        p.push(path::Path::new(&s));
        let path = p.as_path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::File::create(path).unwrap();
        }

        Logger {
            log_file_path: path.to_str().unwrap().to_string(),
            write_file_enabled: true,
        }
    }

    fn log(&self, level: LogLevel, message: String) {
        let s = format!(
            "[{}] {} {}\n",
            level,
            Local::now().format("%Y/%m/%d %H:%M"),
            message
        );
        print!("{}", s);
        if self.write_file_enabled {
            use std::fs::OpenOptions;
            use std::io::Write;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_file_path)
                .unwrap();
            file.write_all(s.as_bytes()).unwrap();
        }
    }

    pub fn debug(&self, message: String) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: String) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: String) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: String) {
        self.log(LogLevel::Error, message);
    }

    pub fn fatal(&self, message: String) {
        self.log(LogLevel::Fatal, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::new();
        print!("{}\n", logger.log_file_path);
        logger.log(LogLevel::Debug, "test".to_string());
        logger.debug("test".to_string());
        logger.info("test".to_string());
        logger.warning("test".to_string());
        logger.error("test".to_string());
        logger.fatal("test".to_string());
    }
}
