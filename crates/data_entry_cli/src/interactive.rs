use std::io::{Read, stdin};

use reals_server_bot_discord_scraper::game_msg_parser::GameSetMessageParser;

use crate::types::SetDate;
use anyhow::anyhow;

#[derive(Debug)]
struct InteractiveState {
    set_parser: GameSetMessageParser,
    active_date: SetDate,
}

impl InteractiveState {
    fn new(active_date: SetDate) -> Self {
        Self {
            set_parser: GameSetMessageParser::default(),
            active_date,
        }
    }
}

pub(crate) fn interactive_loop(active_date: SetDate) {
    let mut should_exit = false;
    let mut state = InteractiveState::new(active_date);

    while !should_exit {
        match process_and_handle_user_input(&mut state) {
            Ok(new_exit_state) => should_exit = new_exit_state,
            Err(err_str) => println!("{}", err_str),
        }
    }
}

fn process_and_handle_user_input(state: &mut InteractiveState) -> anyhow::Result<bool> {
    let mut should_exit = false;

    let input = read_input_from_stdin();

    let mut words = input.split(",");
    let first_word = match words.next() {
        Some(w) => w,
        None => return Ok(false), // Not an error. Just silently wait for the next line.
    };

    let remaining_words: String = words.collect();

    match first_word {
        "add" => add_set(&remaining_words, state)?,
        "date" => set_date(&remaining_words, state)?,
        "quit" | "exit" => should_exit = true,
        _ => println!("{} is not recognized as a valid command", first_word),
    }

    Ok(should_exit)
}

fn add_set(set_str: &str, state: &mut InteractiveState) -> anyhow::Result<()> {
    let set_data = state
        .set_parser
        .extract_game_set_data_from_discord_msg_if_any(set_str)
        .ok_or(anyhow!("Unable to extract set data from string!"))?;

    Ok(())
}

fn set_date(date_arg: &str, state: &mut InteractiveState) -> anyhow::Result<()> {
    state.active_date = dateparser::parse(date_arg)?;
    Ok(())
}

fn read_input_from_stdin() -> String {
    let mut buf = String::new();
    stdin()
        .read_to_string(&mut buf)
        .expect("Unable to read line from standard input!");

    buf
}
