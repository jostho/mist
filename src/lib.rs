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
    is_valid(val, MAX_COUNT)
}

pub fn is_valid_level(val: String) -> Result<(), String> {
    is_valid(val, MAX_LEVEL)
}

fn is_valid(val: String, max: u8) -> Result<(), String> {
    let value: u8 = match val.parse() {
        Ok(value) => value,
        Err(e) => return Err(e.to_string()),
    };

    if value < max {
        Ok(())
    } else {
        Err(format!("value should be less than {}", max))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_for_string() {
        let result = is_valid("str".to_string(), MAX_COUNT);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid digit found in string");
    }

    #[test]
    fn is_valid_for_42() {
        let result = is_valid("42".to_string(), MAX_COUNT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn is_valid_for_100() {
        let result = is_valid("100".to_string(), MAX_COUNT);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!("value should be less than {}", MAX_COUNT)
        );
    }
}
