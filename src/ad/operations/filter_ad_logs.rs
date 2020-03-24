use crate::ad::constant::{EVENT, EVENT_DATA, IP_ADDRESS, SERVICE_NAME, TARGET_USERNAME};
use crate::ad::operations::fetch_ad_logs::fetch_ad_logs;
use serde_json::Value;
use crate::ad::operations::publish_ad_logs::ActiveDirectory;

/// This function filter the Active Directory logs.
///
/// #Arguments
///
/// *'file_path' - A string literal that can store the path of raw AD logs which we want to filter.
///
/// #Return
///
/// Return the string of logs.
pub fn filter_ad_logs(file_path: ActiveDirectory) -> String {
    let mut buffer: String = String::new();
    let fetched_ad_logs: Vec<String> = fetch_ad_logs(file_path);
    for value in fetched_ad_logs {
        let json: Value = serde_json::from_str(value.as_str()).unwrap();
        buffer.push_str(json[EVENT][EVENT_DATA][IP_ADDRESS].to_string().as_ref());
        buffer.push_str(json[EVENT][EVENT_DATA][SERVICE_NAME].to_string().as_ref());
        buffer.push_str(json[EVENT][EVENT_DATA][TARGET_USERNAME].to_string().as_ref());
        buffer.push_str("\r\n");
    }
    buffer
}

#[cfg(test)]
mod tests {
    use crate::ad::operations::filter_ad_logs::filter_ad_logs;
    use crate::ad::operations::publish_ad_logs::ActiveDirectory;

    #[test]
    fn test_filter_ad_logs_success() {
        let path: ActiveDirectory = ActiveDirectory::default();
        assert!(filter_ad_logs(path).contains("192.168."));
    }
}