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

use serde::Deserialize;

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

// TODO: deny unknown fields once Instance type is complete
// #[serde(deny_unknown_fields)]
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Instance {
    name: String,
    project: String,
    stateful: bool,
    status: String,   // TODO: could be an enum
    status_code: u32, // TODO: maybe also an enum?
    location: String,
    profiles: Vec<String>,
    #[serde(rename = "type")]
    instance_type: InstanceType,
    last_used_at: String, // TODO: datetime type
    expanded_devices: HashMap<String, Device>,
    // TODO: the remaining fields
}
