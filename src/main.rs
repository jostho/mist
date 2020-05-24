use clap::{App, Arg};

const ARG_COUNT: &str = "count";
const DEFAULT_COUNT: &str = "5";

fn main() {
    let args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name(ARG_COUNT)
                .short("c")
                .long(ARG_COUNT)
                .help("Number of questions")
                .default_value(DEFAULT_COUNT)
                .validator(mist::is_valid_count),
        )
        .get_matches();

    // number of questions
    let count = args.value_of(ARG_COUNT).unwrap();
    let count = count.parse().unwrap();
    mist::ask_quiz(count);
}
