use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = "Welcome to Guarding Frog!";
    let width = 14;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}