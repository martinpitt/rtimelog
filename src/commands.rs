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
    Day,
    Week,
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
        match input.as_str() {
            "" => Command::Nothing,
            ":q" => Command::Quit,
            ":h" => Command::Help,
            ":e" => Command::Edit,
            ":w" => Command::SwitchMode(TimeMode::Week),
            ":d" => Command::SwitchMode(TimeMode::Day),
            _ => {
                if input.starts_with(':') {
                    Command::Error(format!("Unknown command: {}", input))
                } else {
                    Command::Add(input)
                }
            }
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
            Command::SwitchMode(TimeMode::Week)
        );
        assert_eq!(
            Command::parse(":d".to_string()),
            Command::SwitchMode(TimeMode::Day)
        );
        assert_eq!(
            Command::parse("foo".to_string()),
            Command::Add("foo".to_string())
        );
        // unknown command letter
        assert_eq!(
            Command::parse(":x".to_string()),
            Command::Error("Unknown command: :x".to_string())
        );
        // trailing garbage
        assert_eq!(
            Command::parse(":e2".to_string()),
            Command::Error("Unknown command: :e2".to_string())
        );
    }
}
