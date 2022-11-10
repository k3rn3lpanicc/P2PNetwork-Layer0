use colored::Colorize;


#[derive(PartialEq,Eq)]
pub enum LOGTYPE {
    MORE_INFO,
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub fn log(message : &str , log_type : LOGTYPE){
    //if log_type==LOGTYPE::MORE_INFO{
    //    return;
    //}

    let time = chrono::Local::now();
    //%Y-%m-%d %H:%M:%S
    let time = time.format("%H:%M:%S");
    let time = time.to_string();
    let log_type = match log_type {
        LOGTYPE::MORE_INFO => "MINFO".bright_green().underline(),
        LOGTYPE::INFO => "INFO ".to_string().bright_yellow().bold(),
        LOGTYPE::WARN => "WARN".to_string().yellow().bold().underline(),
        LOGTYPE::ERROR => "ERROR".to_string().red().bold().underline(),
        LOGTYPE::DEBUG => "DEBUG".to_string().bright_cyan().bold().underline().italic(),
    };
    println!("->>  ({}) [{}]  {}",time.black().underline().on_green(),log_type,message.bright_white());
}

pub trait Logger {
    fn log(&self , log_type : LOGTYPE);
}
impl Logger for &str {
    fn log(&self , log_type : LOGTYPE){
        log(self , log_type);
    }
}
impl Logger for String {
    fn log(&self , log_type : LOGTYPE){
        log(self.as_str() , log_type);
    }
}
