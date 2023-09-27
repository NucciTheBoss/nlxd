// Copyright 2023 Jason C. Nucciarone
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 3 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

enum EventType {
    All,
    Operation,
    Logging,
    Lifecycle,
}

struct Cert {
    cert: String,
    key: String,
}

struct Client {
    endpoint: String,
    version: String,
    verify: bool, // Could also potentially be a string - need to figure out how to do that
    timeout: f32, // Could also be a tuple.
    project: String,
    session: Session, // Should be a custom data type possibly
}

type Session = (); // TODO: implement me!

impl Client {
    pub fn connect(&mut self) {
        // TODO: Connect to the LXD REST API.
        //  Try to just pull up some basic info after connecting to the REST API.
    }
}
