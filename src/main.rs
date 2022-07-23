use xcb::{
	Connection,
	Event,
	xfixes,
	x
};
use kira::{
	manager::{
		AudioManager,
		AudioManagerSettings,
		backend::cpal::CpalBackend
	},
	sound::static_sound::{
		StaticSoundData,
		StaticSoundSettings
	}
};

fn intern_atom<'a>(conn: &Connection, name: &'a [u8]) -> Result<x::InternAtomReply, xcb::Error> {
	let cookie = conn.send_request(&x::InternAtom {only_if_exists: false, name});
	conn.wait_for_reply(cookie)
}

macro_rules! intern_atom {
	($c:expr, $n:expr) => {
		intern_atom($c, $n).unwrap().atom()
	}
}

fn main() {
	print!("
Clip notify - notification on clipboard change.
Copyright (C) 2022  Niki4tap
    
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
    
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
    
You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
");

	let (conn, screen_num) = Connection::connect_with_extensions(None, &[xcb::Extension::XFixes], &[]).unwrap();
	let setup = conn.get_setup();
	let screen = setup.roots().nth(screen_num as usize).unwrap();
	
	conn.wait_for_reply(conn.send_request(&xfixes::QueryVersion {
		client_major_version: xfixes::MAJOR_VERSION,
		client_minor_version: xfixes::MINOR_VERSION
	})).unwrap();

	let req = xcb::xfixes::SelectSelectionInput {
		window: screen.root(),
		selection: intern_atom!(&conn, b"CLIPBOARD"),
		event_mask: xfixes::SelectionEventMask::SET_SELECTION_OWNER
	};

	let cookie = conn.send_request_checked(&req);
	conn.check_request(cookie).unwrap();

	let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
	let sound = StaticSoundData::from_file("sound.mp3", StaticSoundSettings::default()).unwrap();

	loop {
		match conn.wait_for_event().unwrap() {
			Event::XFixes(e) => match e {
				xfixes::Event::SelectionNotify(_event) => {
					manager.play(sound.clone()).unwrap();
				},
				_ => {}
			},
			_ => {}
		}
	}
}