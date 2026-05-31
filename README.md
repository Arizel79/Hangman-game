# Hangman game
A classic Hangman game written in Rust. Guess the secret word letter by letter before the hangman drawing is completed.  
The game supports custom word lists and adjustable difficulty via command‑line arguments.

## Run
Clone this repository:
```
git clone https://github.com/Arizel79/Hangman-game
cd Hangman-game
```
Run:
```
cargo run --release
```
## How to play
* The computer randomly chooses a secret word from the provided word list.
* You see underscores (_) representing each missing letter.
* Type a single letter to guess.
* If the letter is in the word, it appears in all correct positions.
* If the letter is NOT in the word, you lose one mistake.
* You have a limited number of mistakes.
* Win by revealing all letters before the hangman drawing is complete.
* Lose if you run out of mistakes - the hangman is fully drawn.
  
## Options
```
Hangman game – guess the secret word before the man is hanged

Usage: hangman [OPTIONS]

Options:
  -w, --words <FILE>      Path to a file with words (one word per line). Can be used multiple times
  -m, --max-mistakes <N>  Maximum number of incorrect guesses allowed
  -h, --help              Print help
  -V, --version           Print version
```
## Examples
```
cargo run --release -- --words words/bip32.txt --max-mistakes 9
