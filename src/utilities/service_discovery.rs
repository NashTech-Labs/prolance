use std::io::Error;
use std::process::{Command, Output};

use crate::dhcp::constant::BULLETIN_HELPER;
use crate::utilities::constant::SERVER_SERVICES;

/// This function discovers the running services on the server.
///
/// #Arguments
///
/// *'protocol_name' - A string literal that can store the protocol name.
/// *'command_prompt' - A string literal that can store the the cmd command.
///
/// #Return
///
/// Return the result in which it returns either bool or error message.
pub fn discover_service(protocol_name: &str, command_prompt: &str) -> Result<bool, Error> {
    let result: Result<Output, Error> = Command::new(command_prompt)
        .args(&[BULLETIN_HELPER, SERVER_SERVICES])
        .output();
    match result {
        Ok(content) => {
            let services = String::from_utf8_lossy(&content.stdout).to_string();
            Ok(services.contains(protocol_name))
        }
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod test {
    use super::discover_service;
    use crate::dhcp::constant::COMMAND_PROMPT;

    static DHCP_PROTOCOL: &str = "DHCP Server";
    static INVALID_COMMAND: &str = "cmmd";

    #[test]
    fn test_discover_service_success() {
        assert!(discover_service(DHCP_PROTOCOL,
                                 COMMAND_PROMPT).is_ok());
    }

    #[test]
    fn test_discover_service_failure() {
        assert!(discover_service(DHCP_PROTOCOL,
                                 INVALID_COMMAND).is_err());
    }
}
