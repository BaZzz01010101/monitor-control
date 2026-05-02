use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dell_controller_core::{snapshot::SnapshotDiff, VcpCode};

#[derive(Debug, Parser)]
#[command(name = "dell-lab")]
#[command(about = "Read-only Dell/DDC discovery utility")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Probe { monitor: String },
    Snapshot { monitor: String },
    Diff { before: String, after: String },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Probe { monitor } => {
            let monitors = dell_controller_core::enumerate_monitors()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            let caps = monitor
                .capabilities
                .as_ref()
                .ok_or_else(|| anyhow!("monitor did not report DDC/CI capabilities"))?;
            let queue = monitor.queue();
            for code in caps.vcp_codes() {
                match queue.get(VcpCode::new(code)) {
                    Ok(feature) => println!(
                        "{} current={} max={}",
                        feature.code, feature.current, feature.maximum
                    ),
                    Err(error) => println!("0x{code:02X} error={error}"),
                }
            }
        }
        Command::Snapshot { monitor } => {
            let monitors = dell_controller_core::enumerate_monitors()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            println!("{}", serde_json::to_string_pretty(&monitor.snapshot())?);
        }
        Command::Diff { before, after } => {
            let before = read_snapshot(&before)?;
            let after = read_snapshot(&after)?;
            let diff = SnapshotDiff::between(&before, &after);
            for changed in diff.changed {
                println!(
                    "changed {}: {} -> {}",
                    changed.code, changed.before, changed.after
                );
            }
            for added in diff.added {
                println!("added {}: {}", added.code, added.value);
            }
            for removed in diff.removed {
                println!("removed {}: {}", removed.code, removed.value);
            }
        }
    }
    Ok(())
}

fn select_monitor<'a>(
    monitors: &'a [dell_controller_core::WindowsMonitor],
    selector: &str,
) -> Result<&'a dell_controller_core::WindowsMonitor> {
    if let Ok(index) = selector.parse::<usize>() {
        return monitors
            .get(index)
            .ok_or_else(|| anyhow!("monitor index {index} not found"));
    }

    let selector = selector.to_ascii_lowercase();
    monitors
        .iter()
        .find(|monitor| {
            monitor
                .info
                .description
                .to_ascii_lowercase()
                .contains(&selector)
                || monitor
                    .info
                    .model
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .contains(&selector)
        })
        .ok_or_else(|| anyhow!("monitor `{selector}` not found"))
}

fn read_snapshot(path: &str) -> Result<dell_controller_core::Snapshot> {
    let input = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&input)?)
}
