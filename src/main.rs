extern crate colored;
extern crate structopt;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Bat Noises!")]
    /// the noises of a regular bat
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// the bat is only pretending to be dead, and is unharmed
    dead: bool,
    #[structopt(short = "u", long = "uwu")]
    /// cringe
    uwu: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// read a custom Bat file from a path
    batfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    let mut eye = if options.dead { "x" } else { "o" };

    if options.uwu {
        eye = "u";
    }

    if message.to_lowercase() == "woof" {
        eprintln!("Bats don't bark!");
    }

    if message.to_lowercase() == "meow" {
        eprintln!("Bats don't meow!");
    }

    println!(
        "                          {}",
        message.bright_yellow().underline().on_purple()
    );
    println!("                              /");
    println!("                             /");

    match &options.batfile {
        Some(path) => {
            let bat_template =
                std::fs::read_to_string(path).expect(&format!("Could not read file {:?}", path));

            let bat_picture = bat_template.replace("{eye}", eye);

            println!("{}", &bat_picture);
        }
        None => {
            println!("_________________               _________________");
            println!(" ~-.              \\  |\\___/|  /              .-~");
            println!(
                "     ~-.           \\ / {eye} {eye} \\ /           .-~",
                eye = eye.magenta().bold()
            );
            println!("        >           \\   W  //           <");
            println!("       /             /~---~\\             \\");
            println!("      /_            |       |            _\\");
            println!("         ~-.        |       |        .-~");
            println!("            ;        \\     /        i");
            println!("           /___      /\\   /\\      ___\\");
            println!("                ~-. /  \\_/  \\ .-~");
            println!("                   V         V")
        }
    }
}
