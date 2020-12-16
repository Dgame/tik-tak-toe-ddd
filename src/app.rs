use crate::domain::{
    Mark, Name, PixelCoord, Player, Playground, PlaygroundDisplay, Reader, Writer, O, X,
};
use std::convert::TryFrom;

pub struct Game<D: PlaygroundDisplay, W: Writer, R: Reader> {
    playground: Playground,
    x: X,
    o: O,
    displayer: D,
    writer: W,
    reader: R,
}

impl<D: PlaygroundDisplay, W: Writer, R: Reader> Game<D, W, R> {
    pub fn single_player(name: Name, displayer: D, reader: R, writer: W) -> Self {
        Self {
            playground: Playground::default(),
            x: X::new(Player::Human { name }),
            o: O::new(Player::KI),
            displayer,
            writer,
            reader,
        }
    }

    pub fn multi_player(x: X, o: O, displayer: D, reader: R, writer: W) -> Self {
        Self {
            playground: Playground::default(),
            x,
            o,
            displayer,
            writer,
            reader,
        }
    }

    pub fn play(&mut self) {
        loop {
            self.display();

            if self.playground.is_full() {
                self.writer.writeln("We've reached a draw.");
                break;
            }

            let pos = self.get_position(self.x.player());
            self.playground
                .mark_field_with(&pos, Mark::X)
                .expect("Could not mark that field with X");
            if self.playground.has_won(Mark::X) {
                self.display();

                self.writer.writeln("X has WON");
                break;
            }

            let pos = self.get_position(self.o.player());
            self.playground
                .mark_field_with(&pos, Mark::O)
                .expect("Could not mark that field with O");
            if self.playground.has_won(Mark::O) {
                self.display();

                self.writer.writeln("O has WON");
                break;
            }
        }
    }

    fn display(&self) {
        self.displayer.display(&self.playground);
    }

    fn get_position(&self, player: &Player) -> PixelCoord {
        loop {
            let pos = match player {
                Player::Human { name } => self.ask_for_direction(&name),
                Player::KI => self.get_random_position(),
            };

            if !self.playground.is_field_occupied(&pos) {
                return pos;
            }

            self.writer
                .writeln("That field is already taken. Please choose another.");
        }
    }

    fn ask_for_direction(&self, name: &Name) -> PixelCoord {
        loop {
            let msg = format!(
                "{} it's your turn. Where do you want to place your mark? Your input should be the column direction (top, center, bottom) and the row direction (left, center, right) separated by a minus, e.g. \"top-left\" or \"center\"",
                name
            );
            self.writer.writeln(&msg);

            let input = self.reader.readln();
            if let Ok(pos) = PixelCoord::try_from(input.as_str()) {
                return pos;
            }

            self.writer.writeln("That is not a valid input...");
        }
    }

    fn get_random_position(&self) -> PixelCoord {
        use random_number::random;

        PixelCoord {
            x: random!(..=2),
            y: random!(..=2),
        }
    }
}
