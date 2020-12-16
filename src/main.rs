use crate::app::Game;
use crate::infra::{
    BracketFieldFormatter, TerminalPlaygroundDisplay, TerminalReader, TerminalWriter,
};
use std::convert::TryInto;

mod app;
mod domain;
mod infra;

fn main() {
    let writer = TerminalWriter;
    let reader = TerminalReader;
    let displayer = TerminalPlaygroundDisplay::new(BracketFieldFormatter);

    let mut game = Game::single_player(
        "Dgame".try_into().expect("Invalid name"),
        displayer,
        reader,
        writer,
    );
    game.play();
}
