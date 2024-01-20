// 🐻‍❄️🧊 provisionerd: Deploy VMs with code.
// Copyright 2023-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::protos::generated;
use chrono::{DateTime, Local};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

/// Represents what backend this VM is from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Backend {
    Libvirt,
}

impl From<Backend> for generated::Backend {
    fn from(value: Backend) -> Self {
        use Backend::*;

        match value {
            Libvirt => Self::Libvirt,
        }
    }
}

/// Represents a [`VM`] status.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Status {
    /// The VM is alive and well.
    Alive,

    /// VM has died for some reason
    Dead,

    /// VM is a zombie! While it can exist, it is not responding
    /// as it should be... I wonder if we need the *crew* back together?
    Zombie,

    /// VM is unavailable due to unknown constraints. This is the default state
    /// of a [`VM`] when it is created.
    #[default]
    Unavailable,

    /// VM is unresponsive!
    Unresponsive,
}

impl From<Status> for generated::Status {
    fn from(value: Status) -> generated::Status {
        use Status::*;

        match value {
            Alive => generated::Status::Alive,
            Dead => generated::Status::Dead,
            Unavailable => generated::Status::Unavailable,
            Unresponsive => generated::Status::Unresponsive,
            Zombie => generated::Status::Zombie,
        }
    }
}

/// Represents a VM that is provisioned by `provisionerd`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VM {
    /// List of labels so that this VM can be searched easily.
    //#[serde(default, skip_serializing_if = "Vec::is_empty")]
    //pub labels: Vec<LabelDescriptor>,

    /// Status of this VM.
    #[serde(default)]
    pub status: Status,

    /// Backend where this VM lives in.
    pub backend: Backend,

    /// Represents what time this VM was created at.
    pub created: DateTime<Local>,

    /// Represents what time this VM was last modified.
    pub updated: DateTime<Local>,

    /// Name of the VM.
    pub name: String,

    /// UUID of the VM that is identifiable.
    pub id: Uuid,
}

const NANOS_PER_SECOND: i64 = 1000000;

impl From<VM> for generated::Vm {
    fn from(value: VM) -> Self {
        generated::Vm {
            networking: None,
            disks: vec![],
            image: String::from("weow"),

            // `last_updated` & `created` are from this issue, so credits to the person who wrote it
            // https://github.com/tokio-rs/prost/issues/590
            last_updated: Some({
                // unless Noelware exceeds expectations over year 2262 and higher, then
                // I don't expect this will panic at all. And if it does and you're
                // reading it, hello! :D
                let nanos = value.updated.timestamp_nanos_opt().unwrap();
                let seconds = nanos / NANOS_PER_SECOND;
                let nanos = (nanos % NANOS_PER_SECOND) as i32;

                Timestamp { seconds, nanos }
            }),

            created: Some({
                let nanos = value.created.timestamp_nanos_opt().unwrap();
                let seconds = nanos / NANOS_PER_SECOND;
                let nanos = (nanos % NANOS_PER_SECOND) as i32;

                Timestamp { seconds, nanos }
            }),

            backend: value.backend as i32,
            status: value.status as i32,
            labels: vec![],
            // labels: value
            //     .labels
            //     .iter()
            //     .filter_map(|(key, value)| match value {
            //         Value::String(st) => Some((key, st)),
            //         _ => None,
            //     })
            //     .map(|(key, value)| generated::LabelDescriptor {
            //         description: String::from("not implemented"),
            //         value: Value::String(value.clone()),
            //         name: key.clone(),
            //     })
            //     .collect(),
            name: value.name,
            id: value.id.to_string(),
        }
    }
}
