mod error;
mod install;
mod start;
mod stop;

use std::process;

use install::install_and_check;
use start::start_and_check;
use stop::stop_and_check;

pub fn cs_install() {
    if let Err(err) = install_and_check() {
        eprintln!("{err:#?}");
        process::exit(1);
    }
}

pub fn cs_start() {
    if let Err(err) = start_and_check() {
        eprintln!("{err:#?}");
        process::exit(1);
    }
}

pub fn cs_stop() {
    if let Err(err) = stop_and_check() {
        eprintln!("{err:#?}");
        process::exit(1);
    }
}
