use clap::Parser;
use dellctl::args::{Cli, Command};

#[test]
fn parses_monitor_list_command() {
    let cli = Cli::try_parse_from(["dellctl", "monitors"]).expect("valid command");

    assert!(matches!(cli.command, Command::Monitors));
}

#[test]
fn parses_set_command_with_hex_vcp_and_value() {
    let cli = Cli::try_parse_from(["dellctl", "set", "0", "0x10", "75"]).expect("valid command");

    match cli.command {
        Command::Set {
            monitor,
            control,
            value,
            advanced,
        } => {
            assert_eq!(monitor, "0");
            assert_eq!(control, "0x10");
            assert_eq!(value, "75");
            assert!(!advanced);
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn probe_is_read_only_by_default() {
    let cli = Cli::try_parse_from(["dellctl", "probe", "0"]).expect("valid command");

    match cli.command {
        Command::Probe { read_only, .. } => assert!(read_only),
        other => panic!("unexpected command: {other:?}"),
    }
}
