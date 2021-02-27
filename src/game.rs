use crate::Component;
use crate::Display;
use crate::Layout;
use crate::Timer;
use crate::WordFeed;
use crate::WordQueue;
use std::collections::HashMap;
use std::io;
use std::process;
use std::thread;
use std::time;
use termion::cursor;
use termion::event;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub trait Mode {}

pub struct TimeMode {
    pub time: u32,
}

pub struct CommandMode {
    pub command: process::Command,
}

impl Mode for TimeMode {}
impl Mode for CommandMode {}

pub struct Game<'a, M: Mode> {
    mode: M,
    word_queue: WordQueue<'a>,
    layout: Layout,
    display: Display,
    timer: Timer,
}

impl<'a, M: Mode> Game<'a, M> {
    pub fn new(mode: M) -> Self {
        new_game(mode)
    }

    fn update_layout(&mut self) {
        let rows = self.word_queue.get_parsed();
        self.layout.update("words", ("row1", rows.get(0).unwrap()));
        self.layout.update("words", ("row2", rows.get(1).unwrap()));
        self.layout.update(
            "word",
            ("word", &self.word_queue.get_current_word_ref().actual()[..]),
        );
    }

    fn process_key(&mut self, key: event::Key) -> Result<(), ()> {
        match key {
            event::Key::Ctrl('c') => Err(()),
            _ => {
                if !self.timer.running() {
                    self.timer.start()
                }
                self.word_queue.register_key(key);
                self.update_layout();
                self.display.render(&self.layout).unwrap();
                Ok(())
            }
        }
    }
}

impl<'a> Game<'a, TimeMode> {
    pub fn start(&mut self) {
        self.timer.set(self.mode.time);
        let mut _stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdin = termion::async_stdin().keys();
        self.update_layout();
        self.display.render(&self.layout).unwrap();

        loop {
            thread::sleep(time::Duration::from_millis(50));

            if self.timer.is_limit() {
                break;
            }

            if let Some(Ok(key)) = stdin.next() {
                if let Err(_) = self.process_key(key) {
                    break;
                }
            }
        }

        self.end();
    }

    fn end(&mut self) {
        let score_layout = build_score_layout(self);
        self.display.render(&score_layout).unwrap();
    }
}

impl<'a> Game<'a, CommandMode> {
    pub fn start(&mut self) {
        let mut child: process::Child;
        if let Ok(c) = self.mode.command.spawn() {
            child = c;
        } else {
            println!("error: failed to start command");
            return;
        }
        let mut _stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdin = termion::async_stdin().keys();
        self.update_layout();
        self.display.render(&self.layout).unwrap();

        loop {
            thread::sleep(time::Duration::from_millis(50));

            if let Ok(Some(_)) = child.try_wait() {
                self.print_output(child);
                break;
            }

            if let Some(Ok(key)) = stdin.next() {
                if let Err(_) = self.process_key(key) {
                    child.kill().unwrap();
                    break;
                }
            }
        }

        self.end();
    }

    fn print_output(&self, child: process::Child) {
        if let Ok(output) = child.wait_with_output() {
            Display::clear();
            if let Ok(o) = String::from_utf8(output.stdout) {
                for line in o.split('\n').into_iter() {
                    println!("{}", line);
                    print!("{}", cursor::Left(100));
                }
            }
            print!("{}", cursor::Left(100));
            println!("Your process has finished. The output is above. Here's your score:");
        }
    }

    fn end(&mut self) {
        let score_layout = build_score_layout(self);
        self.display.render_no_clear(&score_layout).unwrap();
    }
}

fn build_score_layout<M: Mode>(game: &Game<M>) -> Layout {
    let score = Component::new("score");
    let mut score_state = HashMap::new();

    let (correct, incorrect) = game.word_queue.words_count();

    let raw_score_layout = vec![vec![score]];

    let correct_stroke_count = game.word_queue.correct_stroke_count();

    let mut score_layout = Layout {
        layout: raw_score_layout,
    };
    let accuracy = if correct + incorrect == 0 {
        0.0
    } else {
        correct as f32 / (correct + incorrect) as f32 * 100.0
    };
    let wpm = if game.timer.passed() == 0 {
        0.0
    } else {
        correct_stroke_count as f32 / 5.0 / game.timer.passed() as f32 * 60.0
    };

    score_state.insert("correct".to_string(), correct.to_string());
    score_state.insert("incorrect".to_string(), incorrect.to_string());
    score_state.insert("accuracy".to_string(), format!("{:.2}", accuracy));
    score_state.insert("wpm".to_string(), format!("{:.0}", wpm));
    score_state.insert("time".to_string(), game.timer.passed().to_string());

    score_layout.replace("score", &score_state);

    score_layout
}

fn new_game<'a, M: Mode>(mode: M) -> Game<'a, M> {
    let feed = WordFeed::new();
    let mut word_queue = WordQueue::new(feed);
    let display = Display::new();
    let layout = Layout {
        layout: vec![vec![Component::new("words")], vec![Component::new("word")]],
    };
    let timer = Timer::new(60);

    word_queue.init();
    Game {
        mode,
        word_queue,
        layout,
        display,
        timer,
    }
}
