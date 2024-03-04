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

//! Interact with LXD instances.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstanceType {
    Container,
    VirtualMachine,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Device {
    Disk {
        path: String,
        // TODO: check if source and pool are mutually exclusive - if so, use an enum
        source: Option<String>,
        pool: Option<String>,
    },
    Nic {
        name: String,
        network: String,
    },
}

/// Code from the REST API for instance status.
/// Codes documented at https://documentation.ubuntu.com/lxd/en/latest/rest-api/#status-codes
#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
pub enum InstanceStatus {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Canceling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    Error = 112,
    Ready = 113,
    Success = 200,
    Failure = 400,
    Canceled = 401,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Instance {
    architecture: String,
    // TODO: determine if config is completely described.  If so, we can potentially use a struct.
    config: HashMap<String, String>,
    created_at: DateTime<Utc>,
    description: String,
    devices: HashMap<String, Device>,
    ephemeral: bool,
    expanded_config: HashMap<String, String>,
    expanded_devices: HashMap<String, Device>,
    last_used_at: DateTime<Utc>,
    location: String,
    name: String,
    profiles: Vec<String>,
    project: String,
    stateful: bool,
    #[serde(rename = "status_code")]
    status: InstanceStatus,
    #[serde(rename = "type")]
    instance_type: InstanceType,
}
