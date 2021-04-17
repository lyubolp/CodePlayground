pub mod config {
    extern crate yaml_rust;

    use toml::Value;
    use std::fs::read_to_string;
    use toml::de::Error;

    static CONFIG_PATH: &str = "config.toml";


    pub fn get_platform_executable(language: &str) -> Result<String, String>{
        match read_config_file() {
            Ok(content) => {
                match content.get(language) {
                    Some(language_data) => {
                        match language_data.get("executable") {
                            Some(path) =>  Ok(String::from(path.as_str().unwrap())),
                            None => Err(String::from("Could not find executable key"))
                        }
                    },
                    None => Err(String::from("Language not found !"))
                }
            },
            Err(_) => Err(String::from("Unable to read config file"))
        }
    }

    pub fn get_platform_extension(language: &str) -> Result<String, String>{
        match read_config_file() {
            Ok(content) => {
                match content.get(language) {
                    Some(language_data) => {
                        match language_data.get("executable") {
                            Some(path) =>  Ok(String::from(path.as_str().unwrap())),
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