//! CLI argument parser for run subcommand.

use super::types::CliCommand;

/// Parses run subcommand: `inferprobe run [--iterations N] [--out FILE] [--html FILE] -- <cmd> [args...]`
pub fn parse_run(args: &[String]) -> Result<CliCommand, String> {
    let mut iterations = 10;
    let mut out_file = None;
    let mut html_file = None;

    let sep_pos = args.iter().position(|a| a == "--");
    let (flags, cmd_args) = match sep_pos {
        Some(pos) => (&args[..pos], &args[pos + 1..]),
        None => (args, &[][..]),
    };

    let mut i = 0;
    while i < flags.len() {
        match flags[i].as_str() {
            "--iterations" if i + 1 < flags.len() => {
                iterations = flags[i + 1].parse().unwrap_or(10);
                i += 2;
            }
            "--out" if i + 1 < flags.len() => {
                out_file = Some(flags[i + 1].clone());
                i += 2;
            }
            "--html" if i + 1 < flags.len() => {
                html_file = Some(flags[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }

    if cmd_args.is_empty() {
        return Err("run requires '-- <command> [args...]'".into());
    }

    let cmd = cmd_args[0].clone();
    let rest_args = cmd_args[1..].to_vec();

    Ok(CliCommand::Run {
        cmd,
        args: rest_args,
        iterations,
        out_file,
        html_file,
    })
}
