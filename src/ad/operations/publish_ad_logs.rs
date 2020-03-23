use std::str;
use crate::ad::operations::fetch_ad_logs::fetch_ad_logs;
use crate::ad::operations::filter_ad_logs::filter_ad_logs;

use crate::utilities::constant::{ERROR_MESSAGE, INACTIVE_SERVICE, INVALID_MATCH, SERVICE_DISCOVERED,
                                 CONFIG_FILE};
use crate::utilities::configuration::fetch_configuration;
use crate::kafka::producer::{kafka_producer, Kafka};
use crate::ad::constant::{AD_LOGS_PATH, CONFIG_ERROR};

/// This structure we use as the object which contains path of Active Directory logs.
#[derive(Clone, Debug)]
pub struct ActiveDirectory {
    pub ad_logs: Result<Path, String>,
}

#[derive(Clone, Debug)]
pub struct Path {
    pub log_path: String,
}

impl Default for ActiveDirectory {
    fn default() -> self::ActiveDirectory {
        match fetch_configuration(CONFIG_FILE){
            Ok(configurations) => {
                ActiveDirectory {
                    ad_logs: Ok(Path {
                        log_path: configurations.get(&AD_LOGS_PATH.to_string())
                            .expect(CONFIG_ERROR).to_string()
                    })
                }
            }
            Err(error) => {
                ActiveDirectory {
                    ad_logs: Err(error.to_string())
                }
            }
        }
    }
}

/// This function compress the Active Directory logs.
///
/// #Arguments
///
/// *'with_filter' - A bool value that can define either want filtered logs or raw logs.
/// *'service_response' - A string literal that can contain the response of service discovery.
/// *'file_path' - A object of DHCP protocol.
/// *'kafka' - A object which contains the socket and topic.
///
/// #Return
///
/// Return the string literal which contains the success or error message.
pub fn publish_ad_logs(with_filter: bool, service_response: &str,
                       file_path: ActiveDirectory, kafka: Kafka) -> String {
    if !with_filter {
        match service_response {
            SERVICE_DISCOVERED => {
                let data: Vec<String> = fetch_ad_logs(file_path);
                kafka_producer(kafka, data.into_iter().collect())
            }
            INACTIVE_SERVICE => INACTIVE_SERVICE.to_string(),
            ERROR_MESSAGE => ERROR_MESSAGE.to_string(),
            _ => INVALID_MATCH.to_string()
        }
    } else {
        match service_response {
            SERVICE_DISCOVERED => {
                let data: String = filter_ad_logs(file_path);
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
    use crate::ad::operations::publish_ad_logs::{ActiveDirectory, publish_ad_logs};
    use crate::utilities::constant::{ERROR_MESSAGE, INACTIVE_SERVICE, SERVICE_DISCOVERED};
    use crate::kafka::constant::STREAM_SUCCESS;
    use crate::kafka::producer::Kafka;

    pub static LOG_FILE: &str = "C:\\Windows\\System32\\winevt\\Logs\\Security.evtx";

    #[test]
    fn test_publish_ad_logs_success() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(false, SERVICE_DISCOVERED,
                                   ad, kafka), STREAM_SUCCESS);
    }

    #[test]
    fn test_publish_ad_logs_service_failure() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(false, INACTIVE_SERVICE,
                                   ad, kafka), INACTIVE_SERVICE);
    }

    #[test]
    fn test_publish_ad_logs_command_failure() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(false, ERROR_MESSAGE,
                                   ad, kafka), ERROR_MESSAGE);
    }

    #[test]
    fn test_publish_filtered_ad_logs_success() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(true, SERVICE_DISCOVERED,
                                   ad, kafka), STREAM_SUCCESS);
    }

    #[test]
    fn test_publish_filtered_ad_logs_service_failure() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(true, INACTIVE_SERVICE,
                                   ad, kafka), INACTIVE_SERVICE);
    }

    #[test]
    fn test_publish_filtered_ad_logs_command_failure() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        let kafka: Kafka = Kafka::default();
        assert_eq!(publish_ad_logs(true, ERROR_MESSAGE,
                                   ad, kafka), ERROR_MESSAGE);
    }

    #[test]
    fn test_ad_default_success() {
        let ad: ActiveDirectory = ActiveDirectory::default();
        if let Ok(path) = ad.ad_logs {
            assert_eq!(path.log_path, LOG_FILE);
        }
    }
}
