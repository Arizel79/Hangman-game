pub const DEFAULT_WORDS: [&str; 10] = [
    "apple", "banana", "cherry", "dog", "elephant", "fish", "grape", "horse", "igloo", "jacket",
];
pub const DEFAULT_MAX_MISTAKES: u8 = 9;
pub const HANGMAN_STATES: [&str; 10] = [
    "






========",
    "      +
      |
      |
      |
      |
      |
========",
    "  +---+
      |
      |
      |
      |
      |
========",
    "  +---+
  |   |
      |
      |
      |
      |
========",
    "  +---+
  |   |
  O   |
      |
      |
      |
========",
    "  +---+
  |   |
  O   |
  |   |
      |
      |
========",
    "  +---+
  |   |
  O   |
  |\\  |
      |
      |
========",
    "  +---+
  |   |
  O   |
 /|\\  |
      |
      |
========",
    "  +---+
  |   |
  O   |
 /|\\  |
   \\  |
      |
========",
    "  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
========",
];
