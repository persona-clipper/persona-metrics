use crate::model::types::metrics::metric::ToRawQuery;

pub enum ProcessListQuery {
    Process { host: String, process_id: i64},
}

impl ToRawQuery for ProcessListQuery {
    fn to_promql(&self) -> String {
        match self {
            Self::Process { host, process_id} => {
              format!("metrics_process_list_cpu_usage.global{{hostname='{}', program_id='{}'}}", host, process_id)
            },
        }
    }
}