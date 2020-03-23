use std::fs::read_to_string;
use std::process::Command;

use crate::dhcp::constant::{BULLETIN_HELPER, COMMAND_PROMPT, FILTER_DHCP, STORE,
                            SUCCESS_MESSAGE};
use crate::dhcp::operations::fetch_dhcp_logs::fetch_dhcp_logs;
use crate::utilities::store_text::store_in_file;
use crate::dhcp::operations::publish_dhcp_logs::Dhcp;

/// This function filter the DHCP logs.
///
/// #Arguments
///
/// *'Dhcp'- A protocol object which contains the path of logs and related commands.
///
/// #Return
///
/// Return the success message or the error given by the compiler.
pub fn filter_dhcp_logs(dhcp: Dhcp) -> String {
    let mut buffer: String = String::new();
    let mut dhcp_file:  String = String::new();
    let mut path:  String = String::new();
    if let Ok(configurations) = dhcp.clone().configuration {
        dhcp_file = configurations.dhcp_log;
        path = configurations.dhcp_file_path;
    }
    let fetched_dhcp_logs: String = fetch_dhcp_logs(dhcp.clone());
    store_in_file(fetched_dhcp_logs, path.clone().as_str());
    match Command::new(COMMAND_PROMPT)
        .args(&[BULLETIN_HELPER, path.clone().as_str(), FILTER_DHCP, STORE, dhcp_file.clone().as_str()])
        .spawn() {
        Ok(_content) => {
            SUCCESS_MESSAGE.to_string();
        },
        Err(error) => {
            error.to_string();
        }
    }
    let dhcp_file_contents = read_to_string(dhcp_file.clone().as_str());
    match dhcp_file_contents {
        Ok(content) => {
            for each_line in content.lines() {
                let dhcp_logs_data: Vec<&str> = each_line.splitn(7, ',').collect();
                buffer.push_str(&dhcp_logs_data[4]);
                buffer.push(' ');
                buffer.push_str(&dhcp_logs_data[5]);
                buffer.push_str("\r\n");
            }
            buffer
        }
        Err(error) => error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::dhcp::operations::filter_dhcp_logs::filter_dhcp_logs;
    use crate::dhcp::operations::publish_dhcp_logs::{Dhcp, Configurations};
    use crate::dhcp::constant::{BULLETIN_HELPER, WINDOWS_BULLETIN, COMMAND_PROMPT};

    static INVALID_LOG_PATH: &str = "test.txt";
    pub static PATH: &str = "C:\\Windows\\System32\\dhcp\\DhcpSrvLog*.log";
    pub static DHCP_LOG_PATH: &str = "C:\\Users\\Administrator\\logs.txt";
    pub static DHCP_LOGS: &str = "C:\\Users\\Administrator\\Dhcp_log.txt";

    #[test]
    fn test_filter_dhcp_logs_success() {
        let dhcp: Dhcp = Dhcp::default();
        assert!(filter_dhcp_logs(dhcp).contains("192.168."));
    }

    #[test]
    fn test_filter_dhcp_logs_path_failure() {
        let dhcp: Dhcp = Dhcp{configuration: Ok( Configurations{ path: PATH.to_string(),
            dhcp_file: DHCP_LOG_PATH.to_string(), command_prompt: COMMAND_PROMPT.to_string(),
            windows_bulletin: WINDOWS_BULLETIN.to_string(), bulletin_helper: BULLETIN_HELPER.to_string(),
            dhcp_file_path: DHCP_LOGS.to_string(), dhcp_log: INVALID_LOG_PATH.to_string()})};
        assert_eq!(filter_dhcp_logs(dhcp),"The system cannot find the file specified. (os error 2)");
    }
}
