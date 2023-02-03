// Copyright (C) 2022 Martin Pitt <martin@piware.de>
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

extern crate chrono;
extern crate dirs;

use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fmt::Write as _; // import without risk of name clashing
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::PathBuf;

use chrono::{prelude::*, Duration, Local, NaiveDate, NaiveDateTime, Weekday};

/**
 * Single timelog entry
 */

const TIME_FMT: &str = "%Y-%m-%d %H:%M";

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub stop: NaiveDateTime,
    pub task: String,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.stop.format(TIME_FMT), self.task)
    }
}

/**
 * Collection of all entries
 */

#[derive(Default, Debug)]
pub struct Timelog {
    entries: Vec<Entry>,
    pub filename: Option<PathBuf>,
}

impl Timelog {
    pub fn new_from_default_file() -> Timelog {
        Timelog::new_from_file(&Timelog::get_default_file())
    }

    pub fn new_from_file(path: &PathBuf) -> Timelog {
        Timelog {
            entries: Timelog::parse(&Timelog::read(path)),
            filename: Some(path.clone()),
        }
    }

    #[cfg(test)]
    pub fn new_from_string(contents: &str) -> Timelog {
        Timelog {
            entries: Timelog::parse(contents),
            filename: None,
        }
    }

    pub fn get_default_file() -> PathBuf {
        let mut legacy_dir = dirs::home_dir().unwrap();
        legacy_dir.push(".gtimelog");
        let mut log_path = if legacy_dir.is_dir() {
            legacy_dir
        } else {
            let mut data_dir = match env::var_os("XDG_DATA_HOME") {
                Some(val) => PathBuf::from(val.into_string().unwrap()),
                None => dirs::data_dir().unwrap(),
            };
            data_dir.push("gtimelog");
            data_dir
        };
        log_path.push("timelog.txt");
        log_path
    }

    fn read(path: &PathBuf) -> String {
        match File::open(path) {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {:?}", path.display(), e));
                contents
            }

            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    println!("No existing {}, starting new log", path.display());
                    String::new()
                } else {
                    panic!("Could not open {}: {:?}", path.display(), e);
                }
            }
        }
    }

    fn parse(raw: &str) -> Vec<Entry> {
        let mut entries = Vec::new();
        let mut prev: Option<NaiveDateTime> = None;

        for line in raw.lines() {
            if let Some(e) = Timelog::parse_line(line) {
                // require a monotonously increasing file
                if prev.is_some() && e.stop < prev.unwrap() {
                    panic!("line {line} goes back in time");
                }
                prev = Some(e.stop);
                entries.push(e);
            }
        }
        entries
    }

    fn parse_line(line: &str) -> Option<Entry> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        if let Some((time, task)) = line.split_once(": ") {
            if let Ok(dt) = NaiveDateTime::parse_from_str(time, TIME_FMT) {
                Some(Entry {
                    stop: dt,
                    task: task.to_string(),
                })
            } else {
                eprintln!("WARNING: ignoring line with invalid date in timelog: {line}");
                None
            }
        } else {
            eprintln!("WARNING: ignoring invalid line in timelog: {line}");
            None
        }
    }

    fn format_store(&self) -> String {
        let mut output = String::new();
        let mut prev: Option<NaiveDate> = None;

        for entry in &self.entries {
            // leave an empty line between days
            if prev.is_some() && prev.unwrap() != entry.stop.date() {
                output.push('\n');
            }
            prev = Some(entry.stop.date());
            writeln!(output, "{entry}").expect("failed to format entry");
        }

        output
    }

    pub fn save(&self) -> Result<(), io::Error> {
        assert!(self.filename.is_some());
        let filename = self.filename.as_ref().unwrap();
        if let Some(parent) = filename.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = File::create(filename)?;
        write!(f, "{}", self.format_store())?;
        Ok(())
    }

    #[cfg(test)]
    pub fn get_all(&self) -> impl Iterator<Item = &Entry> {
        return self.entries.iter();
    }

    pub fn get_time_range(&self, begin: NaiveDateTime, end: NaiveDateTime) -> &[Entry] {
        let first = self
            .entries
            .iter()
            .position(move |e| e.stop >= begin)
            .unwrap_or(self.entries.len());
        let last = self
            .entries
            .iter()
            .position(move |e| e.stop > end)
            .unwrap_or(self.entries.len());

        &self.entries[first..last]
    }

    pub fn get_day(&self, day: &NaiveDate) -> &[Entry] {
        self.get_time_range(
            day.and_hms_opt(0, 0, 0).unwrap(),
            day.and_hms_opt(23, 59, 59).unwrap(),
        )
    }

    pub fn get_today(&self) -> &[Entry] {
        self.get_day(&Local::now().date_naive())
    }

    pub fn get_today_as_string(&self) -> String {
        Local::now().format("%A, %F (week %W)").to_string()
    }

    pub fn get_week(&self, day: &NaiveDate) -> &[Entry] {
        let week = day.iso_week().week();
        let begin = NaiveDate::from_isoywd_opt(day.year(), week, Weekday::Mon)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let end = NaiveDate::from_isoywd_opt(day.year(), week + 1, Weekday::Mon)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        self.get_time_range(begin, end)
    }

    pub fn get_this_week(&self) -> &[Entry] {
        self.get_week(&Local::now().date_naive())
    }

    pub fn get_this_week_as_string(&self) -> String {
        let now_local = Local::now();
        let week_begin = now_local
            .checked_sub_signed(Duration::days(
                now_local.weekday().num_days_from_monday().into(),
            ))
            .unwrap();
        let week_end = week_begin.checked_add_signed(Duration::days(6)).unwrap();
        let this_week = if week_begin.month() == now_local.month() {
            format!(
                "{} {}-{}",
                now_local.format("%B"),
                week_begin.day(),
                week_end.day()
            )
        } else {
            format!("{}-{}", week_begin.format("%B %e"), week_end.day())
        };
        format!("{} ({})", now_local.format("%Y, week %W"), this_week)
    }

    pub fn get_history(entries: &[Entry]) -> Vec<&String> {
        let mut seen = HashSet::new();
        entries
            .iter()
            .map(|e| &e.task)
            .filter(|&t| {
                if seen.contains(&t) {
                    false
                } else {
                    seen.insert(t);
                    true
                }
            })
            .collect()
    }

    pub fn add(&mut self, task: String) {
        let now = Local::now();
        let naivenow = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .unwrap()
            .and_hms_opt(now.hour(), now.minute(), now.second())
            .unwrap();
        self.entries.push(Entry {
            task,
            stop: naivenow,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use pretty_assertions::assert_eq;

    const TWO_DAYS: &'static str = "
2022-06-09 06:02: arrived
2022-06-09 06:27: email
2022-06-09 06:32: **tea
2022-06-09 12:00: work

2022-06-10 07:00: arrived
2022-06-10 12:05: rtimelog: code
2022-06-10 12:30: **lunch
2022-06-10 14:00: rtimelog: code
2022-06-10 15:00: bug triage
2022-06-10 16:00: customer joe: support
";

    const TWO_WEEKS: &'static str = "
2022-06-01 06:00: arrived
2022-06-01 07:00: workw1
2022-06-01 07:10: ** tea

2022-06-03 06:00: arrived
2022-06-03 07:00: workw1
2022-06-03 07:10: ** tea

2022-06-08 06:00: arrived
2022-06-08 07:00: workw2
2022-06-08 07:10: ** tea

2022-06-09 06:00: arrived
2022-06-09 07:00: workw2

2022-06-10 06:00: arrived
2022-06-10 07:00: workw2
";

    #[test]
    fn test_parse_line_valid() {
        let e1 = Timelog::parse_line("2022-05-31 13:59: email").unwrap();
        assert_eq!(e1.task, "email");
        assert_eq!(e1.stop.format(TIME_FMT).to_string(), "2022-05-31 13:59");

        let e2 = Timelog::parse_line("2022-05-31 14:07: read docs").unwrap();
        assert_eq!(e2.task, "read docs");
        assert_eq!(e2.stop.format(TIME_FMT).to_string(), "2022-05-31 14:07");

        assert_eq!(e2.stop.signed_duration_since(e1.stop), Duration::minutes(8));
    }

    #[test]
    fn test_parse_line_invalid() {
        assert_eq!(Timelog::parse_line(""), None);
        assert_eq!(Timelog::parse_line("  "), None);
        assert_eq!(Timelog::parse_line("a"), None);
        // no ': -'
        assert_eq!(Timelog::parse_line("2022-05-31 13:59 email"), None);
        // invalid time
        assert_eq!(Timelog::parse_line("2022-05-31 25:61: email"), None);
        // invalid date
        assert_eq!(Timelog::parse_line("2022-13-32 13:59: email"), None);
    }

    #[test]
    fn test_parse_valid() {
        let entries = Timelog::parse("");
        assert_eq!(entries.len(), 0);

        let entries = Timelog::parse(TWO_DAYS);
        assert_eq!(entries.len(), 10);
        assert_eq!(&format!("{}", entries[0]), "2022-06-09 06:02: arrived");
        assert_eq!(
            &format!("{}", entries[9]),
            "2022-06-10 16:00: customer joe: support"
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_out_of_order() {
        Timelog::parse(
            "
2022-06-09 06:02: arrived
2022-06-09 06:10: ** tea
2022-06-08 07:32: huh, previous day
",
        );
    }

    #[test]
    fn test_constructor() {
        let tl = Timelog::new_from_string("");
        assert_eq!(tl.get_all().next(), None);

        let tl = Timelog::new_from_file(&PathBuf::from("/nonexisting"));
        assert_eq!(tl.get_all().next(), None);

        let tl = Timelog::new_from_string(TWO_DAYS);
        let mut entries = tl.get_all();
        assert_eq!(
            &format!("{}", entries.next().unwrap()),
            "2022-06-09 06:02: arrived"
        );
        let mut entries = entries.skip(8);
        assert_eq!(
            &format!("{}", entries.next().unwrap()),
            "2022-06-10 16:00: customer joe: support"
        );
        assert_eq!(entries.next(), None);
    }

    #[test]
    fn test_get_day() {
        let tl = Timelog::new_from_string("");
        assert_eq!(
            tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 8).unwrap()),
            &[]
        );

        let tl = Timelog::new_from_string(TWO_DAYS);
        assert_eq!(
            tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 8).unwrap()),
            &[]
        );

        let entries = tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 9).unwrap());
        assert_eq!(entries.len(), 4);
        assert_eq!(&format!("{}", entries[0]), "2022-06-09 06:02: arrived");
        assert_eq!(&format!("{}", entries[3]), "2022-06-09 12:00: work");

        let entries = tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 10).unwrap());
        assert_eq!(entries.len(), 6);
        assert_eq!(&format!("{}", entries[0]), "2022-06-10 07:00: arrived");
        assert_eq!(
            &format!("{}", entries[5]),
            "2022-06-10 16:00: customer joe: support"
        );
    }

    #[test]
    fn test_get_week() {
        let tl = Timelog::new_from_string("");
        assert_eq!(
            tl.get_week(&NaiveDate::from_ymd_opt(2022, 6, 2).unwrap()),
            &[]
        );

        let tl = Timelog::new_from_string(TWO_WEEKS);
        // select Wed, data has Tue and Thu
        let entries = tl.get_week(&NaiveDate::from_ymd_opt(2022, 6, 2).unwrap());
        assert_eq!(entries.len(), 6);
        assert_eq!(&format!("{}", entries[0]), "2022-06-01 06:00: arrived");
        assert_eq!(&format!("{}", entries[5]), "2022-06-03 07:10: ** tea");

        // select Tue, data has Wed to Fri
        let entries = tl.get_week(&NaiveDate::from_ymd_opt(2022, 6, 7).unwrap());
        assert_eq!(entries.len(), 7);
        assert_eq!(&format!("{}", entries[0]), "2022-06-08 06:00: arrived");
        assert_eq!(&format!("{}", entries[6]), "2022-06-10 07:00: workw2");
    }

    #[test]
    fn test_format_store() {
        let tl = Timelog::new_from_string(TWO_DAYS);
        // simple roundtrip; but our constant starts with an empty line
        assert_eq!(tl.format_store(), TWO_DAYS.trim_start());
    }

    #[test]
    fn test_get_history() {
        let tl = Timelog::new_from_string("");
        assert!(
            Timelog::get_history(tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 8).unwrap()))
                .is_empty()
        );

        let tl = Timelog::new_from_string(TWO_DAYS);
        let entries = tl.get_day(&NaiveDate::from_ymd_opt(2022, 6, 10).unwrap());
        assert_eq!(
            Timelog::get_history(entries),
            // no duplicate "rtimelog: code"
            vec![
                "arrived",
                "rtimelog: code",
                "**lunch",
                "bug triage",
                "customer joe: support"
            ]
        );
    }

    #[test]
    fn test_add() {
        let mut tl = Timelog::new_from_string("");
        tl.add("think hard".to_string());
        assert_eq!(tl.entries.len(), 1);
        assert_eq!(tl.entries[0].task, "think hard");
    }
}
