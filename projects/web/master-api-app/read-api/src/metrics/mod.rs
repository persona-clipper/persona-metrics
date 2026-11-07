use axum::extract::{Path, Query, State};
use axum::Json;
use url::quirks::host;
use core_library_db_types::error::ApiErrorCode;
use core_library_tsdb::queries::cpu::CpuQuery;
use core_library_tsdb::types::metrics_query::{MetricQuery};
use core_library_tsdb::types::QueryParams;
use core_library_tsdb::types::response::{VmResponse, VmVectorItem};
use core_library_tsdb::types::step::{Step, StepRange};
use crate::state::AppState;

pub async fn global_cpu_usage(
    Query(params): Query<QueryParams>,
    Path(hostname): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<VmResponse<VmVectorItem>>, ApiErrorCode>
{
    let query = MetricQuery::Cpu(CpuQuery::CpuUsage { host: hostname });
    let step = params.step.unwrap_or_else(|| Step(5).to_metrics_step(StepRange::Seconds));

    let mut vm_state = state.vm.lock().await;
    let mut buffer = std::mem::take(&mut vm_state.query_buffer);

    let response = vm_state.query(query, &step, &mut buffer).await;
    match response {
        Ok(val) => {
            Ok(Json(val))
        }
        Err(err) => {
            Err(ApiErrorCode::Other { reason: err.to_string()})
        }
    }
}