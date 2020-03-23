extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use prolance::ad::constant::CONFIG_ERROR;
use prolance::ad::operations::publish_ad_logs::{ActiveDirectory, publish_ad_logs};
use prolance::dhcp::constant::COMMAND_PROMPT;
use prolance::dhcp::operations::publish_dhcp_logs::{Dhcp, publish_dhcp_logs};
use prolance::kafka::producer::Kafka;
use prolance::utilities::configuration::fetch_configuration;
use prolance::utilities::constant::{AD_PROTOCOL, CONFIG_FILE, DHCP_PROTOCOL, FALSE, FILTER_STATUS_AD,
                                      FILTER_STATUS_DHCP, TIMER, TRUE};
use prolance::utilities::validate_service_discovery::validate_service_discovery;

/// ERROR_MESSAGE - A string literal which can store the error message.
pub static ERROR_MESSAGE: &str = "Please add protocol filter status carefully.";

/// This project is intended to Produce DHCP server logs and AD logs on kafka topic.
fn main() {
    let dhcp: Dhcp = Dhcp::default();
    let ad: ActiveDirectory = ActiveDirectory::default();
    let kafka: Kafka = Kafka::default();
    match fetch_configuration(CONFIG_FILE) {
        Ok(configurations) => {
            let timer: String = configurations.get(&TIMER.to_string())
                .expect(CONFIG_ERROR).to_string();
            let dhcp_filter_status: String = configurations.get(&FILTER_STATUS_DHCP.to_string())
                .expect(CONFIG_ERROR).to_string();
            let ad_filter_status: String = configurations.get(&FILTER_STATUS_AD.to_string())
                .expect(CONFIG_ERROR).to_string();

            let mut scheduler: JobScheduler = JobScheduler::new();
            match (dhcp_filter_status.as_str(), ad_filter_status.as_str()) {
                (TRUE, TRUE) => {
                    scheduler.add(Job::new(timer.parse().unwrap(), || {
                        dbg!(publish_dhcp_logs(true, validate_service_discovery(
                            DHCP_PROTOCOL, COMMAND_PROMPT), dhcp.clone(), kafka.clone()));
                        dbg!(publish_ad_logs(true, validate_service_discovery(
                            AD_PROTOCOL, COMMAND_PROMPT), ad.clone(), kafka.clone()));
                    }));
                }
                (FALSE, TRUE) => {
                    scheduler.add(Job::new(timer.parse().unwrap(), || {
                        dbg!(publish_dhcp_logs(false, validate_service_discovery(
                            DHCP_PROTOCOL, COMMAND_PROMPT), dhcp.clone(), kafka.clone()));
                        dbg!(publish_ad_logs(true, validate_service_discovery(
                            AD_PROTOCOL, COMMAND_PROMPT), ad.clone(), kafka.clone()));
                    }));
                }
                (TRUE, FALSE) => {
                    scheduler.add(Job::new(timer.parse().unwrap(), || {
                        dbg!(publish_dhcp_logs(true, validate_service_discovery(
                            DHCP_PROTOCOL, COMMAND_PROMPT), dhcp.clone(), kafka.clone()));
                        dbg!(publish_ad_logs(false, validate_service_discovery(
                            AD_PROTOCOL, COMMAND_PROMPT), ad.clone(), kafka.clone()));
                    }));
                }
                (FALSE, FALSE) => {
                    scheduler.add(Job::new(timer.parse().unwrap(), || {
                        dbg!(publish_dhcp_logs(false, validate_service_discovery(
                            DHCP_PROTOCOL, COMMAND_PROMPT), dhcp.clone(), kafka.clone()));
                        dbg!(publish_ad_logs(false, validate_service_discovery(
                            AD_PROTOCOL, COMMAND_PROMPT), ad.clone(), kafka.clone()));
                    }));
                }
                _ => {
                    eprint!("{}", ERROR_MESSAGE);
                }
            }
            loop {
                scheduler.tick();
            }
        }
        Err(error) => {
            eprint!("{}", error.to_string());
        }
    }
}
