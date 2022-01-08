use std::fs::*;
use std::io::*;
mod words;
use words::*;

fn most_common_letters(word_list: &Vec<String>) -> Vec<u64> {
    let mut counts = vec![0; 26];

    for word in word_list {
        let mut unique_chars = word.chars().collect::<Vec<char>>();
        unique_chars.sort();
        unique_chars.dedup();

        for c in unique_chars {
            counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    counts
}

fn main() {
    println!("Welcome to Wordle Solver!");
    
    let mut words_5 = WORD_LIST
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>();
    
    words_5.append(&mut SOLUTION_LIST.iter().map(|w| w.to_string()).collect::<Vec<String>>());

    let mut disallowed_letters: Vec<String> = Vec::new();
    let mut needed_letters: Vec<String> = Vec::new();
    let mut needed_positioned_letters: Vec<(String, usize)> = Vec::new();

    println!("Loaded!");

    // Start solving loop
    for i in 0..5 {
        // Remove words with disallowed letters && those without needed letters
        words_5 = words_5
            .iter()
            .filter(|word| {
                let mut dis_valid = true;
                let mut needed_valid = false;

                for c in word.chars() {
                    if disallowed_letters.contains(&c.to_string()) {
                        dis_valid = false;
                    }

                    if needed_letters.contains(&c.to_string()) || needed_letters.len() == 0 {
                        needed_valid = true;
                    }
                }

                dis_valid && needed_valid
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        // Remove words with non-correctly positioned letters
        words_5 = words_5
            .iter()
            .filter(|word| {
                let mut correct_positioned_letters = true;
                for pair in &needed_positioned_letters {
                    if word.chars().nth(pair.1) != Some(pair.0.chars().nth(0).unwrap()) {
                        correct_positioned_letters = false;
                    }
                }
                correct_positioned_letters
            })
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        println!("Word list size: {}", words_5.len());

        // Find most common letters
        let counts = most_common_letters(&words_5);

        // First, find the best possible word
        let mut best_word = String::new();
        let mut best_count = 0;

        for word in &words_5 {
            let mut count = 0;

            let mut unique_chars = word.chars().collect::<Vec<char>>();
            unique_chars.sort();
            unique_chars.dedup();

            for c in unique_chars {
                count += counts[(c as u8 - b'a') as usize];
            }

            if count > best_count {
                best_count = count;
                best_word = word.to_string();
            }
        }

        println!("Guess #{}: {}", i + 1, best_word);
        println!("Input the feedback for this guess: 'c' for correct pos. and letter, 'p' for correct letter, and 'n' for no match");
        // Get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "ccccc" {
            println!("You win!");
            std::process::exit(0);
        }
        
        // Parse input
        for (index, c) in input.char_indices() {
            if c == 'c' {
                needed_positioned_letters
                    .push((best_word.chars().nth(index).unwrap().to_string(), index));
            } else if c == 'p' {
                needed_letters.push(best_word.chars().nth(index).unwrap().to_string());
            } else if c == 'n' {
                disallowed_letters.push(best_word.chars().nth(index).unwrap().to_string());
            }
        }
    }
}
