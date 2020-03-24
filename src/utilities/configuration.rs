use std::fs::read_to_string;
use json;
use std::collections::HashMap;

/// This function retrieve the value from the config file.
///
/// #Arguments
///
/// *'config_file_path' - A string literal which can store the path of config file.
///
/// #Return
///
/// Return the value of the key or the error given by the compiler.
pub fn fetch_configuration(config_file_path: &str ) -> Result<HashMap< String, String>, String> {
    let config_path = read_to_string(config_file_path);
    let mut configurations = HashMap::new();
    match config_path {
        Ok(path) => {
            let data = json::parse(path.as_ref());
            match data {
                Ok(values) => {
                    for (key, value) in values.entries(){
                        configurations.insert(key.to_string(),value.as_str().unwrap().to_string());
                    }
                    Ok(configurations)
                }
                Err(error) => Err(error.to_string())
            }
        }
        Err(error) => Err(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::configuration::fetch_configuration;
    use crate::utilities::constant::CONFIG_FILE;
    use crate::ad::constant::CONFIG_ERROR;
    use crate::kafka::constant::TOPIC_NAME;

    static INVALID_CONFIG_FILE_PATH: &str ="configg.txt";

    #[test]
    fn test_variable_value_success() {
        if let Ok(configuration) = fetch_configuration(CONFIG_FILE) {
            assert_eq!(configuration.get(&TOPIC_NAME.to_string()).expect(CONFIG_ERROR), "logs");
        }

    }

    #[test]
    fn test_variable_value_failure() {
        if let Err(error) = fetch_configuration(INVALID_CONFIG_FILE_PATH ){
            assert_eq!(error, "The system cannot find the file specified. (os error 2)");
        }
    }
}