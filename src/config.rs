use std::{fs::File, io::Write};

use configparser::ini::Ini;
use backtrace::Backtrace;

use crate::log::Log;

static mut WRITE_LOG: bool = false;
static mut WORKING_CALLER: u8 = 0;

pub struct Config;

impl Config {
    pub fn get_all() -> (String,String,u8,String,bool) {
        let r = Config::load();
        let app = r.get("listen","app").unwrap();
        let http = r.get("listen","http").unwrap();
        let worker:u8 = r.getuint("other","worker").unwrap().unwrap() as u8;
        let proxy_server_addr = r.get("server","addr").unwrap();
        let write_log = r.getbool("other", "write_log").unwrap().unwrap();
        (app,http,worker,proxy_server_addr,write_log)
    }

    pub fn get_proxy_server_addr() -> String {
        let r = Config::load();
        r.get("server","addr").unwrap()
    }

    pub fn get_listen_addr() -> (String,String,bool) {
        let r = Config::load();
        let app = r.get("listen","app").unwrap();
        let http = r.get("listen","http").unwrap();
        let write_log = r.getbool("other", "write_log").unwrap().unwrap();
        (app,http,write_log)
    }

    pub fn set_panic_hook() {
        std::panic::set_hook(Box::new(|_| {
            let bt = Backtrace::new();
            let mut f = File::options().append(true).open(Log::panic_file()).unwrap();
            f.write(format!("{:?}",bt).as_bytes()).unwrap();
        }));
    }

    fn load() -> Ini {
        let mut config = Ini::new();
        config.load("theshy.ini".to_string()).unwrap();
        config
    }

}

impl Config {
    pub fn turn_on() {
        unsafe{ WRITE_LOG = true }
    }

    pub fn log_off() -> bool {
        unsafe{ if !WRITE_LOG { return true; } }
        false
    }

    pub fn working_caller_count() -> u8 {
        unsafe{ WORKING_CALLER }
    }

    pub fn set_working_caller_count(n:u8) {
        unsafe{ WORKING_CALLER = n; }
    }
}