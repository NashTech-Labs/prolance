use evtx::EvtxParser;
use serde_json::Value;
use std::fs::File;
use crate::ad::operations::publish_ad_logs::ActiveDirectory;

use crate::ad::constant::{BLANK_IP, EVENT, EVENT_DATA, EVENT_ID, IP_ADDRESS, SERVICE_NAME,
                          SERVICE_NAME1, SERVICE_NAME2, SYSTEM, AD_EVENT_ID};
use evtx::err::Error;

/// This function fetch the Active directory logs.
///
/// #Arguments
///
/// *'active_directory' - A protocol object which contains the path of the logs.
///
/// #Return
///
/// Return the Vector of logs or error message.
pub fn fetch_ad_logs(active_directory: ActiveDirectory) -> Vec<String> {
    let mut response: Vec<String> = Vec::new();
    match  active_directory.ad_logs {
        Ok(path) => {
            let parser: Result<EvtxParser<File>, Error> = EvtxParser::from_path(path.log_path);
            match parser {
                Ok(mut records) => {
                    for record in records.records_json() {
                        let data: Value = serde_json::from_str(&record.unwrap().data).unwrap();
                        if data[EVENT][SYSTEM][EVENT_ID] == AD_EVENT_ID &&
                            data[EVENT][EVENT_DATA][IP_ADDRESS] != BLANK_IP &&
                            data[EVENT][EVENT_DATA][SERVICE_NAME] != SERVICE_NAME1 &&
                            data[EVENT][EVENT_DATA][SERVICE_NAME] != SERVICE_NAME2 {
                            response.push(data.to_string());
                        }
                    }
                    response
                }
                Err(error) => {
                    eprint!("{}", error.to_string());
                    response.push(error.to_string());
                    response
                }
            }
        }
        Err(error) => { eprint!("{}", error.to_string());
            response.push(error.to_string());
            response
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ad::operations::fetch_ad_logs::fetch_ad_logs;
    use crate::ad::constant::{IP_ADDRESS, EVENT_DATA, EVENT};
    use serde_json::Value;
    use crate::ad::operations::publish_ad_logs::{ActiveDirectory, Path};

    pub static INVALID_PATH: &str = "securityy.evtx";

    #[test]
    fn test_fetch_ad_logs_success() {
        let ad = ActiveDirectory { ..Default::default()};
        let logs: Vec<String> = fetch_ad_logs(ad);
        for value in logs {
            let json: Value = serde_json::from_str(value.as_str()).unwrap();
            assert!(json[EVENT][EVENT_DATA][IP_ADDRESS].to_string()
                .contains("::ffff:192.168."));
        }
    }

    #[test]
    fn test_fetch_ad_logs_failure() {
        let ad = ActiveDirectory { ad_logs: Ok(Path { log_path: INVALID_PATH.to_string() })};
        let logs: Vec<String> = fetch_ad_logs(ad);
        assert!(logs[0].contains("The system cannot find the file specified."));
    }
}
