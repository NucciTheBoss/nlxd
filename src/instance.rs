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
use std::default::Default;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ResourceStatus;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum VirtualisationType {
    Container,
    VirtualMachine,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    architecture: String,
    // TODO: determine if config is completely described.  If so, we can potentially use a struct.
    config: HashMap<String, String>,
    created_at: DateTime<Utc>,
    description: String,
    devices: HashMap<String, Device>,
    /// Whether the instance is ephemeral (deleted on shutdown)
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
    status: ResourceStatus,
    #[serde(rename = "type")]
    virtualisation_type: VirtualisationType,
}

/// Represents a new instance that has not been created yet.
/// Matches
/// [InstancePost](https://documentation.ubuntu.com/lxd/en/latest/api/#/instances/instances_post)
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NewInstance {
    /// only used for source: None or Migration
    // TODO: make enum from https://github.com/canonical/lxd/blob/854006f80fb9a6d5d5c42900a4352f3d0381bc13/shared/osarch/architectures.go#L26-L59
    architecture: String,
    config: HashMap<String, String>, // TODO: is config completely described - can we use a struct?
    description: String,
    devices: HashMap<String, Device>,
    /// Whether the instance is ephemeral (deleted on shutdown)
    ephemeral: bool,
    /// Cloud instance type (AWS, GCP, Azure, ...) to emulate with limits (eg. "t1.micro")
    instance_type: String,
    name: String,
    /// None -> let the server set default profiles
    /// Some([]) -> explicitly no profiles set
    /// Some(["profile1", "profile2"]) -> manually set profile1 and profile2
    profiles: Option<Vec<String>>,
    source: InstanceSource,
    /// Whether the instance currently has saved state on disk (only used for migration or copy)
    stateful: bool,
    #[serde(rename = "type")]
    virtualisation_type: VirtualisationType,
}

impl Default for NewInstance {
    fn default() -> Self {
        Self {
            architecture: "".to_owned(),
            config: HashMap::new(),
            description: "instance created by nlxd rust library".to_owned(),
            devices: HashMap::new(),
            ephemeral: false,
            instance_type: "".to_owned(),
            name: "".to_owned(),
            profiles: None,
            source: InstanceSource::None {},
            stateful: false,
            virtualisation_type: VirtualisationType::Container,
        }
    }
}

impl NewInstance {
    pub fn default_with_image(image: String) -> Self {
        Self {
            source: InstanceSource::LocalImage {
                alias: image,
                fingerprint: "".to_owned(),
                properties: HashMap::new(),
                project: "".to_owned(),
            },
            ..Self::default()
        }
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum InstanceSource {
    Copy {
        /// Existing instance name or snapshot
        source: String,
        /// Whether to ignore errors when copying (e.g. for volatile files)
        allow_inconsistent: bool,
        #[serde(rename = "base-image")]
        base_image: String,
        /// Whether the copy should skip the snapshots
        instance_only: bool,
        /// Whether this is refreshing an existing instance
        refresh: bool,
        /// source project name
        project: String,
    },
    // TODO: unsure if this will cause issues (both local image and remote image use "image" type)
    #[serde(rename = "image")]
    LocalImage {
        alias: String,
        fingerprint: String,
        /// Image filters
        properties: HashMap<String, String>,
        project: String,
    },
    #[serde(rename = "image")]
    RemoteImage {
        alias: String,
        fingerprint: String,
        /// Image filters
        properties: HashMap<String, String>,
        certificate: String,
        protocol: ImageProtocol,
        secret: String,
        server: String,
    },
    Migration {
        /// Base image fingerprint (for faster migration)
        #[serde(rename = "base-image")]
        base_image: String,
        certificate: String,
        /// Whether this is a live migration
        live: bool,
        /// Whether to use pull or push mode
        mode: MigrationMode,
        /// Remote operation URL
        operation: String,
        /// Whether this is refreshing an existing instance
        refresh: bool,
        /// Map of migration websockets
        secrets: HashMap<String, String>,
    },
    None {},
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MigrationMode {
    Pull,
    Push,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub enum ImageProtocol {
    Lxd,
    Simplestreams,
}

/// Represents the response to a new instance created.
/// Matches
/// [InstancePost HTTP 200 response](https://documentation.ubuntu.com/lxd/en/latest/api/#/instances/instances_post)
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub struct NewInstanceResponse {
    #[serde(rename = "type")]
    sync_type: String, // eg. async
    #[serde(rename = "status_code")]
    status: ResourceStatus,
    operation: String, // eg. "/1.0/operations/86955e6c-f58c-45e8-b80c-48ae374583e8"
    metadata: Operation,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationClass {
    Task,
    Token,
    Websocket,
}

/// Respresents a background operation in the LXD server.
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Operation {
    class: OperationClass,
    created_at: DateTime<Utc>,
    description: String,
    /// error message if available
    err: String,
    /// UUID of the operation
    id: String,
    /// What cluster member this record was found on (may be "none").
    location: String,
    /// Whether the operation can be canceled
    may_cancel: bool,
    metadata: Option<serde_json::Value>,
    resources: HashMap<String, Vec<String>>,
    #[serde(rename = "status_code")]
    status: ResourceStatus,
    updated_at: DateTime<Utc>,
}
