use std::{fs, path::Path};

use anyhow::{anyhow, bail, Context, Result};
use dell_controller_core::{
    ddc::{parse_u32, VcpCode},
    hdr,
    profile::MonitorProfile,
    profiles::profile_for_model,
    snapshot::{Snapshot, SnapshotDiff},
    CommandQueue, RetryPolicy, WindowsMonitor,
};

use crate::args::{Cli, Command};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedWrite {
    pub code: VcpCode,
    pub value: u32,
}

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Monitors => list_monitors(),
        Command::Caps { monitor } => {
            let monitors = enumerate()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            print_caps(monitor);
            Ok(())
        }
        Command::Get { monitor, control } => {
            let monitors = enumerate()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            let profile = profile_for_monitor(monitor);
            let code = resolve_read(profile.as_ref(), &control)?;
            let feature = monitor.queue().get(code)?;
            println!(
                "{} current={} max={}",
                feature.code, feature.current, feature.maximum
            );
            Ok(())
        }
        Command::Set {
            monitor,
            control,
            value,
            advanced,
        } => {
            let monitors = enumerate()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            let profile = profile_for_monitor(monitor);
            let write = resolve_write(profile.as_ref(), &control, &value, advanced)?;
            monitor.queue().set(write.code, write.value)?;
            println!("set {} to {}", write.code, write.value);
            Ok(())
        }
        Command::Probe { monitor, read_only } => {
            if !read_only {
                bail!("probe write mode is intentionally not implemented in v1; use set --advanced for explicit writes");
            }
            let monitors = enumerate()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            probe_monitor(monitor)
        }
        Command::Snapshot { monitor } => {
            let monitors = enumerate()?;
            let monitor = select_monitor(&monitors, &monitor)?;
            let snapshot = monitor.snapshot();
            println!("{}", serde_json::to_string_pretty(&snapshot)?);
            Ok(())
        }
        Command::Diff { before, after } => {
            let before = read_snapshot(before)?;
            let after = read_snapshot(after)?;
            print_diff(&SnapshotDiff::between(&before, &after));
            Ok(())
        }
    }
}

pub fn resolve_write(
    profile: Option<&MonitorProfile>,
    control: &str,
    value: &str,
    advanced: bool,
) -> Result<ResolvedWrite> {
    let value = parse_u32(value).with_context(|| format!("invalid value `{value}`"))?;

    if let Some(profile) = profile {
        if let Some(control_def) = profile.control(control) {
            if !advanced && !control_def.safe_write {
                bail!("control `{control}` is not marked safe for writes");
            }
            return Ok(ResolvedWrite {
                code: control_def.vcp,
                value,
            });
        }
    }

    let code: VcpCode = control
        .parse()
        .with_context(|| format!("unknown control or VCP code `{control}`"))?;

    let safe_raw_write = profile
        .map(|profile| profile.can_write_vcp(code.get()))
        .unwrap_or(false);
    if !advanced && !safe_raw_write {
        bail!("raw or unknown VCP writes require --advanced");
    }

    Ok(ResolvedWrite { code, value })
}

fn resolve_read(profile: Option<&MonitorProfile>, control: &str) -> Result<VcpCode> {
    if let Some(profile) = profile {
        if let Some(control_def) = profile.control(control) {
            return Ok(control_def.vcp);
        }
    }

    control
        .parse()
        .with_context(|| format!("unknown control or VCP code `{control}`"))
}

fn list_monitors() -> Result<()> {
    let monitors = enumerate()?;
    for (index, monitor) in monitors.iter().enumerate() {
        let model = monitor
            .info
            .model
            .as_deref()
            .or_else(|| {
                monitor
                    .capabilities
                    .as_ref()
                    .and_then(|caps| caps.model.as_deref())
            })
            .unwrap_or("unknown");
        println!(
            "{index}: {} model={} ddc={}",
            monitor.info.description,
            model,
            monitor.capabilities.is_some()
        );
    }

    match hdr::hdr_states() {
        Ok(states) if !states.is_empty() => {
            for state in states {
                println!(
                    "hdr display={} supported={} enabled={} bits={}",
                    state.display_index,
                    state.supported,
                    state.enabled,
                    state.bits_per_color_channel
                );
            }
        }
        _ => {}
    }

    Ok(())
}

fn print_caps(monitor: &WindowsMonitor) {
    if let Some(raw) = &monitor.raw_capabilities {
        println!("{raw}");
    }
    if let Some(caps) = &monitor.capabilities {
        println!("model={}", caps.model.as_deref().unwrap_or("unknown"));
        println!("mccs={}", caps.mccs_version.as_deref().unwrap_or("unknown"));
        let codes = caps
            .vcp_codes()
            .map(|code| format!("0x{code:02X}"))
            .collect::<Vec<_>>()
            .join(" ");
        println!("vcp={codes}");
    }
}

fn probe_monitor(monitor: &WindowsMonitor) -> Result<()> {
    let caps = monitor
        .capabilities
        .as_ref()
        .ok_or_else(|| anyhow!("monitor did not report DDC/CI capabilities"))?;
    let queue: CommandQueue<_> = CommandQueue::new(monitor.backend.clone(), RetryPolicy::default());

    for code in caps.vcp_codes() {
        match queue.get(VcpCode::new(code)) {
            Ok(feature) => println!(
                "{} current={} max={}",
                feature.code, feature.current, feature.maximum
            ),
            Err(error) => println!("0x{code:02X} error={error}"),
        }
    }

    Ok(())
}

fn enumerate() -> Result<Vec<WindowsMonitor>> {
    dell_controller_core::enumerate_monitors().map_err(Into::into)
}

fn select_monitor<'a>(
    monitors: &'a [WindowsMonitor],
    selector: &str,
) -> Result<&'a WindowsMonitor> {
    if let Ok(index) = selector.parse::<usize>() {
        return monitors
            .get(index)
            .ok_or_else(|| anyhow!("monitor index {index} not found"));
    }

    let selector_lower = selector.to_ascii_lowercase();
    monitors
        .iter()
        .find(|monitor| {
            monitor
                .info
                .description
                .to_ascii_lowercase()
                .contains(&selector_lower)
                || monitor
                    .info
                    .model
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .contains(&selector_lower)
        })
        .ok_or_else(|| anyhow!("monitor `{selector}` not found"))
}

fn profile_for_monitor(monitor: &WindowsMonitor) -> Option<MonitorProfile> {
    profile_for_model(monitor.info.model.as_deref())
}

fn read_snapshot(path: impl AsRef<Path>) -> Result<Snapshot> {
    let path = path.as_ref();
    let input = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&input).with_context(|| format!("parse {}", path.display()))
}

fn print_diff(diff: &SnapshotDiff) {
    for changed in &diff.changed {
        println!(
            "changed {}: {} -> {}",
            changed.code, changed.before, changed.after
        );
    }
    for added in &diff.added {
        println!("added {}: {}", added.code, added.value);
    }
    for removed in &diff.removed {
        println!("removed {}: {}", removed.code, removed.value);
    }
    if diff.changed.is_empty() && diff.added.is_empty() && diff.removed.is_empty() {
        println!("no changes");
    }
}
