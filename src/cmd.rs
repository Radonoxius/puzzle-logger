use colored::Colorize;
use crate::{
    equals_download,
    equals_exit,
    equals_help,
    equals_invalid,
    equals_login,
    equals_run,
    equals_upload,
    file_exists
};
use crate::io::{animator, read_console};

pub(super) enum Commands {
    Help,
    
    Login {
        password: String
    },
    
    Upload {
        file_name: String
    },
    
    Download,
    
    Run {
        malware_password: String,
    },
    
    Exit,
    
    Invalid
}

enum LoginState {
    Success,
    
    Fail
}

pub async fn looper() -> bool {
    println!("{}", "Type help to know more.".blue());
    let mut parseable = Commands::Help ;
    let mut state = LoginState::Fail;
    let mut uploaded = false;
    let mut downloaded = false;
    let mut has_run = false;
    while !equals_exit(&parseable) && !(downloaded && has_run) {
        parseable = parse_commands();
        if equals_help(&parseable) { help() }
        if equals_login(&parseable) { state = login(&parseable, &state); }
        if equals_upload(&parseable) { uploaded = upload(&parseable, &state, &uploaded).await }
        if equals_download(&parseable) { downloaded = download(&state, &downloaded).await }
        if equals_run(&parseable) { has_run = run(&parseable, &state, &uploaded, &has_run, &downloaded).await }
        if equals_invalid(&parseable) { println!("{}", "Command is invalid. Try again.".red().bold()) }
    }
    println!("{}", "Program exited.".yellow().bold());
    downloaded && has_run
}
fn parse_commands() -> Commands {
    let input = read_console();
    let mut words = input.split_ascii_whitespace();
    let input_words: Vec<&str> = words.clone().collect();
    let command = words.next();
    if command == None || input_words.len() > 2 {
        Commands::Invalid
    } else {
        match command.unwrap() {
            "help" => help_parse(words.next()),
            "login" => login_parse(words.next()),
            "upload" => upload_parse(words.next()),
            "download" => download_parse(words.next()),
            "run" => run_parse(words.next()),
            "exit" => exit_parse(words.next()),
            _ => invalid_parse(words.next())
        }
    }
}

fn help() {
    println!("{}",
        "---Help section of the Logger program---\n\
        Use login <password> to login to host computer.\n\
        Use upload <file_name> to upload the malware.\n\
        Use download to download our data.\n\
        Use run <malware_password> to run the malware on host computer.\n\
        Use exit to close the program.\n\
        (Exclude the angle braces, and type whitespaces between words)\n\
        The commands will be taken until you exit the program.".bold()
    );
}

fn login(login: &Commands, state: &LoginState) -> LoginState {
    if let LoginState::Success = *state {
        println!("{}", "You are already logged in!".bold());
        LoginState::Success
    }
    else {
        if let Commands::Login { password: input } = login {
            if *input == String::from("b45ixx") {
                println!("{}", "Successfully logged in!".green());
                LoginState::Success
            }
            else {
                println!("{}", "Wrong password!".red());
                LoginState::Fail
            }
        } else {
            LoginState::Fail
        }
    }
}

async fn upload(upload: &Commands, state: &LoginState, uploaded: &bool) -> bool {
    if let LoginState::Success = *state {
        if *uploaded == false {
            if let Commands::Upload { file_name: name } = upload {
                if file_exists(&name) {
                    print!("File is uploading ");
                    animator(1).await;
                    println!("{}", "Upload finished!".green());
                    true
                }
                else {
                    println!("{}", "File path is incorrect or file doesnt exist.".red());
                    false
                }
            }
            else {
                false
            }
        }
        else {
            println!("{}", "File has been uploaded! No need to do it again!".bold());
            true
        }
    }
    else {
        println!("{}", "Process failed. Log in to the system.".red());
        false
    }
}

async fn download(state: &LoginState, downloaded: &bool) -> bool {
    if *downloaded { 
        println!("{}", "Data has been downloaded. Nothing to download.".bold());
        true
    }
    else {
        if let LoginState::Success = *state {
            print!("Data is downloading ");
            animator(4).await;
            println!("{}", "Download finished!".green());
            true
        }
        else {
            println!("{}", "Process failed. Log in to the system.".red());
            false
        }
    }
}

async fn run(run: &Commands, state: &LoginState, uploaded: &bool, has_run: &bool, downloaded: &bool) -> bool {
    if let LoginState::Success = *state {
        if *has_run {
            println!("{}", "Malware can run only once. You have executed it already.".bold());
            true
        }
        else {
            if *uploaded {
                if *downloaded {
                    if let Commands::Run { malware_password: password } = run {
                        if *password == String::from("h4ck3r") {
                            print!("Malware is running ");
                            animator(4).await;
                            println!("{}", "Run finished!".green());
                            true
                        }
                        else {
                            println!("{}", "Malware password is incorrect.".red());
                            false
                        }
                    }
                    else {
                        false
                    }
                }
                else { 
                    println!("{}", "Download data before executing malware!".red());
                    false
                }
            }
            else {
                println!("{}", "Malware hasn't been uploaded!".red());
                false
            }
        }
    }
    else {
        println!("{}", "Process failed. Log in to the system.".red());
        false
    }
}

fn help_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Help
    } else {
        Commands::Invalid
    }
}

fn download_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Download
    } else {
        Commands::Invalid
    }
}

fn exit_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Exit
    } else {
        Commands::Invalid
    }
}

fn invalid_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Invalid
    } else {
        Commands::Invalid
    }
}

fn login_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Invalid
    } else {
        Commands::Login { password: String::from(word.unwrap()) }
    }
}

fn upload_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Invalid
    } else {
        Commands::Upload { file_name: String::from(word.unwrap()) }
    }
}

fn run_parse(word: Option<&str>) -> Commands {
    if let None = word {
        Commands::Invalid
    } else {
        Commands::Run { malware_password: String::from(word.unwrap()) }
    }
}