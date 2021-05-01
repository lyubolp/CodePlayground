pub mod config {
    extern crate yaml_rust;

    use toml::Value;
    use std::fs::read_to_string;
    use toml::de::Error;

    static CONFIG_PATH: &str = "config.toml";

    pub fn get_work_dir() -> Result<String, String> {
        match read_config_file() {
            //TODO - This is not okay, but works for now
            Ok(content) => Ok(String::from(content["config"]["work_dir"].as_str().unwrap())),
            Err(_) => Err(String::from("Can't open config file"))
        }
    }

    pub fn get_platform_executable(language: &str) -> Result<String, String>{
        match get_value_from_category(language, "executable") {
            Ok(value) => Ok(String::from(value.as_str().unwrap())),
            Err(error) => Err(error)
        }
    }

    pub fn get_platform_extension(language: &str) -> Result<String, String> {
        match get_value_from_category(language, "executable") {
            Ok(value) => Ok(String::from(value.as_str().unwrap())),
            Err(error) => Err(error)
        }
    }

    pub fn get_platform_is_compiled(language: &str) -> Result<bool, String> {
        match get_value_from_category(language, "executable") {
            Ok(value) => Ok(value.as_bool().unwrap()),
            Err(error) => Err(error)
        }
    }

    fn get_value_from_category(language: &str, key: &str) -> Result<&Value, String> {
        match read_config_file() {
            Ok(content) => {
                match content.get(language) {
                    Some(language_data) => {
                        match language_data.get(key) {
                            Some(path) =>  Ok(path),
                            None => Err(String::from("Could not find executable key"))
                        }
                    },
                    None => Err(String::from("Language not found !"))
                }
            },
            Err(_) => Err(String::from("Unable to read config file"))
        }
    }

    fn read_config_file() -> Result<Value, Error> {
        let config_content: String = read_to_string(CONFIG_PATH)
            .expect("Something went wrong reading the file");

        config_content.as_str().parse::<Value>()
    }
}