use std::time::Duration;
use kafka::Error;
use kafka::producer::{Producer, Record, RequiredAcks};
use crate::kafka::constant::{STREAM_SUCCESS, SOCKET, TOPIC_NAME};
use kafka::client::Compression::GZIP;
use crate::utilities::configuration::fetch_configuration;
use crate::utilities::constant::CONFIG_FILE;
use crate::ad::constant::CONFIG_ERROR;

/// This structure we use as the object which contains socket and topic for kafka.
#[derive(Clone, Debug)]
pub struct Kafka{
    pub configuration: Result<Configurations, String>,
}

#[derive(Clone, Debug)]
pub struct Configurations {
    pub socket: String,
    pub topic: String,
}

impl Default for Kafka{
    fn default() -> self::Kafka {
        match fetch_configuration(CONFIG_FILE) {
            Ok(configurations) => {
                Kafka {
                    configuration: Ok(Configurations {
                        socket: configurations.get(&SOCKET.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                        topic: configurations.get(&TOPIC_NAME.to_string())
                            .expect(CONFIG_ERROR).to_string(),
                    })
                }
            }
            Err(error) => {
                Kafka {
                    configuration: Err(error.to_string())
                }
            }
        }
    }
}

/// This function stream the BUFFER to kafka topic.
///
/// #Arguments
///
/// *'kafka' - A protocol object which contains the socket and topic of Kafka.
/// *'buffer' - A string slice that holds data to be stream.
///
/// #Return
///
/// The string which contains the message.
pub fn kafka_producer(kafka: Kafka, buffer: String) -> String {
    match kafka.configuration {
        Ok(configurations) => {
            let producer: Result<Producer, Error> = Producer::from_hosts(
                vec!(configurations.socket))
                .with_ack_timeout(Duration::from_secs(1))
                .with_required_acks(RequiredAcks::One)
                .with_compression(GZIP)
                .create();
            match producer {
                Ok(mut content) =>
                    match content.send(&Record::from_value(configurations.topic.as_str(), buffer)) {
                        Ok(_success) => STREAM_SUCCESS.to_string(),
                        Err(error) => error.to_string(),
                    }
                Err(error) => error.to_string(),
            }
        }
        Err(error) => { eprint!("{}", error.to_string());
            error.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::kafka::constant::STREAM_SUCCESS;
    use super::kafka_producer;
    use crate::kafka::producer::{Kafka, Configurations};

    pub static SOCKET: &str = "localhost:9092";
    pub static TOPIC: &str = "logs";
    static INVALID_TOPIC: &str = "tests";
    static INVALID_SOCKET: &str = "192.168.1.5:9098";
    static TEST_BUFFER: &str = "192.168.1.1";
    static TOPIC_ERROR: &str = "Kafka Error (UnknownTopicOrPartition)";
    static SOCKET_ERROR: &str = "No host reachable";

    #[test]
    fn test_kafka_producer_success() {
        let kafka: Kafka = Kafka::default();
        assert_eq!(kafka_producer(kafka, TEST_BUFFER.to_string()), STREAM_SUCCESS);
    }

    #[test]
    fn test_kafka_producer_invalid_topic() {
        let kafka: Kafka = Kafka{configuration: Ok( Configurations{ socket: SOCKET.to_string(),
            topic: INVALID_TOPIC.to_string()})};
        assert_eq!(kafka_producer(kafka, TEST_BUFFER.to_string()), TOPIC_ERROR);
    }

    #[test]
    fn test_kafka_producer_invalid_socket() {
        let kafka: Kafka = Kafka{configuration: Ok( Configurations{ socket: INVALID_SOCKET.to_string(),
            topic: TOPIC.to_string()})};;
        assert_eq!(kafka_producer(kafka, TEST_BUFFER.to_string()), SOCKET_ERROR);
    }

    #[test]
    fn test_kafka_default_success() {
        let kafka: Kafka = Kafka::default();
        if let Ok(configurations) = kafka.configuration {
            assert_eq!(configurations.topic, "logs");
        }
    }
}
