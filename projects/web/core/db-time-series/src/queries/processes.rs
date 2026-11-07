use std::fmt::Write;
use crate::types::metrics_query::ToRawQuery;

pub enum ProcessListQuery {
    Process { host: String, process_id: i64},
}

impl ToRawQuery for ProcessListQuery {
    fn to_promql(&self, buffer: &mut String) {
        match self {
            Self::Process { host, process_id} => {
                let _ = write!(buffer, "metrics_process_list_cpu_usage.global{{hostname='{}', program_id='{}'}}", host, process_id);
            },
        }
    }
}