use ansi_term::Colour::{Cyan, Green, Red};
use rand::Rng;
use std::io;
use std::time::Instant;

const STEP_LEVEL: u8 = 10;

pub fn ask_quiz(count: u8, level: u8) {
    let mut rng = rand::thread_rng();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    let max = STEP_LEVEL + (level * STEP_LEVEL);
    let header = format!("Count: {}, Level: {}, Max: {}", count, level, max);
    println!("{}", Cyan.paint(header));

    let start_time = Instant::now();
    while !done {
        let a = rng.gen_range(1..max);
        let b = rng.gen_range(1..max);
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
            "Your answer is {}. {} {} {} = {}",
            verdict, a, oper, b, answer
        );
        println!("{}", Cyan.paint("-"));
        if q_count == count {
            done = true;
        }
    }
    let footer = format!(
        "Score: {}/{}, Time: {}s",
        correct_answer_count,
        count,
        start_time.elapsed().as_secs()
    );
    println!("{}", Cyan.paint(footer));
}

pub fn ask_sequence(count: u8, level: u8) {
    let mut rng = rand::thread_rng();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    let max = level * STEP_LEVEL;
    let header = format!("Count: {}, Level: {}, Max: {}", count, level, max);
    println!("{}", Cyan.paint(header));

    let start_time = Instant::now();
    while !done {
        let num: u32 = rng.gen_range(2..max).into();
        let mut multiples: Vec<u32> = (1..11).map(|x| x * num).collect();
        let index: usize = rng.gen_range(1..multiples.len());
        let answer = multiples[index];
        multiples[index] = 0;
        let mut multiples_as_str: Vec<String> = multiples.iter().map(|x| x.to_string()).collect();
        multiples_as_str[index] = "x".to_string();
        q_count += 1;
        println!(
            "Question {}/{}: What is the missing number: {}",
            q_count,
            count,
            Cyan.paint(multiples_as_str.join(", "))
        );
        let mut input = String::new();
        let _result = io::stdin().read_line(&mut input);
        let input: u32 = input.trim().parse().unwrap_or(0);
        let mut verdict = Red.bold().paint("wrong");
        if input == answer {
            correct_answer_count += 1;
            verdict = Green.bold().paint("correct");
        }
        println!("Your answer is {}. Answer is {}", verdict, answer);
        println!("{}", Cyan.paint("-"));
        if q_count == count {
            done = true;
        }
    }
    let footer = format!(
        "Score: {}/{}, Time: {}s",
        correct_answer_count,
        count,
        start_time.elapsed().as_secs()
    );
    println!("{}", Cyan.paint(footer));
}
