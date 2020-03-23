/// # Static Variables
/// COMPRESSED_AD_FILE - A String slice that contain name of AD file in compressed file.
pub static COMPRESSED_FILE: &str = "logs.txt";

/// COMPRESS_PATH - A String slice that contain path of compress file.
pub static COMPRESS_PATH: &str = "C:\\Users\\Administrator\\Desktop\\logs.zip";

/// SERVER_SERVICES - A String slice that contain command "net start".
pub static SERVER_SERVICES: &str = "net start";

/// FILE_ERROR - error Message folder not created in ZIP.
pub static FILE_ERROR: &str = "File not created.";

/// ZIP_ERROR - error Message ZIP not created.
pub static ZIP_ERROR: &str = "Zip file not created.";

/// SUCCESS - error Message ZIP not created.
pub static SUCCESS: &str = "Logs successfully saved into the file.";

/// SUCCESS_MESSAGE - Success Message after creation of ZIP.
pub static SUCCESS_MESSAGE: &str = "Logs compressed successfully...";

/// INACTIVE_SERVICE - A string slice that contains the message when the service is not running.
pub const INACTIVE_SERVICE: &str = "Service not Active.";

/// ERROR_MESSAGE - A string slice that contains the message when the running services not found.
pub const ERROR_MESSAGE: &str = "We are unable to find the running services.";

/// SERVICE_DISCOVERED - A string slice that contains the message when the service is discovered.
pub const SERVICE_DISCOVERED: &str = "service discovered successfully.";

/// DHCP_PROTOCOL - A static variable that contains the DHCP service name
pub static DHCP_PROTOCOL: &str="DHCP Server";

/// AD_PROTOCOL - A static variable that contains the AD service name
pub static AD_PROTOCOL: &str = "Active Directory Web Services";

/// INVALID_MATCH - A static variable that contains the message if the match exhaust.
pub static INVALID_MATCH: &str = "Match Exhausted.";

/// CONFIG_FILE - A static literal that can store the config file name.
pub static CONFIG_FILE: &str = "config.txt";

// TIMER - A string literal which can store the name TIMER.
pub static TIMER: &str ="TIMER";

// TRUE - A string literal which can store the name TRUE.
pub const TRUE: &str ="true";

// FALSE - A string literal which can store the name FALSE.
pub const FALSE: &str ="false";

// FILTER_STATUS_DHCP - A string literal which can store the name FILTER_STATUS_DHCP.
pub static FILTER_STATUS_DHCP: &str ="FILTER_STATUS_DHCP";

// FILTER_STATUS_AD - A string literal which can store the name FILTER_STATUS_AD.
pub static FILTER_STATUS_AD: &str ="FILTER_STATUS_AD";
