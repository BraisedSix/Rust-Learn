use std::env;
use std::fs;
use std::io;

fn count_stats(content: &str) -> (usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    (lines, words, chars)
}

pub fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        println!("{}", arg);
    }
    println!("args.len:{}", args.len());
    println!("args[0]:{}", args[0]);
    println!("args[1]:{}", args[1]);
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename)?;
    let (lines, words, chars) = count_stats(&content);
    println!("{} {} {} {}", lines, words, chars, filename);
    Ok(())
}
