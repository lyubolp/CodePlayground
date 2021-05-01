pub mod executor{
    use std::fs::{read_to_string, File};
    use crate::platform::platform::Platform;
    use std::fs;
    use std::io::Error;
    use std::process::{Command, Output};
    use crate::config::config::{get_work_dir, get_platform_extension};

    pub struct Executor {
        code: String,
        platform: Platform
    }

    impl Executor {
        pub fn new(code: &str, platform: Platform) -> Self {
            Executor {
                code: String::from(code),
                platform
            }
        }
        pub fn run(&self) -> Result<String, String> {
            match self.save_code_to_temp_file() {
                Ok(code_path) => self.run_code(code_path.as_str()),
                Err(err) => Err(err)
            }
        }

        fn save_code_to_temp_file(&self) -> Result<String, String> {
            let code_file_path: String = String::from(get_work_dir().unwrap() + self.generate_filename().as_str());
            match fs::write(code_file_path.clone(), self.code.clone()) {
                Ok(_) => Ok(code_file_path.clone()),
                Err(_) => Err(String::from("Can't create file"))
            }
        }

        fn run_code(&self, code_path: &str) -> Result<String, String> {
            let executable: &str = self.platform.get_executable();

            let output = Command::new(executable)
                .arg(code_path)
                .output();

            match output {
                Ok(process) => {
                    Ok(String::from_utf8(process.stdout).unwrap())
                },
                Err(_) => Err(String::from("Can't execute the code"))
            }
        }

        fn generate_filename(&self) -> String {
            let mut result = String::from("/code");
            result.push_str(self.platform.get_file_extension());
            result
        }
    }

}