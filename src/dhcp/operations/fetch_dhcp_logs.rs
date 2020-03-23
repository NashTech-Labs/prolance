use std::process::{Command, Output};
use crate::dhcp::operations::publish_dhcp_logs::Dhcp;
use std::io::Error;

/// This function fetch DHCP logs.
///
/// #Arguments
///
/// *'Dhcp'- A protocol object which contains the path of logs in the commands.
///
/// #Return
///
/// Return the string which contain either logs or the error message.
pub fn fetch_dhcp_logs(dhcp: Dhcp) -> String {
    match dhcp.configuration {
        Ok(configuration) => {
            let result: Result<Output, Error> = Command::new(configuration.command_prompt)
                .args(&[configuration.bulletin_helper, configuration.windows_bulletin, configuration.path])
                .output();
            match result {
                Ok(response) => {
                    String::from_utf8_lossy(&response.stdout).to_string()
                }
                Err(error) => error.to_string()
            }
        }
        Err(error) => { eprint!("{}", error.to_string());
            error.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dhcp::operations::publish_dhcp_logs::{Dhcp, Configurations};

    use super::fetch_dhcp_logs;
    use crate::dhcp::constant::{WINDOWS_BULLETIN, BULLETIN_HELPER};

    static INVALID_COMMAND: &str = "cmmd";
    pub static PATH: &str = "C:\\Windows\\System32\\dhcp\\DhcpSrvLog*.log";
    pub static DHCP_LOG_PATH: &str = "C:\\Users\\Administrator\\logs.txt";
    pub static DHCP_LOGS: &str = "C:\\Users\\Administrator\\Dhcp_log.txt";
    pub static DHCP_FILE_PATH: &str = "type C:\\Users\\Administrator\\logs.txt";

    #[test]
    fn test_fetch_dhcp_logs_success() {
        let dhcp: Dhcp = Dhcp::default();
        assert!(fetch_dhcp_logs(dhcp).contains("Microsoft DHCP Service Activity Log"));
    }

    #[test]
    fn test_fetch_dhcp_logs_invalid_command() {
        let dhcp: Dhcp = Dhcp{configuration: Ok( Configurations{ path: PATH.to_string(),
            dhcp_file: DHCP_LOG_PATH.to_string(), command_prompt: INVALID_COMMAND.to_string(),
            windows_bulletin: WINDOWS_BULLETIN.to_string(), bulletin_helper: BULLETIN_HELPER.to_string(),
            dhcp_file_path: DHCP_LOGS.to_string(), dhcp_log: DHCP_FILE_PATH.to_string()})};
        assert_eq!(fetch_dhcp_logs(dhcp),
                   "The system cannot find the file specified. (os error 2)".to_string());
    }
}
