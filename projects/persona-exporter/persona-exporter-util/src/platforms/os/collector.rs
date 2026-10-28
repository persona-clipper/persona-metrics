use crate::config::{AgentConfigFile, DataType};
use crate::platforms::os::methods::metrics::processes::{
    TryCollectProcessesArgs, try_collect_process_list_metrics,
};
use core::time;
use std::cmp::Ordering;
use crate::platforms::os::methods::arguments::{GlobalContext, RequestBodyOptions};
use crate::platforms::os::methods::{
    build_request_body, collect_metrics_as_line_protocol, send_request,
};

use crate::platforms::os::methods::metrics::components::try_collect_components_metrics;
use crate::platforms::os::methods::metrics::cpu::try_collect_cpu_metrics;
use crate::platforms::os::methods::metrics::disk::try_collect_disk_metrics;
use crate::platforms::os::methods::metrics::memory::try_collect_memory_metrics;
use crate::platforms::os::methods::metrics::network::try_collect_network_metrics;
use crate::platforms::os::methods::metrics::system::try_collect_system_metrics;
use smol::stream::StreamExt;
use std::time::{Duration, SystemTime};
use persona_exporter_types::metrics::structs::processes::ProcessInfo;
use tracing::{debug, info};
use persona_exporter_types::metrics::line_protocol::GlobalTags;
use persona_exporter_types::metrics::server::ServerMetrics;

pub async fn collect_metrics_for_os(
    config: &AgentConfigFile,
    global_context: &mut GlobalContext<'_, impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering + Send + Sync>
) {
    // TODO LIST:
    // Добавить массив url и send_interval - чтобы один экспортер мой слать метрики на разные url с разной переодичностью.
    // Как это будет выглядеть:
    // ...
    //  targets:
    //      - urls: ["https://example.com/" "https://example2.com"]
    //        send_interval: 10s
    //        timeout: 30s
    // ...

    // Specific objects for the push model
    let additional_headers = &config.server.push.http_headers;
    let url_params = &config.server.push.url_params;
    let target_url = &config.server.push.url;
    let await_seconds = config.server.push.send_interval;
    let mut interval = smol::Timer::interval(Duration::from_secs(await_seconds));

    let http_client_config = ureq::Agent::config_builder()
        .timeout_global(Some(time::Duration::from_secs(64)))
        .build();

    let request_options = RequestBodyOptions {
        config: &http_client_config,
        url: target_url,
        get_params: url_params,
        headers: additional_headers,
    };
    let mut process_update_context = TryCollectProcessesArgs {
        include_exporter_metrics: config.metrics.processes.include_exporter_metrics,
        sort_by: &global_context.variables.sort_by,
        process_limit: global_context.variables.process_limit,
        remove_dead_processes: config.metrics.processes.remove_dead_processes,
        refresh_kind: None,
    };

    // Main
    while interval.next().await.is_some() {
        info!("Next metrics created after {} seconds", await_seconds);

        // Метрики требующиe [`sysinfo::System`]
        if let Some(sys_context) = &mut global_context.snapshots.system_context {
            let s = &mut sys_context.snapshot;
            let metrics_buf = &mut global_context.buffers.metrics;
            process_update_context.refresh_kind = sys_context.refresh_kinds.process_refresh_kind;

            // Здесь не нужна проверка, включена ли та или иная секция метрик в конфиге. Так как если выключена
            // То buffers.metrics.memory будет равнятся None. Такое поведение заложено ещё в инициализации структуры ServerMetrics
            try_collect_memory_metrics(s, &mut metrics_buf.memory);
            try_collect_cpu_metrics(s, &mut metrics_buf.cpu, global_context.variables.physical_core_count);
            try_collect_system_metrics(&mut metrics_buf.system);

            // if let Some(ref mut mem_buf) = global_context.buffers.metrics.memory {
            //     s.refresh_memory();
            //     // mem_buf.clear_dynamic();
            //     collect_memory_metrics(s, mem_buf);
            // }
            // if let Some(ref mut cpu_buf) = global_context.buffers.metrics.cpu {
            //     s.refresh_cpu_all();
            //     cpu_buf.clear_dynamic();
            //     collect_cpus_metrics(s, global_context.variables.physical_core_count, cpu_buf);
            // }
            // if let Some(ref mut system_buf) = global_context.buffers.metrics.system {
            //     system_buf.clear_dynamic();
            //     collect_system_metrics(system_buf);
            // }

            try_collect_process_list_metrics(
                s,
                &mut global_context.buffers.metrics.process_list,
                &process_update_context
            );
        //     if let (Some(process_list_buf), Some(refresh_kind)) = (
        //         &mut global_context.buffers.metrics.process_list,
        //         sys_context.refresh_kinds.process_refresh_kind,
        //     ) {
        //         s.refresh_processes_specifics(
        //             ProcessesToUpdate::All,
        //             config.metrics.processes.remove_dead_processes,
        //             refresh_kind,
        //         );
        //         // s.refresh_processes(ProcessesToUpdate::All, false);
        //         process_list_buf.clear_dynamic();

        //         // Информация о процессах
        //         update_process_list_info(s, &mut process_list_buf.process_list);
        //         //// Сортируем по заданной функции aka замыканию
        //         process_list_buf
        //             .process_list
        //             .sort_unstable_by(&global_context.variables.sort_by);
        //         //// Обрезаем готовый массив
        //         process_list_buf
        //             .process_list
        //             .truncate(global_context.variables.process_limit);

        //         // Отдельная информация о самом экспортере
        //         if config.metrics.processes.include_exporter_metrics
        //             && let (Ok(pid), Some(self_metrics)) =
        //                 (get_current_pid(), &mut process_list_buf.exporter_metrics)
        //         {
        //             self_metrics.clear_dynamic();
        //             let process = get_process_by_id(s, pid);
        //             if let Some(process) = process {
        //                 write_process_info(process, self_metrics);
        //             }
        //         }
        //     }
        }

        try_collect_disk_metrics(
            &mut global_context.snapshots.disks,
            &mut global_context.buffers.metrics.disk
        );
        try_collect_network_metrics(
            &mut global_context.snapshots.networks,
            &mut global_context.buffers.metrics.network,
        );
        try_collect_components_metrics(
            &mut global_context.snapshots.components,
            &mut global_context.buffers.metrics.components,
        );

        // if let (Some(disk_buffer), Some(d)) = (
        //     &mut global_context.buffers.metrics.disk,
        //     &mut global_context.snapshots.disks,
        // ) {
        //     d.refresh(false);
        //     disk_buffer.clear_dynamic();
        //     collect_storage_list_metrics(d, disk_buffer);
        // }
        // if let (Some(network_buffer), Some(n)) = (
        //     &mut global_context.buffers.metrics.network,
        //     &mut global_context.snapshots.networks,
        // ) {
        //     n.refresh(false);
        //     network_buffer.clear_dynamic();
        //     collect_network_metrics(n, network_buffer);
        // }
        // if let (Some(components_info), Some(c)) = (
        //     &mut global_context.buffers.metrics.components,
        //     &mut global_context.snapshots.components,
        // ) {
        //     c.refresh(false);
        //     components_info.clear_dynamic();
        //     collect_components_metrics(c, components_info);
        // }

        global_context.buffers.metrics.time = get_timestamp();

        let request = build_request_body(&request_options);
        match config.agent.data_type {
            DataType::LineProtocol => {
                let mut line_buffer =
                    std::mem::take(&mut global_context.buffers.line_protocol_buffer);

                build_body_as_line_protocol(
                    &global_context.buffers.metrics,
                    &mut line_buffer,
                    global_context.variables.global_tags
                );
                send_request(request, &line_buffer).await;
            }
            DataType::Json => {
                build_body_as_json(&global_context.buffers.metrics, &mut global_context.buffers.json_buffer);

                send_request(request, &global_context.buffers.json_buffer.to_string()).await;
            }
        }
    }
}

pub fn build_body_as_json(
    metrics: &ServerMetrics,
    buffer: &mut serde_json::Value,
) {
    *buffer = serde_json::json!(metrics);
    debug!("Machine metrics: {:#?}", buffer);
}

pub fn build_body_as_line_protocol(
    metrics: &ServerMetrics,
    buffer: &mut Vec<u8>,
    global_tags: &GlobalTags
) {
    buffer.clear();
    collect_metrics_as_line_protocol(
        metrics, buffer, global_tags
    );
    debug!("{:#?}", String::from_utf8(buffer.clone()));
}

#[must_use]
pub fn get_timestamp() -> i64 {
    i64::try_from(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    ).unwrap_or_default()
}
