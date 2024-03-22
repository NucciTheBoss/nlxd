// Copyright 2024 Canonical
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

//! Types for LXD images

use std::collections::HashMap;

use serde::Deserialize;

// NOTE: this can be moved to a shared types file later.
/// https://github.com/canonical/lxd/blob/main/doc/architectures.md
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    I686,
    X86_64,
    Armv7l,
    Aarch64,
    Ppc,
    Ppc64,
    Ppc64le,
    S390x,
    Mips,
    Mips64,
    Riscv32,
    Riscv64,
}

// NOTE: this too can be moved to a shared location
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Bytes(u64);

// #[serde(deny_unknown_fields)]
#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Image {
    aliases: Vec<Alias>,
    architecture: Arch,
    /// Whether the image should auto-update when a new build is available
    auto_update: bool,
    /// Whether the image is an automatically cached remote image
    cached: bool,
    // TODO: these should be datetimes - added in https://github.com/NucciTheBoss/nlxd/pull/19/files#diff-2e9d962a08321605940b5a657135052fbcef87b5e360662bb527c96d9a615542
    created_at: String,
    expires_at: String,
    filename: String,
    fingerprint: String,
    // TODO: also a datetime
    last_used_at: String,
    /// List of profiles to use when creating from this image (if none provided by user)
    profiles: Vec<String>,
    properties: HashMap<String, String>,
    /// Whether the image is available to unauthenticated users
    public: bool,
    /// Size of the image in bytes
    size: Bytes,
    #[serde(rename = "type")]
    image_type: ImageType,
    update_source: Option<ImageSource>,
    // TODO: also a datetime
    uploaded_at: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Alias {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ImageType {
    Container,
    VirtualMachine,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ImageSource {
    alias: String,
    certificate: String,
    /// NOTE: lxd api docs say:
    /// Type of image (container or virtual-machine).
    /// However this field has observed to also be an empty string.
    image_type: String,
    protocol: RemoteImageProtocol,
    server: String,
}

/// Related LXD docs:
/// - https://documentation.ubuntu.com/lxd/to/latest/howto/images_remote/
/// - https://documentation.ubuntu.com/lxd/to/latest/reference/remote_image_servers/#remote-image-servers
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteImageProtocol {
    Lxd,
    Simplestreams,
}
