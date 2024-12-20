// Copyright (C) 2024 Guilhem Bonnefille <guilhem.bonnefille@gmail.com>
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
use std::error::Error;
use std::io;

use clap::{Parser, Subcommand};

use common::store::Timelog;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add { task: String}
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
  
    match args.cmd {
        Commands::Add{task} => {
            add_entry(task)?;
        }
    }
    Ok(())
}

fn add_entry(task: String) -> Result<(), io::Error> {
    let mut timelog = Timelog::new_from_default_file();
    timelog.add(task);
    timelog.save()
}