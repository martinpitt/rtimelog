// Copyright (C) 2023 Martin Pitt <martin@piware.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[derive(PartialEq, Debug)]
pub enum TimeMode {
    Day(u32),
    Week(u32),
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Nothing,
    Quit,
    Help,
    Edit,
    SwitchMode(TimeMode),
    Add(String),
    Error(String),
}

impl Command {
    pub fn parse(input: String) -> Command {
        match input.chars().next() {
            None => Command::Nothing,

            Some(':') => match input.as_str() {
                ":q" => Command::Quit,
                ":h" => Command::Help,
                ":e" => Command::Edit,
                ":w" => Command::SwitchMode(TimeMode::Week(1)),
                ":d" => Command::SwitchMode(TimeMode::Day(1)),

                _ => {
                    if let Some(arg) = input.strip_prefix(":d") {
                        match arg.parse::<u32>() {
                            Ok(n) => Command::SwitchMode(TimeMode::Day(n)),
                            Err(_) => Command::Error("Invalid day number".to_string()),
                        }
                    } else if let Some(arg) = input.strip_prefix(":w") {
                        match arg.parse::<u32>() {
                            Ok(week) => Command::SwitchMode(TimeMode::Week(week)),
                            Err(_) => Command::Error("Invalid week number".to_string()),
                        }
                    } else {
                        Command::Error("Unknown command".to_string())
                    }
                }
            },

            Some(_) => Command::Add(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Command::parse("".to_string()), Command::Nothing);
        assert_eq!(Command::parse(":q".to_string()), Command::Quit);
        assert_eq!(Command::parse(":h".to_string()), Command::Help);
        assert_eq!(Command::parse(":e".to_string()), Command::Edit);
        assert_eq!(
            Command::parse(":w".to_string()),
            Command::SwitchMode(TimeMode::Week(1))
        );
        assert_eq!(
            Command::parse(":w2".to_string()),
            Command::SwitchMode(TimeMode::Week(2))
        );
        assert_eq!(
            Command::parse(":d".to_string()),
            Command::SwitchMode(TimeMode::Day(1))
        );
        assert_eq!(
            Command::parse(":d7".to_string()),
            Command::SwitchMode(TimeMode::Day(7))
        );
        assert_eq!(
            Command::parse("foo".to_string()),
            Command::Add("foo".to_string())
        );
        // unknown command letter
        assert_eq!(
            Command::parse(":x".to_string()),
            Command::Error("Unknown command".to_string())
        );
        // trailing garbage
        assert_eq!(
            Command::parse(":e2".to_string()),
            Command::Error("Unknown command".to_string())
        );
        // invalid day/week args
        assert_eq!(
            Command::parse(":da".to_string()),
            Command::Error("Invalid day number".to_string())
        );
        assert_eq!(
            Command::parse(":w ".to_string()),
            Command::Error("Invalid week number".to_string())
        );
    }
}
