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

use crate::Result;

#[allow(unused)]
enum EventType {
    All,
    Operation,
    Logging,
    Lifecycle,
}

#[allow(unused)]
struct Cert {
    cert: String,
    key: String,
}

#[allow(unused)]
enum Endpoint {
    Http(String),
    UnixSocket(String),
}

#[allow(unused)]
enum LxdAPIVersion {
    V1_0,
    // TODO: complete me!
}

#[allow(unused)]
struct Timeouts {
    server_timeout_seconds: u32,
    connection_timeout_seconds: u32,
}

impl Timeouts {
    #[allow(unused)]
    pub fn new(server_timeout_seconds: u32, connection_timeout_seconds: u32) -> Self {
        Self {
            server_timeout_seconds,
            connection_timeout_seconds,
        }
    }

    #[allow(unused)]
    pub fn from_seconds(timeout: u32) -> Self {
        Self::new(timeout, timeout)
    }
}

#[allow(unused)]
struct ClientConfig {
    pub endpoint: Endpoint, // address to lxd server e.g. http:./// or unix socket
    pub version: LxdAPIVersion,
    pub verify: bool, // Could also potentially be a string - need to figure out how to do that
    pub timeout_seconds: Option<Timeouts>, // Could also be a tuple.
    pub project: Option<String>,
}

struct Client {
    #[allow(unused)]
    endpoint: Endpoint, // address to lxd server e.g. http:./// or unix socket
    #[allow(unused)]
    version: LxdAPIVersion,
    #[allow(unused)]
    verify: bool, // Could also potentially be a string - need to figure out how to do that
    #[allow(unused)]
    timeout_seconds: Option<Timeouts>, // Could also be a tuple.
    #[allow(unused)]
    project: String,
}

impl Client {
    #[allow(unused)]
    pub fn connect(&mut self) -> Result<()> {
        // TODO: Connect to the LXD REST API.
        //  Try to just pull up some basic info after connecting to the REST API.
        todo!()
    }
}

// example function demonstrating using isahc http library with interchangable unix domain socket
// or standard http/https host.
use isahc::{config::Dialer, prelude::*, Request};
pub fn get_server_info(host: &str) -> String {
    let path = "/1.0";

    let (request, host) = if let Some(prefix) = host.strip_prefix("unix:") {
        let socket = Dialer::unix_socket(prefix.to_string());
        // host is arbitrarily set to 'lxd' - ignored, but required as part of the http spec
        (Request::builder().dial(socket), "http://lxd")
    } else {
        (Request::builder(), host)
    };

    let uri = format!("{}{}", host, path);
    let mut response = request
        .uri(uri)
        .method("GET")
        .body(())
        .unwrap()
        .send()
        .unwrap();
    response.text().unwrap()
}
