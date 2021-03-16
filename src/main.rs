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
