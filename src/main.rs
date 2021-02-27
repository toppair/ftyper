use std::process;
use std::str::FromStr;

use clap;

mod game;
use game::{CommandMode, Game, TimeMode};

mod types;
use types::Result;

mod words;
use words::{WordFeed, WordQueue};

mod components;
use components::{Component, Layout};

mod layout;

mod display;
use display::Display;

mod timer;
use timer::Timer;

fn main() -> Result<()> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("time")
                .short("t")
                .long("time")
                .takes_value(true)
                .value_name("TIME")
                .help("set time limit in seconds for a time mode game"),
        )
        .arg(
            clap::Arg::with_name("command")
                .short("c")
                .long("command")
                .takes_value(true)
                .value_name("COMMAND")
                .help("a command to execute")
                .multiple(true),
        )
        .get_matches();

    if let Some(values) = matches.values_of("command") {
        let command_args: Vec<&str> = values.collect();

        let program = command_args.get(0).unwrap();
        let rest = &command_args[1..];

        let mut command = process::Command::new(program);

        command
            .args(rest)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::null());

        let mode = CommandMode { command };
        let mut game = Game::new(mode);
        game.start();
    } else {
        let time = FromStr::from_str(matches.value_of("time").unwrap_or("60")).unwrap_or(60);

        let mode = TimeMode { time };
        let mut game = Game::new(mode);
        game.start();
    }

    Ok(())
}
