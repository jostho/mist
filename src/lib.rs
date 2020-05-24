use ansi_term::Colour::{Green, Red};
use rand::Rng;
use std::io;
use std::time::Instant;

const MAX_COUNT: u8 = 100; // stay under 255, u8::MAX
const MAX_INT_A: u8 = 21;
const MAX_INT_B: u8 = 11;

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

pub fn ask_quiz(count: u8) {
    let mut rng = rand::thread_rng();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    let start_time = Instant::now();
    while !done {
        let a = rng.gen_range(1, MAX_INT_A);
        let b = rng.gen_range(1, MAX_INT_B);
        if a < b {
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
        let mut verdict = Red.paint("wrong");
        if input == answer {
            correct_answer_count += 1;
            verdict = Green.paint("correct");
        }
        println!(
            "Your answer is {}. {} {} {} = {}",
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
