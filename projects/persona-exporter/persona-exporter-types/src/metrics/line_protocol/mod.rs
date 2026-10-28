use crate::traits::line_protocol::InsertGlobalTags;
use compact_str::CompactString;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterMeasurement;
use std::collections::HashMap;

#[cfg(feature = "components")]
pub mod components;
#[cfg(feature = "cpu")]
pub mod cpu;
#[cfg(feature = "disk")]
pub mod disk;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "processes")]
pub mod processes;
pub mod server;
#[cfg(feature = "system")]
pub mod system;

#[must_use]
pub const fn unknown_or_value(v: &str) -> &str {
    if v.is_empty() {
        crate::DEFAULT_UNKNOWN_MESSAGE
    } else {
        v
    }
}

pub type GlobalTags = HashMap<CompactString, CompactString>;

impl InsertGlobalTags for LineProtocolBuilder<Vec<u8>, AfterMeasurement> {
    fn insert_global_tags(self, global_tags: &GlobalTags) -> Self {
        let mut builder = self;

        for (tag, value) in global_tags {
            builder = builder.tag(tag, value);
        }

        builder
    }
}
