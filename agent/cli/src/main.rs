use agent_cli::app::pipeline::Pipeline;
use agent_cli::app::reporting::{build_payload, submit_report, ReportOptions};
use agent_cli::plugins::registry::{builtin_plugins, plugin_by_name};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "agent", version, about = "Security Agent CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        #[command(subcommand)]
        target: ScanTargets,
    },
    Report {
        #[arg(long, default_value = "http://localhost:8082/v1/reports/upload")]
        endpoint: String,
        #[arg(long, default_value = "fintech.mobile")]
        app_id: String,
        #[arg(long, default_value = "local")]
        env: String,
        #[arg(long, default_value = "ci")]
        source: String,
        #[arg(long, env = "AGENT_API_TOKEN")]
        token: Option<String>,
        #[arg(long)]
        pipeline_provider: Option<String>,
        #[arg(long)]
        pipeline_run_id: Option<String>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ScanTargets {
    Perimeter,
    RateLimit,
    Authz,
    MobileBuild,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { target } => {
            let plugin_name = match target {
                ScanTargets::Perimeter => "perimeter",
                ScanTargets::RateLimit => "rate-limit",
                ScanTargets::Authz => "authz",
                ScanTargets::MobileBuild => "mobile-build",
            };

            let plugins = plugin_by_name(plugin_name)
                .into_iter()
                .collect::<Vec<_>>();
            let pipeline = Pipeline::new(plugins);
            let report = pipeline.run();

            println!(
                "scan {} completed. findings: {}",
                plugin_name,
                report.findings.len()
            );
        }
        Commands::Report {
            endpoint,
            app_id,
            env,
            source,
            token,
            pipeline_provider,
            pipeline_run_id,
            dry_run,
        } => {
            let plugins = builtin_plugins();
            let pipeline = Pipeline::new(plugins);
            let report = pipeline.run();

            let options = ReportOptions {
                endpoint,
                app_id,
                env,
                source,
                pipeline_provider,
                pipeline_run_id,
                token,
            };

            if dry_run {
                let payload = match build_payload(&report, &options) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("failed to build report payload: {}", err.message);
                        return;
                    }
                };
                let json = serde_json::to_string_pretty(&payload.body)
                    .unwrap_or_else(|_| "{}".to_string());
                println!("{}", json);
                return;
            }

            if let Err(err) = submit_report(&report, options) {
                eprintln!("report upload failed: {}", err.message);
                return;
            }

            println!("report uploaded. findings: {}", report.findings.len());
        }
    }
}
