use clap::{App, Arg};

const ARG_COMMAND_MODE: &str = "mode";
const MODE_QUIZ: &str = "quiz";
const MODE_SEQUENCE: &str = "sequence";

const ARG_COUNT: &str = "count";
const ARG_LEVEL: &str = "level";
const DEFAULT_COUNT: &str = "5";
const DEFAULT_LEVEL: &str = "1";

fn main() {
    let args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name(ARG_COMMAND_MODE)
                .short("m")
                .long(ARG_COMMAND_MODE)
                .help("mode")
                .possible_values(&[MODE_QUIZ, MODE_SEQUENCE])
                .default_value(MODE_QUIZ)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_COUNT)
                .short("c")
                .long(ARG_COUNT)
                .help("Number of questions")
                .default_value(DEFAULT_COUNT)
                .validator(mist::is_valid_count),
        )
        .arg(
            Arg::with_name(ARG_LEVEL)
                .short("l")
                .long(ARG_LEVEL)
                .help("Level")
                .default_value(DEFAULT_LEVEL)
                .validator(mist::is_valid_level),
        )
        .get_matches();

    // number of questions
    let count = args.value_of(ARG_COUNT).unwrap();
    let count = count.parse().unwrap();
    // quiz level
    let level = args.value_of(ARG_LEVEL).unwrap();
    let level = level.parse().unwrap();

    match args.value_of(ARG_COMMAND_MODE).unwrap() {
        MODE_QUIZ => mist::ask_quiz(count, level),
        MODE_SEQUENCE => mist::ask_sequence(count, level),
        _ => println!("Not a valid mode"),
    }
}
