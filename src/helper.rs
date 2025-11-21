// Copyright (C) 2025 Guilhem Bonnefille <guilhem.bonnefille@gmail.com>
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

use std::collections::HashSet;
use std::collections::VecDeque;

use crate::store::Entry;
use crate::store::Timelog;

use rustyline::completion::Completer;
use rustyline::Context;
use rustyline::Result;
use rustyline::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Hinter, Validator, Highlighter, Default)]
pub struct TimelogHelper {
    entries: VecDeque<String>,
}

impl TimelogHelper {
    pub fn add(&mut self, entry: String) {
        if !self.entries.contains(&entry) {
            self.entries.push_back(entry);
        }
    }
}

impl From<&Timelog> for TimelogHelper {
    fn from(timelog: &Timelog) -> Self {
        let entries = timelog.get_all();
        // Convert entries to VecDeque<String>
        let entries: Vec<String> = entries
            .map(|entry: &Entry| entry.task.clone())
            .collect::<HashSet<_>>() // deduplicate
            .into_iter()
            .collect();

        TimelogHelper {
            entries: VecDeque::from(entries),
        }
    }
}

impl Completer for TimelogHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        if line.is_empty() || pos < line.len() {
            return Ok((pos, Vec::new()));
        }
        let _ = (ctx, pos);

        let subproject = line.contains(":");
        let task = line.contains("--");
        let sep = if !subproject {
            Some(":")
        } else if !task {
            Some("--")
        } else {
            None
        };
        let candidates = self
            .entries
            .iter()
            .filter_map(|entry_str| {
                if entry_str.starts_with(line) {
                    match sep {
                        Some(sep) => {
                            // If the entry contains the separator, we split it
                            // to remove the sub-project or task information
                            Some(
                                entry_str
                                    .split(sep)
                                    .next()
                                    .unwrap_or(entry_str)
                                    .trim()
                                    .to_string(),
                            )
                        }
                        None => {
                            // If no separator is found, we keep the whole entry
                            Some(entry_str.to_string())
                        }
                    }
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>() // deduplicate
            .into_iter()
            .collect();
        Ok((0, candidates))
    }
}
