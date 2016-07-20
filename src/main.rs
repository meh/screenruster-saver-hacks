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

#![feature(type_ascription)]

#[macro_use]
extern crate screenruster_saver as screen;
use screen::{json, Request, Response};

extern crate rand;
use rand::Rng;

use std::process::Command;

mod config;
pub use config::Config;

fn main() {
	let channel = screen::init().unwrap();
	let config  = if let Request::Config(config) = channel.recv().unwrap() {
		Config::load(config).unwrap()
	}
	else {
		panic!("protocol mismatch");
	};

	let (display, window) = if let Request::Target { display, window, .. } = channel.recv().unwrap() {
		(display, window)
	}
	else {
		panic!("protocol mismatch");
	};

	let     hack    = config.using()[rand::thread_rng().gen_range(0, config.using().len())];
	let mut command = Command::new(format!("{}/{}", config.path().to_str().unwrap(), hack));
	command.env("DISPLAY", display);
	command.arg("-window-id")
	       .arg(format!("{}", window));
	configure(&mut command, config.get(hack));

	channel.send(Response::Initialized).unwrap();

	let mut child = None;
	while let Ok(message) = channel.recv() {
		match message {
			Request::Start => {
				child = Some(command.spawn().unwrap());
				channel.send(Response::Started).unwrap();
			}

			Request::Stop => {
				child.as_mut().unwrap().kill().unwrap();
				child.as_mut().unwrap().wait().unwrap();

				break;
			}

			_ => ()
		}
	}

	channel.send(Response::Stopped).unwrap();
}

fn configure<'a>(command: &'a mut Command, config: &json::JsonValue) -> &'a mut Command {
	for (key, value) in config.entries() {
		if key == "window-id" ||
		   key == "window" ||
		   key == "root"
		{
			continue;
		}

		match *value {
			json::JsonValue::Boolean(true) => {
				command.arg(format!("-{}", key));
			}

			json::JsonValue::Boolean(false) => {
				command.arg(format!("-no-{}", key));
			}

			json::JsonValue::String(ref string) => {
				command.arg(format!("-{}", key)).arg(string);
			}

			json::JsonValue::Number(number) => {
				command.arg(format!("-{}", key)).arg(format!("{}", number));
			}

			_ => ()
		}
	}

	command
}
