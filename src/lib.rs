use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

const WORDS: &str = include_str!("words.txt");
pub const GUESS_LENGTH: usize = 5;
pub const GUESS_MAX: usize = 6;

pub struct Dictionary {
    words: HashSet<&'static str>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split("\n").collect();

        Self { words }
    }

    pub fn get_random_word(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum HitAccuracy {
    InRightPlace,
    InWord,
    NotInWord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GuessResult {
    DuplicateGuess,  //推理単語が重複している
    IncorrectLength, //文字数が不正
    NotInDictionary, //単語辞書にない
    Valid,           //有効
    GameOver,        //既にゲームが終了している
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Won,        //勝利(推理が当たった)
    InProgress, //推理中
    Lost,       //敗北(推理が全て当たらなかった)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GuessLetter {
    pub letter: char,
    pub accuracy: HitAccuracy,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordGuess {
    pub letters: Vec<GuessLetter>,
}

impl WordGuess {
    pub fn word(&self) -> String {
        self.letters.as_slice().iter().map(|gl| gl.letter).collect()
    }
}
pub struct Game {
    guesses: Vec<WordGuess>,
    answer: String,
    dictionary: Dictionary,
    game_status: GameStatus,
}

impl Default for Game {
    fn default() -> Self {
        let dict = Dictionary::new();
        let answer = dict.get_random_word();

        Game {
            answer,
            dictionary: dict,
            guesses: Vec::with_capacity(GUESS_MAX),
            game_status: GameStatus::InProgress,
        }
    }
}

impl Game {
    pub fn get_answer(&self) -> String {
        self.answer.to_string()
    }
    pub fn in_dictionary(&self, word: &str) -> bool {
        self.dictionary.words.get(word).is_some()
    }
    fn build_letter_counts(&self, word: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for c in word.chars() {
            match counts.get_mut(&c) {
                Some(v) => *v += 1,
                None => {
                    counts.insert(c, 1);
                }
            };
        }
        counts
    }
    fn answer_char_at_index(&self, index: usize) -> char {
        self.answer.chars().nth(index).unwrap()
    }
    fn matches_answer_at_index(&self, index: usize, letter: char) -> bool {
        letter == self.answer_char_at_index(index)
    }
    fn build_guess_letter_with_accuracy(
        &mut self,
        letter_index: usize,
        letter: char,
        available_letters: &mut HashMap<char, usize>,
    ) -> GuessLetter {
        let accuracy = match &self.answer.contains(letter) {
            true => {
                let in_same_place = self.matches_answer_at_index(letter_index, letter);

                if in_same_place {
                    if let Some(ch) = available_letters.get_mut(&letter) {
                        *ch -= 1;
                    }

                    HitAccuracy::InRightPlace
                } else if let Some(ch) = available_letters.get_mut(&letter) {
                    if (*ch) >= 1 {
                        *ch -= 1;
                        HitAccuracy::InWord
                    } else {
                        HitAccuracy::NotInWord
                    }
                } else {
                    HitAccuracy::NotInWord
                }
            }
            false => HitAccuracy::NotInWord,
        };

        GuessLetter { letter, accuracy }
    }

    fn build_guess(&mut self, guess_input: &str) -> WordGuess {
        let mut available_letters = self.build_letter_counts(&self.answer);

        let mut guess_letters: Vec<Option<GuessLetter>> = vec![None; GUESS_LENGTH];

        for (idx, c) in guess_input.chars().enumerate() {
            if self.matches_answer_at_index(idx, c) {
                guess_letters[idx] =
                    Some(self.build_guess_letter_with_accuracy(idx, c, &mut available_letters))
            }
        }

        for (idx, c) in guess_input.chars().enumerate() {
            if guess_letters[idx].is_none() {
                guess_letters[idx] =
                    Some(self.build_guess_letter_with_accuracy(idx, c, &mut available_letters))
            }
        }

        WordGuess {
            letters: guess_letters.iter().map(|o| o.unwrap()).collect(),
        }
    }
    pub fn guess(&mut self, guess_input: &str) -> (GameStatus, GuessResult) {
        if self.game_status == GameStatus::Won || self.game_status == GameStatus::Lost {
            return (self.game_status, GuessResult::GameOver);
        }

        if guess_input.len() != GUESS_LENGTH {
            return (self.game_status, GuessResult::IncorrectLength);
        }

        let guess = self.build_guess(guess_input);
        self.guesses.push(guess);

        (self.game_status, GuessResult::Valid)
    }

    fn guess_already_exists(&self, guess_input: &str) -> bool {
        self.guesses
            .iter()
            .map(|g| g.word())
            .any(|x| x.eq(guess_input))
    }
}
