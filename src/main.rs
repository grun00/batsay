extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Bat Noises!")]
    /// the noises of a regular bat
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// the bat is only pretending to be dead, and is unharmed
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("Bats don't bark!");
    }

    if message.to_lowercase() == "meow" {
        eprintln!("Bats don't meow!");
    }

    println!("                          {}", message);
    println!("                              /");
    println!("_________________               _________________");
    println!(" ~-.              \\  |\\___/|  /              .-~");
    println!(
        "     ~-.           \\ / {eye} {eye} \\ /           .-~",
        eye = eye
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
