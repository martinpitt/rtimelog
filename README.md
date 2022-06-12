rtimelog
========

rtimelog is a simple console/text based time tracker. It is inspired by
[gtimelog](https://gtimelog.org/) and uses the same data format and data file
(`~/.gtimelog/timelog.txt`), so you can use gtimelog and rtimelog in parallel.

Hoever, it has far fewer features. This was a toy project for learning Rust,
and I don't use many gtimelog features myself. If you need one, please feel
free to send a PR, or file an issue.

```
Work done today:
10 h 45 min: day of learning: time logger in Rust
 3 h 44 min: **
 0 h 18 min: team meeting
-------
Total work done: 11 h 3 min
Total slacking: 3 h 44 min


0h 23 min since last entry; command (:h for help) or entry
> :h

:w - switch to weekly mode
:d - switch to daily mode
:q - quit
:h - show this help
:e - open timelog.txt in $EDITOR

Any other input is the description of a task that you just finished.

0h 24 min since last entry; command (:h for help) or entry
>
```

Operation
---------
This is an interactive program. On startup, it shows the work done so far
today, grouped by entries with the same name, and the per-activity and total
time.

Start the day with some first entry (like "arrived" or "start"). The text will
be ignored, this is just to record the time. Everytime you complete something
or switch activities, type its description.

If you do something non-work related, start the description with `**`, then it
will be accounted as "slack time". You can be specific like `** lunch`, or just
have a single "unnamed" `**` slack activity, depending on whether you care
about tracking individual slack activities.

You can switch between per-day and per-week mode with `:d` and `:w`
respectively.

Type `:q` to end the program.

Whenever you add an entry, it will be immediately saved to
~/.gtimelog/timelog.txt. It's possible to manually edit the file (directly or
wiht the `:e` command), just be cautious to not break the format.

Installation
------------
The [releases page](https://github.com/martinpitt/rtimelog/releases) has
automatically built binaries for Linux, Windows, and MacOS. Download the
archive for your operating system, unpack it, and run `rtimelog`. You can copy
that to `/usr/local/bin/` or `~/.bin/` of course.

Building
--------

This project uses standard [cargo](https://doc.rust-lang.org/cargo/). Install
the Rust toolchain from your distro or with
[rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html), then
you can run it with

    cargo run --release

Run the unit tests with

    cargo build

![tests](https://github.com/martinpitt/rtimelog/actions/workflows/tests.yml/badge.svg)
