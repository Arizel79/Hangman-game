mod consts;
mod game;

use clap::Parser;
use consts::{DEFAULT_MAX_MISTAKES, DEFAULT_WORDS, HANGMAN_STATES};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Hangman game – guess the secret word before the man is hanged.
#[derive(Parser)]
#[command(
    version,
    about,
    after_help = r#"
HOW TO PLAY:
  • The computer randomly chooses a secret word from the provided word list.
  • You see underscores (_) representing each missing letter.
  • Type a single letter to guess.
  • If the letter is in the word, it appears in all correct positions.
  • If the letter is NOT in the word, you lose one mistake.
  • You have a limited number of mistakes.
  • Win by revealing all letters before the hangman drawing is complete.
  • Lose if you run out of mistakes - the hangman is fully drawn.

EXAMPLES:
  hangman
  hangman --words words.txt --max-mistakes 8
  hangman -w words1.txt -w words2.txt -m 5

NOTE:
  Word files should contain one word per line (case-sensitive; words are used as-is).
  The -w/--words option can be used multiple times to combine words from several files.
"#
)]
struct Args {
    /// Path to a file with words (one word per line). Can be used multiple times.
    #[arg(short = 'w', long = "words", value_name = "FILE")]
    words: Vec<String>,

    /// Maximum number of incorrect guesses allowed
    #[arg(short = 'm', long = "max-mistakes", value_name = "N")]
    max_mistakes: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let word_list = if args.words.is_empty() {
        DEFAULT_WORDS.iter().map(|&s| s.to_string()).collect()
    } else {
        let mut all_words = Vec::new();
        for file_path in &args.words {
            match read_words_from_file(file_path) {
                Ok(words) => all_words.extend(words),
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
        if all_words.is_empty() {
            eprintln!("Error: no valid words found in the provided files.");
            std::process::exit(1);
        }
        all_words
    };

    let max_mistakes = args
        .max_mistakes
        .unwrap_or(DEFAULT_MAX_MISTAKES)
        .clamp(1, (HANGMAN_STATES.len() - 1) as u8);

    if max_mistakes <= 0 || max_mistakes > DEFAULT_MAX_MISTAKES {
        eprintln!(
            "Error: max-mistakes must be between 0 and {}",
            DEFAULT_MAX_MISTAKES
        );
        std::process::exit(1);
    }

    let start_index = HANGMAN_STATES.len() - (max_mistakes as usize + 1);
    let hangman_states = &HANGMAN_STATES[start_index..];

    game::game(&word_list, max_mistakes, hangman_states);
}

fn read_words_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(words)
}
