// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of screenruster.
//
// screenruster is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// screenruster is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with screenruster.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

use screen::json;

pub struct Config {
	path: PathBuf,

	using: Vec<String>,
	hacks: HashMap<String, json::JsonValue>,
	empty: json::JsonValue,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	Random,
	One,
}

impl Config {
	pub fn load(config: json::JsonValue) -> io::Result<Config> {
		let     home  = try!(env::home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "home not found")));
		let     file  = try!(File::open(home.join(".xscreensaver")));
		let mut lines = BufReader::new(file).lines();

		let mut using    = Vec::new();
		let mut hacks    = HashMap::new();
		let mut programs = String::new();
		let mut mode     = Mode::Random;
		let mut selected = 0;

		while let Some(Ok(line)) = lines.next() {
			if line.starts_with("mode:") {
				mode = match line.splitn(2, ':').collect::<Vec<&str>>().pop().unwrap().trim() {
					"one"    => Mode::One,
					"random" => Mode::Random,

					name => {
						error!("unknown mode: {}", name);
						unreachable!();
					}
				}
			}

			if line.starts_with("selected:") {
				selected = line.splitn(2, ':').collect::<Vec<&str>>().pop().unwrap().trim().parse().unwrap()
			}

			if line.starts_with("programs:") {
				while let Some(Ok(line)) = lines.next() {
					if !line.ends_with('\\') {
						break;
					}

					let mut line = line.as_ref(): &str;

					if line.starts_with('-') {
						programs.push('-');
						line = line[1..].into();
					}

					programs.push_str(line.replace("GL:", "").replace("\\n", "").replace("\\", "").trim());

					if line.contains("\\n") {
						programs.push('\n');
					}
				}
			}
		}

		if let json::JsonValue::Array(ref array) = config["use"] {
			for item in array {
				if let Some(item) = item.as_str() {
					using.push(item.into());
					hacks.insert(item.into(), config[item].clone());
				}
			}
		}
		else {
			for (index, program) in programs.split('\n').enumerate() {
				if program.starts_with('-') {
					continue;
				}

				let mut name = program.split(' ').collect::<Vec<&str>>();
				let mut args = name.split_off(1).into_iter().peekable();
				let mut config = object!{};

				while let Some(mut item) = args.next() {
					item = &item[1..];

					match args.peek() {
						Some(&arg) if !arg.starts_with('-') => {
							config[item] = arg.into();
							args.next();
						}

						_ if item.starts_with("-no-") => {
							config[item] = false.into();
						}

						_ => {
							config[item] = true.into();
						}
					}
				}

				if mode == Mode::Random || (mode == Mode::One && index + 1 == selected) {
					using.push(name[0].into());
					hacks.insert(name[0].into(), config);
				}
			}
		}

		Ok(Config {
			path: config["path"].as_str().unwrap_or("/usr/libexec/xscreensaver").into(),

			using: using,
			hacks: hacks,
			empty: json::JsonValue::new_object(),
		})
	}

	pub fn using(&self) -> Vec<&str> {
		self.using.iter().map(|v| v.as_ref()).collect()
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn get<T: AsRef<str>>(&self, name: T) -> &json::JsonValue {
		match self.hacks.get(name.as_ref()) {
			Some(value) if value.is_null() => {
				&self.empty
			}

			Some(value) => {
				value
			}

			None => {
				&self.empty
			}
		}
	}
}
