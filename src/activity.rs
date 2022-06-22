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

use std::fmt;

use chrono::{prelude::*, Duration, NaiveDateTime};

use crate::store::Entry;

/**
 * Activity: Duration of all Entry's with the same task
 */
pub struct Activity {
    name: String,
    duration: Duration,
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>2} h {:>2} min: {}",
            self.duration.num_hours(),
            self.duration.num_minutes() % 60,
            self.name
        )
    }
}

/**
 * Activities: Collection of Activity with total durations
 */
pub struct Activities {
    activities: Vec<Activity>,
    total_work: Duration,
    total_slack: Duration,
}

impl Activities {
    pub fn new_from_entries(entries: &[Entry]) -> Activities {
        // don't use a hashmap here, we do want to keep this sorted by "first occurrence of task"
        let mut activities = Vec::new();
        let mut total_work = Duration::minutes(0);
        let mut total_slack = Duration::minutes(0);
        let mut prev_stop: Option<NaiveDateTime> = None;

        for entry in entries {
            // first entry's task is ignored, it just provides the start time
            if prev_stop.is_none() {
                prev_stop = Some(entry.stop);
                continue;
            }
            // likewise, first entry of every day gets ignored
            if prev_stop.unwrap().day() != entry.stop.day() {
                prev_stop = Some(entry.stop);
                continue;
            }
            let duration = entry.stop.signed_duration_since(prev_stop.unwrap());
            if entry.task.starts_with("**") {
                total_slack = total_slack + duration;
            } else {
                total_work = total_work + duration;
            }

            // meh quadratic loop, but not important
            match activities
                .iter_mut()
                .find(|a: &&mut Activity| a.name == entry.task)
            {
                Some(a) => a.duration = a.duration + duration,
                None => activities.push(Activity {
                    name: entry.task.to_string(),
                    duration,
                }),
            }

            prev_stop = Some(entry.stop);
        }

        Activities {
            activities,
            total_work,
            total_slack,
        }
    }
}

impl fmt::Display for Activities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for a in &self.activities {
            writeln!(f, "{}", a)?;
        }
        writeln!(f, "-------")?;
        writeln!(
            f,
            "Total work done: {} h {} min",
            self.total_work.num_hours(),
            self.total_work.num_minutes() % 60
        )?;
        writeln!(
            f,
            "Total slacking: {} h {} min",
            self.total_slack.num_hours(),
            self.total_slack.num_minutes() % 60
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Timelog;
    use chrono::NaiveDate;

    #[test]
    fn test_activity_display() {
        assert_eq!(
            &format!(
                "{}",
                Activity {
                    name: "code this".to_string(),
                    duration: Duration::minutes(3)
                }
            ),
            " 0 h  3 min: code this"
        );
        assert_eq!(
            &format!(
                "{}",
                Activity {
                    name: "code this".to_string(),
                    duration: Duration::minutes(59)
                }
            ),
            " 0 h 59 min: code this"
        );
        assert_eq!(
            &format!(
                "{}",
                Activity {
                    name: "code this".to_string(),
                    duration: Duration::minutes(60)
                }
            ),
            " 1 h  0 min: code this"
        );
        assert_eq!(
            &format!(
                "{}",
                Activity {
                    name: "code this".to_string(),
                    duration: Duration::minutes(23 * 60 + 1)
                }
            ),
            "23 h  1 min: code this"
        );
    }

    #[test]
    fn test_activities_empty() {
        let a = Activities::new_from_entries(&[]);
        assert_eq!(a.activities.len(), 0);
        assert_eq!(a.total_work, Duration::minutes(0));
        assert_eq!(a.total_slack, Duration::minutes(0));
    }

    #[test]
    fn test_activities_daily() {
        let tl = Timelog::new_from_string(
            "
2022-06-10 07:00: arrived
2022-06-10 08:45: gtimelog: code
2022-06-10 09:00: ** tea
2022-06-10 12:05: gtimelog: code
2022-06-10 12:35: customer joe: inquiry
2022-06-10 13:15: ** lunch
2022-06-10 14:00: code
2022-06-10 15:00: bug triage
2022-06-10 15:10: ** tea
2022-06-10 16:00: customer joe: support
",
        );

        let a = Activities::new_from_entries(tl.get_day(&NaiveDate::from_ymd(2022, 6, 10)));
        assert_eq!(a.total_work, Duration::minutes(475));
        assert_eq!(a.total_slack, Duration::minutes(65));
        assert_eq!(a.activities.len(), 7);
        assert_eq!(a.activities[0].name, "gtimelog: code");
        // first block 1:45, second block 3:05
        assert_eq!(
            a.activities[0].duration,
            Duration::hours(4) + Duration::minutes(50)
        );

        assert_eq!(
            format!("{}", a),
            " 4 h 50 min: gtimelog: code
 0 h 25 min: ** tea
 0 h 30 min: customer joe: inquiry
 0 h 40 min: ** lunch
 0 h 45 min: code
 1 h  0 min: bug triage
 0 h 50 min: customer joe: support
-------
Total work done: 7 h 55 min
Total slacking: 1 h 5 min\n"
        )
    }

    #[test]
    fn test_activities_weekly() {
        let tl = Timelog::new_from_string(
            "
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
2022-06-10 07:10: ** tea
",
        );

        let a = Activities::new_from_entries(tl.get_week(&NaiveDate::from_ymd(2022, 6, 7)));
        assert_eq!(a.total_work, Duration::hours(3));
        assert_eq!(a.total_slack, Duration::minutes(20));
        assert_eq!(a.activities.len(), 2);
        assert_eq!(a.activities[0].name, "workw2");
        assert_eq!(a.activities[0].duration, Duration::hours(3));
        assert_eq!(a.activities[1].name, "** tea");
        assert_eq!(a.activities[1].duration, Duration::minutes(20));

        assert_eq!(
            format!("{}", a),
            " 3 h  0 min: workw2
 0 h 20 min: ** tea
-------
Total work done: 3 h 0 min
Total slacking: 0 h 20 min
"
        );
    }
}
