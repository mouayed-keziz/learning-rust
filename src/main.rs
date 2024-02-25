use ferris_says::say;
use std::io::{stdout, BufWriter};

fn ferris_say(message: &str) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn main() {
    ferris_say("Hello world, this is Ferris speaking!")
}
