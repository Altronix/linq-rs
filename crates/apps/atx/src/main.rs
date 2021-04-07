#[macro_use]
extern crate clap;
extern crate futures;
extern crate linq;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

mod logger;
mod process_cmd;
mod process_ipconfig;
mod process_update;

/// logging
use log::error;

/// Cli
use clap::{load_yaml, App};

/// App
use process_cmd::*;
use process_ipconfig::*;
use process_update::*;

fn main() {
    // parse cli
    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    // Init logger
    let logger = logger::init(m.is_present("verbose"));
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();

    let result = if let Some(cmd) = m.subcommand_matches("cmd") {
        process_cmd(cmd)
    } else if let Some(cmd) = m.subcommand_matches("ipconfig") {
        process_ipconfig(cmd)
    } else if let Some(cmd) = m.subcommand_matches("update") {
        process_update(cmd)
    } else {
        Err(linq::error::LinqError::Unknown)
    };
    match result {
        Ok(result) => println!("{}", result),
        Err(e) => error!("{}", e),
    }
}
