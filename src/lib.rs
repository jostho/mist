use ansi_term::Colour::{Cyan, Green, Red};
use rand::Rng;
use std::io;
use std::time::Instant;

// all u8s - please stay under 255, u8::MAX
const MAX_COUNT: u8 = 100;
const MAX_LEVEL: u8 = 10;
const STEP_LEVEL: u8 = 10;
const MAX_INT: u8 = 11;

pub fn is_valid_count(val: String) -> Result<(), String> {
    let count: u8 = match val.parse() {
        Ok(count) => count,
        Err(e) => return Err(e.to_string()),
    };

    if count < MAX_COUNT {
        Ok(())
    } else {
        Err(format!("Value should be less than {}", MAX_COUNT))
    }
}

pub fn is_valid_level(val: String) -> Result<(), String> {
    let level: u8 = match val.parse() {
        Ok(level) => level,
        Err(e) => return Err(e.to_string()),
    };

    if level < MAX_LEVEL {
        Ok(())
    } else {
        Err(format!("Value should be less than {}", MAX_LEVEL))
    }
}

pub fn ask_quiz(count: u8, level: u8) {
    let mut rng = rand::thread_rng();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    let max = MAX_INT + (level * STEP_LEVEL);
    let header = format!("Count: {}, Level: {}, Max: {}", count, level, max);
    println!("{}", Cyan.paint(header));

    let start_time = Instant::now();
    while !done {
        let a = rng.gen_range(1, max);
        let b = rng.gen_range(1, max);
        if a <= b {
            continue;
        }
        // it's a GO
        let mut answer = a + b;
        let mut oper = "+";
        // decide between addition or substraction
        let num: f32 = rng.gen();
        if num > 0.5 {
            answer = a - b;
            oper = "-";
        }
        q_count += 1;
        println!(
            "Question {}/{}: What is {} {} {} = ?",
            q_count, count, a, oper, b
        );
        let mut input = String::new();
        let _result = io::stdin().read_line(&mut input);
        let input: u8 = input.trim().parse().unwrap_or(0);
        let mut verdict = Red.bold().paint("wrong");
        if input == answer {
            correct_answer_count += 1;
            verdict = Green.bold().paint("correct");
        }
        println!(
            "Your answer is {}. {} {} {} = {}\n",
            verdict, a, oper, b, answer
        );
        if q_count == count {
            done = true;
        }
    }
    println!(
        "Final score: {}/{} . Time: {}s",
        correct_answer_count,
        count,
        start_time.elapsed().as_secs()
    );
}
