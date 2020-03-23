use crate::dhcp::constant::{BULLETIN_HELPER, COMMAND_PROMPT, WINDOWS_BULLETIN, DIRECTORY_PATH,
                            DHCP_FILE, DHCP_FILE_PATH, DHCP_LOGS};
use crate::dhcp::operations::fetch_dhcp_logs::fetch_dhcp_logs;
use crate::dhcp::operations::filter_dhcp_logs::filter_dhcp_logs;
use crate::utilities::constant::{ERROR_MESSAGE, INACTIVE_SERVICE, INVALID_MATCH, SERVICE_DISCOVERED,
                                 CONFIG_FILE};
use crate::utilities::configuration::fetch_configuration;
use crate::kafka::producer::{kafka_producer, Kafka};
use crate::ad::constant::CONFIG_ERROR;

/// This structure we use as the object which contains path of DHCP logs and related commands.
#[derive(Clone, Debug)]
pub struct Dhcp{
    pub configuration: Result<Configurations, String>
}

#[derive(Clone, Debug)]
pub struct Configurations {
    pub path: String,
    pub dhcp_file: String,
    pub command_prompt: String,
    pub windows_bulletin: String,
    pub bulletin_helper: String,
    pub dhcp_file_path: String,
    pub dhcp_log: String,
}
impl Default for Dhcp{
    fn default() -> self::Dhcp {
        match fetch_configuration(CONFIG_FILE){
            Ok(configurations) => {
                Dhcp {
                    configuration: Ok(Configurations{
                        path: configurations.get(&DIRECTORY_PATH.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                        dhcp_file: configurations.get(&DHCP_FILE.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                        command_prompt: COMMAND_PROMPT.to_string(),
                        windows_bulletin: WINDOWS_BULLETIN.to_string(),
                        bulletin_helper: BULLETIN_HELPER.to_string(),
                        dhcp_file_path: configurations.get(&DHCP_FILE_PATH.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                        dhcp_log: configurations.get(&DHCP_LOGS.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                    })
                }
            }
            Err(error) => {
                Dhcp{
                    configuration: Err(error.to_string())
                }
            }
        }
    }
}

/// This function compress the DHCP logs.
///
/// #Arguments
///
/// *'with_filter' - A bool value that can define either want filtered logs or raw logs.
/// *'service_response' - A string literal that can contain the response of service discovery.
/// *'dhcp' - A object of DHCP protocol.
/// *'kafka' - A object which contains the socket and topic.
///
/// #Return
///
/// Return the string literal which contains the success or error message.
pub fn publish_dhcp_logs(with_filter: bool, service_response: &str,
                         dhcp: Dhcp, kafka: Kafka) -> String {
    if !with_filter {
        match service_response {
            SERVICE_DISCOVERED => {
                let data: String = fetch_dhcp_logs(dhcp);
                kafka_producer(kafka, data)
            }
            INACTIVE_SERVICE => INACTIVE_SERVICE.to_string(),
            ERROR_MESSAGE => ERROR_MESSAGE.to_string(),
            _ => INVALID_MATCH.to_string()
        }
    } else {
        match service_response {
            SERVICE_DISCOVERED => {
                let data: String = filter_dhcp_logs(dhcp);
                kafka_producer(kafka, data)
            }
            INACTIVE_SERVICE => INACTIVE_SERVICE.to_string(),
            ERROR_MESSAGE => ERROR_MESSAGE.to_string(),
            _ => INVALID_MATCH.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dhcp::operations::publish_dhcp_logs::{publish_dhcp_logs, Dhcp};
    use crate::utilities::constant::{ERROR_MESSAGE, INACTIVE_SERVICE, SERVICE_DISCOVERED};
    use crate::kafka::constant::STREAM_SUCCESS;
    use crate::kafka::producer::Kafka;

    #[test]
    fn test_publish_dhcp_logs_success() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(false, SERVICE_DISCOVERED,
                                     dhcp, kafka), STREAM_SUCCESS);
    }

    #[test]
    fn test_publish_dhcp_logs_service_failure() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(false, INACTIVE_SERVICE,
                                     dhcp, kafka), INACTIVE_SERVICE);
    }

    #[test]
    fn test_publish_dhcp_logs_command_failure() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(false, ERROR_MESSAGE,
                                     dhcp, kafka), ERROR_MESSAGE);
    }

    #[test]
    fn test_publish_filtered_dhcp_logs_success() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(true, SERVICE_DISCOVERED,
                                     dhcp, kafka), STREAM_SUCCESS);
    }

    #[test]
    fn test_publish_filtered_dhcp_logs_service_failure() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(true, INACTIVE_SERVICE,
                                     dhcp, kafka), INACTIVE_SERVICE);
    }

    #[test]
    fn test_publish_filtered_dhcp_logs_command_failure() {
        let dhcp: Dhcp = Dhcp::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_dhcp_logs(true, ERROR_MESSAGE,
                                     dhcp, kafka), ERROR_MESSAGE);
    }

    #[test]
    fn test_dhcp_default_success() {
        let dhcp: Dhcp = Dhcp::default();
        if let Ok(configurations) = dhcp.clone().configuration {
            assert_eq!(configurations.command_prompt, "cmd");
        }
    }
}
