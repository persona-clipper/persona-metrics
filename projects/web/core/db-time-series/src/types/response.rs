use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VmResponse<T> {
    pub status: String,
    pub data: VmData<T>,
    pub stats: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct VmData<T> {
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub result: Vec<T>,
}

/// Used for parsing the response from VictoriaMetrics when **resultType** is *vector*
/// 
/// # Example:
/// ```json
/// "metric": {
///     "__name__": "metrics_component_list_critical_temp",
///     "hostname": "test-exporter",
///     "id": "hwmon3_1",
///     "name": "acpitz temp1"
/// },
/// "value": [
///     1794070661,
///     "0"
/// ]
///```
#[derive(Serialize, Deserialize)]
pub struct VmVectorItem {
    pub metric: HashMap<String, String>,
    pub value: (i64, String),
}

/// Used for parsing the response from VictoriaMetrics when **resultType** is *matrix*
///
/// # Example:
/// ```json
/// "metric": {
///     "__name__": "metrics_component_list_critical_temp",
///     "hostname": "test-exporter",
///     "id": "hwmon4_2",
///     "name": "coretemp Core 0"
///  },
///  "values": [
///     [
///         1794070000,
///         "100"
///     ],
///     [
///         1794070015,
///         "100"
///     ],
///     ...
///  ]
///```
pub struct VmMatrixItem {
    pub metric: HashMap<String, String>,
    pub value: Vec<(i64, String)>,
}