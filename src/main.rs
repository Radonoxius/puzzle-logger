use colored::Colorize;
use end::TXT_8;
use logger::cmd::looper;

#[tokio::main]
async fn main() {
    let finished = looper().await;
    if finished {
        println!("{}", "Mission Success! Culprit has been taught a lesson!".green().bold());
        end::write_to("8.txt", String::from(TXT_8));
    }
    else {
        println!("{}", "Mission Failed! Try harder!".red().bold());
    }
}
