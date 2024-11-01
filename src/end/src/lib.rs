use std::{fs::File, io::Write};

pub fn write_to(path: &str, content: String) -> () {
    let file = File::create_new(path);
    if let Err(_) = file {
        panic!("The destination file already exists! Consider changing the destination!");
    }
    else {
        let mut file = file.unwrap();
        file.write_all(&content[..].as_bytes()).unwrap();
    }
}

pub static TXT_8: &str = "\
    Kudos Q!\n\
    Finally taught him a lesson ey?\n\
    Cool work bro\n\
    Nice meeting you\n\
    Hope to see you soon....\n\
    Or maybe not eh? lol!\n\
    See ya!!";