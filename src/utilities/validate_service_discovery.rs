use crate::utilities::service_discovery::discover_service;
use crate::utilities::constant::{INACTIVE_SERVICE, ERROR_MESSAGE, SERVICE_DISCOVERED};

/// This function validate response of service discovery function for protocols.
///
/// #Arguments
///
/// *'protocol_name' - A string literal that can store the protocol name.
/// *'command_prompt' - A string literal that can store the cmd command.
///
/// #Return
///
/// Return the string literal in which it can give the success or error message.
pub fn validate_service_discovery<'a>(protocol: &str, command_prompt: &str) -> &'a str{
    match discover_service(protocol, command_prompt) {
        Ok(response) => {
            if response{
                SERVICE_DISCOVERED
            }
            else {
                INACTIVE_SERVICE
            }
        }
        Err(_error) => {
            ERROR_MESSAGE
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utilities::constant::{INACTIVE_SERVICE, SERVICE_DISCOVERED, ERROR_MESSAGE};
    use crate::dhcp::constant::COMMAND_PROMPT;
    use crate::utilities::validate_service_discovery::validate_service_discovery;

    static DHCP_PROTOCOL: &str = "DHCP Server";
    static INVALID_PROTOCOL: &str = "Internet services";
    static INVALID_COMMAND: &str = "cmmd";

    #[test]
    fn test_validate_service_discovery_success() {
        assert_eq!(validate_service_discovery(DHCP_PROTOCOL,
                                              COMMAND_PROMPT),SERVICE_DISCOVERED);
    }

    #[test]
    fn test_validate_service_discovery_service_failure() {
       assert_eq!(validate_service_discovery(INVALID_PROTOCOL,
                                             COMMAND_PROMPT), INACTIVE_SERVICE);
    }

    #[test]
    fn test_validate_service_discovery_commmand_failure() {
        assert_eq!(validate_service_discovery(DHCP_PROTOCOL,
                                              INVALID_COMMAND), ERROR_MESSAGE);
    }
}
