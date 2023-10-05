use crate::core::utils::*;
use crate::meow::meow::read_meow;
use std::fs::File;
use std::io::prelude::*;

pub fn json_filter(line: &str) -> String {
    let mut filtered_line = "".to_string();
    for symbol in line.chars() {
        if symbol == '"' {
            filtered_line = format!("{}{}", filtered_line, "\"");
        } else if symbol == '\\' {
            filtered_line = format!("{}{}", filtered_line, "");
        } else if symbol == '\n' {
            filtered_line = format!("{}{}", filtered_line, "");
        } else if symbol == '\t' {
            // tab to 4
            filtered_line = format!("{}{}", filtered_line, "    ");
        } else {
            filtered_line = format!("{}{}", filtered_line, symbol);
        }
    }
    return filtered_line;
}

pub fn format_std(data: String) -> Vec<String> {
    let mut formated: Vec<String> = Vec::new();
    let mut lines = data.lines();
    loop {
        match lines.next() {
            Some(line) => {
                let filtered_line = json_filter(line);
                formated.push(filtered_line);
            }
            None => break,
        }
    }
    return formated;
}

pub fn write_report(
    command_string: String,
    status: String,
    stdout: String,
    stderr: String,
    _debug: bool,
) -> std::io::Result<()> {
    let mut formated_stdout: Vec<String> = format_std(stdout);
    let mut formated_stderr: Vec<String> = format_std(stderr);

    let cmd: Vec<String> = command_string
        .split_whitespace()
        .map(String::from)
        .collect();

    let report_config = read_meow("/var/maid/maid_lists/embedded/config.meow", false);
    let report = format!("{}{}", report_config["REPORT_BASE_PATH"], report_config["REPORT_LOG"]);

    let session_config = read_meow("/var/maid/maid_lists/embedded/session.meow", false);
    let session = session_config["LATEST_SESSION"].to_string();

    let contents = format!(
        "{{ \"session\": \"{}\", \"command_base\": \"{}\", \"timestemp\": \"{}\", \"command_status\": \"{}\", \"command_string\": \"{}\", \"formated_stdout\": {:?}, \"formated_stderr\": {:?} }}\n",
        session,
        cmd[0],
        system_time(),
        status,
        command_string,
        formated_stdout,
        formated_stderr,
    );

    let file = File::options().append(true).open(&report);
    let _ = file.expect("FILE").write_all(&contents.as_bytes());
    Ok(())
}
