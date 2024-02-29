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

use isahc::{config::Dialer, prelude::*, HttpClient, Request};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::{instance::Instance, Result};

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
pub enum Endpoint {
    Http(String),
    UnixSocket(String),
}

impl Default for Endpoint {
    fn default() -> Self {
        // TODO: implement default for endpoint, autodetecting the local lxd socket
        Endpoint::UnixSocket("/var/snap/lxd/common/lxd/unix.socket".to_owned())
    }
}

// TODO: remove the api version config?
#[derive(Default)]
pub enum LxdAPIVersion {
    #[default]
    V1_0,
}

impl LxdAPIVersion {
    fn to_url_segment(&self) -> &'static str {
        match self {
            LxdAPIVersion::V1_0 => "/1.0",
        }
    }
}

#[allow(unused)]
pub struct Timeout {
    server_timeout_seconds: u32,
    connection_timeout_seconds: u32,
}

impl Timeout {
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

impl Default for Timeout {
    fn default() -> Self {
        Self {
            server_timeout_seconds: 60,
            connection_timeout_seconds: 60,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ResponseType {
    Sync,
    Async,
    Error,
}

// TODO: turn this into an enum based on the three standard return types: https://documentation.ubuntu.com/lxd/en/latest/rest-api/#return-values
#[derive(Debug, Deserialize)]
struct Response<T> {
    metadata: T,
    #[allow(unused)]
    status: String,
    #[allow(unused)]
    #[serde(rename = "type")]
    response_type: ResponseType,
    #[allow(unused)]
    // TODO: enum for http codes - use serde_repr
    status_code: u32,
}

pub struct ClientConfig {
    pub endpoint: Endpoint,
    pub version: LxdAPIVersion,
    pub verify: bool,
    pub timeout: Timeout,
    pub project: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            endpoint: Endpoint::default(),
            version: LxdAPIVersion::default(),
            verify: true,
            timeout: Timeout::default(),
            project: "default".to_owned(),
        }
    }
}

pub struct Client {
    #[allow(unused)]
    endpoint: Endpoint, // address to lxd server e.g. http:./// or unix socket
    #[allow(unused)]
    version: LxdAPIVersion,
    #[allow(unused)]
    verify: bool, // Could also potentially be a string - need to figure out how to do that
    #[allow(unused)]
    timeout: Timeout,
    #[allow(unused)]
    project: String,
    #[allow(unused)]
    client: HttpClient,
}

impl Client {
    #[allow(unused)]
    pub fn connect(&mut self) -> Result<()> {
        // TODO: Connect to the LXD REST API.
        //  Try to just pull up some basic info after connecting to the REST API.
        todo!()
    }

    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            endpoint: config.endpoint,
            version: config.version,
            timeout: config.timeout,
            project: config.project,
            verify: config.verify,
            client: HttpClient::new()?,
        })
    }

    // `path` is expected to begin with a slash
    fn get<T>(&self, path: &str) -> Result<Response<T>>
    where
        T: DeserializeOwned,
    {
        let (request_builder, host) = match &self.endpoint {
            Endpoint::Http(host) => {
                // TODO: https requests will need some kind of authentication
                (Request::builder(), host.as_str())
            }
            Endpoint::UnixSocket(host) => {
                let socket = Dialer::unix_socket(host);
                // host is arbitrarily set to 'lxd' - ignored, but required as part of the http spec
                (Request::builder().dial(socket), "http://lxd")
            }
        };

        let uri = format!("{}{}", host, path);
        let mut response = request_builder
            .uri(uri)
            .method("GET")
            .body(())
            .unwrap()
            .send()
            .unwrap();
        Ok(response.json()?)
    }

    /// Get a list of instance names
    pub fn instances(&self) -> Result<Vec<String>> {
        Ok(self
            .get(&format!("{}/instances", self.version.to_url_segment()))?
            .metadata)
    }

    /// Get an instance's by instance name.
    pub fn get_instance(&self, name: &str) -> Result<Instance> {
        Ok(self.get(name)?.metadata)
    }
}
