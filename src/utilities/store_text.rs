use std::fs::{OpenOptions, File};
use std::io::{Write, Error};

use crate::utilities::constant::SUCCESS;

/// This function Store the data into text file.
///
/// #Arguments
///
/// *'data' - A string slice that can store the data to be stored.
/// *'path' - A string literal that can store the path where we want to store the data.
///
/// #Return
///
/// Return the success message or the error given by the compiler.
pub fn store_in_file(data: String, path: &str) -> String {
    let file: Result<File,Error> = OpenOptions::new()
        .write(true)
        .open(path);
    match file {
        Ok(mut content) => {
            match write!(content, "{}", data) {
                Ok(_success) => SUCCESS.to_string(),
                Err(error) => error.to_string(),
            }
        },
        Err(error) => error.to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::utilities::constant::SUCCESS;
    use crate::utilities::store_text::store_in_file;

    static CONTENT: &str = "content";
    pub static DHCP_LOGS: &str = "C:\\Users\\Administrator\\Dhcp_log.txt";
    static INVALID_PATH: &str = "C:\\Users\\Administrator\\Filter_logs.txt";

    #[test]
    fn test_store_data_success() {
        assert_eq!(store_in_file(CONTENT.to_string(), DHCP_LOGS), SUCCESS);
    }

    #[test]
    fn test_store_data_path_failure() {
        assert_eq!(store_in_file(CONTENT.to_string(), INVALID_PATH),
                   "The system cannot find the file specified. (os error 2)");
    }
}
