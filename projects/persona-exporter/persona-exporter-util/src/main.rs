use persona_exporter::platforms::*;
use std::process::exit;

#[cfg_attr(target_os = "none", no_std)]
#[cfg_attr(target_os = "none", no_main)]
#[cfg(target_os = "none")]
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    microcontroller::collect_metrics::collect_metrics_for_microcontroller();
}

#[cfg(not(target_os = "none"))]
use mimalloc::MiMalloc;
use persona_exporter::config::{AgentConfigFile, MainCliArguments, SendModel};
use persona_exporter::platforms::os::methods::{create_metrics_struct_by_config, initial_tracing};
use tracing::info;
use persona_exporter::platforms::os::methods::arguments::GlobalContext;
use persona_exporter::platforms::os::methods::metrics::processes::get_sort_closure;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
fn main() {
    let args: MainCliArguments = argh::from_env();

    let config = match AgentConfigFile::new(args.config_path) {
        Ok(config) => {
            if args.config_test {
                println!("OK");
                exit(0);
            }
            config
        }
        Err(err) => {
            println!("FAIL - {}", err);
            exit(0);
        }
    };

    let verbose_level = if args.config_test { 2 } else { args.verbose };

    if verbose_level != 0 {
        println!(
            "The exporter is running and collecting metrics. To enable logging, run the program with the `-v 2` flag."
        );
    }

    initial_tracing(verbose_level);

    info!("{:#?}", config);

    info!("Exporter initialized");

    let sort_by = get_sort_closure(&config.metrics.processes.sort_by);
    let mut global_context =
        GlobalContext::new(&config, sort_by, create_metrics_struct_by_config(&config));
    
    smol::block_on(async {
        match config.agent.send_model {
            SendModel::Push => {
                info!("Exporter work send model: PUSH");
                os::collector::collect_metrics_for_os(&config, &mut global_context).await;
            }
            SendModel::Pull => {
                info!("Exporter work send model: PULL");
            }
        }
    });
}
