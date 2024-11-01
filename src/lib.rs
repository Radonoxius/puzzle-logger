use std::fs::File;
use crate::cmd::Commands;

pub mod cmd;
pub mod io;

pub(crate) fn file_exists(path: &str) -> bool {
    let exists = File::open(path);
    let name_check = path == "malware";
    if let Err(_) = exists {
        false
    } else {
        name_check
    }
}

pub(crate) fn equals_help(command: &Commands) -> bool {
    if let Commands::Help = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_login(command: &Commands) -> bool {
    if let Commands::Login { password: _ } = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_upload(command: &Commands) -> bool {
    if let Commands::Upload { file_name: _ } = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_download(command: &Commands) -> bool {
    if let Commands::Download = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_run(command: &Commands) -> bool {
    if let Commands::Run { malware_password: _ } = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_exit(command: &Commands) -> bool {
    if let Commands::Exit = command {
        true
    }
    else {
        false
    }
}

pub(crate) fn equals_invalid(command: &Commands) -> bool {
    if let Commands::Invalid = command {
        true
    }
    else {
        false
    }
}