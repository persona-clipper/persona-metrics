use crate::config::ProcessSortBy;
use persona_exporter_types::metrics::processes::ProcessListInfo;
use persona_exporter_types::metrics::traits::Clear;
use persona_exporter_types::metrics::structs::processes::ProcessInfo;
use persona_exporter_types::metrics::sysinfo::processes::FromWithNormalizeCpu;
use std::cmp::Ordering;
use sysinfo::{Pid, ProcessRefreshKind, System, get_current_pid};

pub struct TryCollectProcessesArgs<'a, F>
where
    F: Fn(&ProcessInfo, &ProcessInfo) -> Ordering + 'a
{
   pub include_exporter_metrics: bool,
   pub sort_by: &'a F,
   pub process_limit: usize,
   pub remove_dead_processes: bool,
   pub refresh_kind: Option<ProcessRefreshKind>,
}


#[must_use]
pub fn get_process_by_id(sys: &System, pid: Pid) -> Option<ProcessInfo> {
    sys.process(pid).map(|process| {
        ProcessInfo::from_with_cpu(process, sys.cpus().len())
    })
}

pub fn get_sort_closure(
    sort_by: &ProcessSortBy,
) -> impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering {
    match sort_by {
        ProcessSortBy::Memory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.memory_usage.cmp(&a.memory_usage)
        }
        ProcessSortBy::VirtualMemory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.virtual_memory.cmp(&a.virtual_memory)
        }
        ProcessSortBy::RunTime => |a: &ProcessInfo, b: &ProcessInfo| b.run_time.cmp(&a.run_time),
        ProcessSortBy::StartTime => {
            |a: &ProcessInfo, b: &ProcessInfo| b.start_time.cmp(&a.start_time)
        }
        // default also contain ProcessSortBy::CpuUsage
        ProcessSortBy::CpuUsage => {
            |a: &ProcessInfo, b: &ProcessInfo| b.global_cpu_usage.total_cmp(&a.global_cpu_usage)
        }
    }
}

pub fn write_process_info(process: ProcessInfo, buffer: &mut ProcessInfo) {
    buffer.status = process.status;
    buffer.disk_usage = process.disk_usage;

    buffer.global_cpu_usage = process.global_cpu_usage;
    buffer.cpu_usage_per_thread = process.cpu_usage_per_thread;
    buffer.memory_usage = process.memory_usage;
    buffer.virtual_memory = process.virtual_memory;
    buffer.run_time = process.run_time;
    buffer.start_time = process.start_time;

    buffer.name.push_str(&process.name);
    buffer.program_id.push_str(&process.program_id);
    buffer.user_id.push_str(&process.user_id);
    buffer.group_id.push_str(&process.group_id);
}

pub fn update_process_list_info(
    sys: &System,
    // process_list_buffer: &mut Vec<ProcessInfo>,
    process_list_buffer: &mut Vec<ProcessInfo>,
) {
    process_list_buffer.clear();
    // let mut local_process_list_buffer = take(&mut process_list_struct.process_list);

    sys.processes().values().for_each(|process| {
        process_list_buffer.push(ProcessInfo::from_with_cpu(process, sys.cpus().len()));
    });

    // let self_process_metrics: Option<ProcessInfo> = get_current_pid()
    //     .ok()
    //     .and_then(|pid| sys.process(pid))
    //     .map(Into::into);

    // local_process_list_buffer.extend(
    //     sys.processes().values().map(|p| ProcessInfo::from_process(p))
    // );
    // sys
    //     .processes()
    //     .iter()
    //     .for_each(|process| process_list_buffer.push(ProcessInfo::from(process.1)));
    // .collect();

    // let sort_by_closure = get_sort_closure(sort_by);
    //
    // local_process_list_buffer.sort_unstable_by(sort_by_closure);
    // local_process_list_buffer.truncate(process_limit);
    // process_list_struct.process_list = local_process_list_buffer;

    // *process_list_struct = ProcessListInfo {
    //     exporter_metrics: self_process_metrics,
    //     process_list: local_process_list_buffer,
    // };
}

/// TODO:
/// Optimize too many arguments in one function
/// Solution:
/// Move the arguments for this function into a separate structure
pub fn try_collect_process_list_metrics(
    sys: &mut System,
    process_list_buf: &mut Option<ProcessListInfo>,
    context: &TryCollectProcessesArgs<'_, impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering>,

    // process_refresh_kind: &Option<ProcessRefreshKind>,
    // remove_dead_processes: bool,
    // sort_by: impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering,
    // process_limit: usize,
    // include_exporter_metrics: bool,
) {
    if let (Some(buf), Some(refresh_kind)) = (process_list_buf, context.refresh_kind)
    {
        sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            context.remove_dead_processes,
            refresh_kind,
        );
        buf.clear_dynamic();

        update_process_list_info(sys, &mut buf.process_list);

        buf.process_list.sort_unstable_by(context.sort_by);
        buf.process_list.truncate(context.process_limit);

        if context.include_exporter_metrics
            && let(Ok(pid), Some(self_metrics)) =
                (get_current_pid(), &mut buf.exporter_metrics)
        {
            self_metrics.clear_dynamic();
            if let Some(process) = get_process_by_id(sys, pid) {
                write_process_info(process, self_metrics);
            }
        }
    }
}
