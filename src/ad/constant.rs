/// # Static Variables
/// EVENT - to parse the json file.
pub static EVENT: &str = "Event";

/// SYSTEM - to parse the json file.
pub static SYSTEM: &str = "System";

/// EVENT_ID - to parse the json file.
pub static EVENT_ID: &str = "EventID";

/// EVENT_DATA - to parse the json file.
pub static EVENT_DATA: &str = "EventData";

/// IP_ADDRESS - to parse the json file.
pub static IP_ADDRESS: &str = "IpAddress";

/// SERVICE_NAME - to parse the json file.
pub static SERVICE_NAME: &str = "ServiceName";

/// TARGET_USERNAME - to parse the json file.
pub static TARGET_USERNAME: &str = "TargetUserName";

/// SERVICE_NAME1 - A String slice that contains "PANKAJ$".
pub static SERVICE_NAME1: &str = "PANKAJ$";

/// SERVICE_NAME2 - A String slice that contains "krbtgt".
pub static SERVICE_NAME2: &str = "krbtgt";

/// BLANK_IP - A String slice that contains "::1".
pub static BLANK_IP: &str = "::1";

/// AD_EVENT_ID - A const which contains the event id of Active directory logs.
pub const AD_EVENT_ID: i32 = 4769;

/// CONFIG_ERROR - error Message if the defined key not available in the config file.
pub static CONFIG_ERROR: &str = "Key not present in the config file.";

/// AD_LOGS_PATH - A string literal which can store the name of Active directory log.
pub static AD_LOGS_PATH: &str = "AD_LOGS_PATH";
