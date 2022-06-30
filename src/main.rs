use clap::{ArgEnum, Parser};

/// Read cli args
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Mode
    #[clap(short, long, arg_enum, value_parser)]
    mode: Mode,

    /// Number of questions
    #[clap(short, long, default_value_t = 5, value_parser = clap::value_parser!(u8).range(1..100))]
    count: u8,

    /// Level
    #[clap(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..10))]
    level: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Quiz,
    Sequence,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Quiz => mist::ask_quiz(args.count, args.level),
        Mode::Sequence => mist::ask_sequence(args.count, args.level),
    }
}
