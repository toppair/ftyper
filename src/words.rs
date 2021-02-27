use rand::Rng;
use std::ops::Range;
use termion::event::Key;

const WORDS: &str = include_str!("ngsl.txt");

#[allow(dead_code)]
enum Color {
    Reset,
    Green,
    Red,
    Yellow,
    White,
}

impl Color {
    pub fn value(&self) -> &str {
        match self {
            Color::Reset => "\u{001b}[0m",
            Color::Green => "\u{001b}[32m",
            Color::Red => "\u{001b}[31m",
            Color::Yellow => "\u{001b}[33m",
            Color::White => "\u{001b}[37m",
        }
    }
}

#[derive(Debug)]
pub struct WordFeed<'a> {
    words: Vec<&'a str>,
}

impl<'a> WordFeed<'a> {
    pub fn new() -> Self {
        let words: Vec<&str> = WORDS.lines().collect();

        WordFeed { words }
    }

    pub fn get_random(&self) -> LiveWord {
        let random_int: u32 = random(0..self.words.len() as u32);
        let word = &self.words[random_int as usize];
        LiveWord::new(word.to_string())
    }
}

#[derive(Debug)]
pub struct LiveWord((String, String), bool);

impl LiveWord {
    pub fn new(expected: String) -> Self {
        LiveWord((expected, String::new()), false)
    }

    pub fn expected(&self) -> String {
        self.0 .0.clone()
    }

    pub fn actual(&self) -> String {
        self.0 .1.clone()
    }

    pub fn push_char(&mut self, c: char) -> bool {
        self.0 .1.push(c);
        self.1 = self.0 .0 == self.0 .1;
        self.1
    }

    pub fn pop_char(&mut self) -> bool {
        self.0 .1.pop();
        self.1 = self.0 .0 == self.0 .1;
        self.1
    }

    pub fn correct_stroke_count(&self) -> u16 {
        let mut count = 0;
        for (a, e) in self.actual().chars().zip(self.expected().chars()) {
            if a == e {
                count += 1
            } else {
                break;
            }
        }
        count
    }

    pub fn is_correct(&self) -> bool {
        self.1
    }

    pub fn to_color_string_outcome(&self) -> String {
        let color = if self.is_correct() {
            Color::Green.value()
        } else {
            Color::Red.value()
        };
        format!("{}{}{}", color, self.expected(), Color::Reset.value())
    }

    pub fn to_color_string_outcome_detail(&self) -> String {
        let colors = (
            Color::Green.value(),
            Color::Red.value(),
            Color::Reset.value(),
        );
        get_color_string_outcome_detail(&self.expected(), &self.actual(), colors)
    }
}

pub struct WordQueue<'a> {
    feed: WordFeed<'a>,
    current_index: u8,
    fit_row_into_len: u8,
    num_rows: u8,
    rows: Vec<Vec<LiveWord>>,
    correct_count: u16,
    incorrect_count: u16,
    correct_stroke_count: u16,
}

impl<'a> WordQueue<'a> {
    pub fn new(feed: WordFeed<'a>) -> Self {
        Self {
            feed,
            current_index: 0,
            fit_row_into_len: 60,
            num_rows: 2,
            rows: vec![],
            correct_count: 0,
            incorrect_count: 0,
            correct_stroke_count: 0,
        }
    }

    pub fn init(&mut self) {
        for _ in 0..self.num_rows {
            self.rows.push(self.gen_row());
        }
    }

    fn gen_row(&self) -> Vec<LiveWord> {
        gen_row(&self.feed, self.fit_row_into_len)
    }

    pub fn get_current_word_ref(&mut self) -> &mut LiveWord {
        self.rows
            .get_mut(0)
            .unwrap()
            .get_mut(self.current_index as usize)
            .unwrap()
    }

    fn move_index(&mut self) {
        if self.get_current_word_ref().is_correct() {
            self.correct_count += 1;
        } else {
            self.incorrect_count += 1;
        }

        self.correct_stroke_count += self.get_current_word_ref().correct_stroke_count();

        if let Some(_) = self
            .rows
            .get(0)
            .unwrap()
            .get(self.current_index as usize + 1)
        {
            self.current_index += 1;
        } else {
            self.flush();
        }
    }

    pub fn words_count(&self) -> (u16, u16) {
        (self.correct_count, self.incorrect_count)
    }

    pub fn correct_stroke_count(&self) -> u16 {
        self.correct_stroke_count
    }

    fn flush(&mut self) {
        self.rows.remove(0);
        self.rows.push(self.gen_row());
        self.current_index = 0;
    }

    pub fn get_parsed(&self) -> Vec<String> {
        self.rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i == 0 {
                    Self::to_string(row, Some(self.current_index))
                } else {
                    Self::to_string(row, None)
                }
            })
            .collect()
    }

    fn to_string(words: &[LiveWord], active_index: Option<u8>) -> String {
        if let Some(index) = active_index {
            to_colored_string(words, index)
        } else {
            to_string(words)
        }
    }

    pub fn register_key(&mut self, key: Key) {
        let word = self.get_current_word_ref();
        match key {
            Key::Char(c) => {
                if c != ' ' {
                    word.push_char(c);
                } else {
                    if word.actual().len() != 0 {
                        self.move_index();
                    }
                }
            }
            Key::Backspace => {
                word.pop_char();
            }
            _ => {}
        }
    }
}

fn gen_row(feed: &WordFeed, fit_row_into_len: u8) -> Vec<LiveWord> {
    let mut total_lenght = 0;
    let mut words = Vec::new();

    loop {
        let word = feed.get_random();
        let fits = total_lenght + word.expected().len() < fit_row_into_len as usize;

        if !fits {
            break;
        };

        total_lenght += word.expected().len() + 1;
        words.push(word);
    }

    words
}

fn to_colored_string(words: &[LiveWord], index: u8) -> String {
    let mut buffer = String::new();
    let i = index as usize;
    for (x, word) in words.iter().enumerate() {
        let string = if x < i {
            word.to_color_string_outcome()
        } else if x == i {
            word.to_color_string_outcome_detail()
        } else {
            word.expected()
        };
        buffer.push_str(&string);
        buffer.push(' ');
    }

    buffer
}

fn to_string(words: &[LiveWord]) -> String {
    let mut buffer = String::new();
    for word in words.iter() {
        buffer.push_str(&word.expected());
        buffer.push(' ');
    }
    buffer
}

fn get_color_string_outcome_detail(
    expected: &String,
    actual: &String,
    colors: (&str, &str, &str),
) -> String {
    let (green, red, reset) = colors;
    let mut a_chars = actual.chars();
    let mut buffer = String::new();
    let mut stop = false;

    buffer.push_str(green);

    for e_char in expected.chars() {
        if stop {
            buffer.push(e_char);
            continue;
        }

        if let Some(a_char) = a_chars.next() {
            if e_char == a_char {
                buffer.push(e_char);
            } else {
                buffer.push_str(red);
                buffer.push(e_char);
                buffer.push_str(reset);
                stop = true
            }
        } else {
            buffer.push_str(reset);
            buffer.push(e_char);
        }
    }

    buffer.push_str(reset);

    buffer
}

pub fn random(range: Range<u32>) -> u32 {
    rand::thread_rng().gen_range(range)
}
