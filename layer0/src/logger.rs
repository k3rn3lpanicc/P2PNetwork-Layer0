use colored::Colorize;

pub enum LOGTYPE {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub fn log(message : &str , log_type : LOGTYPE){
    let time = chrono::Local::now();
    let time = time.format("%Y-%m-%d %H:%M:%S");
    let time = time.to_string();
    let log_type = match log_type {
        LOGTYPE::INFO => format!("INFO").blue().bold(),
        LOGTYPE::WARN => format!("WARN").yellow().bold().underline(),
        LOGTYPE::ERROR => format!("ERROR").red().bold().underline(),
        LOGTYPE::DEBUG => format!("DEBUG").cyan().bold(),
    };
    println!("{} [{}] {}",time.bright_green().underline(),log_type,message.bright_white());
}

pub trait Logger {
    fn log(&self , log_type : LOGTYPE);
}
impl Logger for &str {
    fn log(&self , log_type : LOGTYPE){
        log(self , log_type);
    }
}
    
