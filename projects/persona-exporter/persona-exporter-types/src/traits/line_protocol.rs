use crate::metrics::line_protocol::GlobalTags;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

pub trait FromWithMeasurement<T> {
    fn from_with_name(value: T, measurement: &str, global_tags: &GlobalTags) -> Self;
}

pub trait IntoWithMeasurement<T> {
    // type Target;
    fn into_with_name(self, measurement: &str, global_tags: &GlobalTags) -> T;
}

pub trait InsertGlobalTags {
    fn insert_global_tags(self, global_tags: &GlobalTags) -> Self;
}

pub trait FinishLineProtocol {
    fn finish(self, timestamp: i64) -> Vec<u8>;
}

impl FinishLineProtocol for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn finish(self, timestamp: i64) -> Vec<u8> {
        self.timestamp(timestamp).close_line().build()
    }
}

impl<T, U> IntoWithMeasurement<U> for T
where
    U: FromWithMeasurement<T>,
{
    fn into_with_name(self, measurement: &str, global_tags: &GlobalTags) -> U {
        U::from_with_name(self, measurement, global_tags)
    }
}
