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

// I have no idea if half of these should be strings but oh well
pub struct Instance {
    name: String, // TODO: Ensure that this is read-only
    description: String,
    architecture: String,
    created_at: String,
    config: String,
    ephemeral: bool,
    devices: String,
    expanded_config: String,
    expanded_devices: String,
    profiles: String,
    status: String,        // TODO: Ensure that this is read-only
    last_used_at: String,  // TODO: Ensure that this is read-only
    location: String,      // TODO: Ensure that this is read-only
    instance_type: String, // TODO: Ensure that this is read-only
    project: String,       // TODO: Ensure this is read-only and optional.
    status_code: String,   // TODO: Ensure that this is read-only
    stateful: String,      // TODO: Ensure that this is read-only
    snapshots: String,
    files: String,
}
