use colored::*;
use console::Term;
use rand::seq::IndexedRandom;

pub fn game(word_list: &[String], max_mistakes: u8, hangman_states: &[&str]) {
    let mut rng = rand::rng();

    let word = match word_list.choose(&mut rng) {
        Some(i) => i,
        None => panic!("failed to choice random word"),
    };

    let word_length = word.chars().count();
    let mut guessed: Vec<Option<char>> = vec![None; word_length];
    let mut steps_left = max_mistakes;

    let mut tried_letters: Vec<char> = Vec::new();

    let mut guess_count: u8 = 0;

    clear();

    println!(
        "Enter letters one by one, try to guess the word. You have {} mistakes allowed.",
        max_mistakes
    );

    loop {
        let wrong_guesses = max_mistakes - steps_left;
        let current_hangman_state = hangman_states[wrong_guesses as usize];
        println!();
        print_indented(&current_hangman_state, 8);

        print!("\nWord: ");
        for i in &guessed {
            match i {
                Some(c) => print!("{}", c),
                None => print!("_"),
            }
            print!(" ");
        }
        println!();
        println!(
            "Checked letters: {}",
            tried_letters
                .clone()
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        println!("Mistakes left: {}", steps_left);

        if !guessed.iter().any(|x| x.is_none()) {
            println!(
                "{}",
                format!(
                    "You won by guessing all the letters in {} attempts. Word: {}",
                    guess_count, word
                )
                .bright_green()
            );

            break;
        }

        if steps_left == 0 {
            println!("{}", format!("You lost. Word: {}", word).bright_red());
            break;
        }

        println!("Enter letter:");
        let mut input: Option<char> = None;

        let term = Term::stdout();
        if let Ok(c) = term.read_char() {
            input = Some(c);
        }

        let letter = input.expect("no letter");

        clear();

        if !letter.is_alphabetic() {
            println!("{}", format!("'{letter}' is not valid letter").yellow());
            continue;
        }
        if tried_letters.contains(&letter) {
            println!(
                "{}",
                format!("Letter '{}' you already tried", letter).yellow()
            );

            continue;
        }
        tried_letters.push(letter);
        guess_count += 1;

        let indices: Vec<usize> = word
            .char_indices()
            .filter(|&(_, ch)| ch == letter)
            .map(|(idx, _)| idx)
            .collect();

        let len_indices = indices.len();
        if len_indices == 0 {
            println!(
                "{}",
                format!("Letter '{letter}' not contains in secret word").red()
            );
            steps_left -= 1;
        } else {
            println!(
                "{}",
                format!("Letter '{letter}' contains in secret word {len_indices} count").green()
            );
        }

        for i in indices {
            guessed[i] = Some(letter);
        }
    }
}

fn clear() {
    Term::stdout().clear_screen().unwrap();
}
fn print_indented(text: &str, indent: usize) {
    let spaces = " ".repeat(indent);
    for line in text.lines() {
        println!("{}{}", spaces, line);
    }
}
