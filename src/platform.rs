pub mod platform {
    use crate::config::config::{get_platform_executable, get_platform_extension};

    #[derive(Debug)]
    pub struct Platform {
        name: String,
        executable: String,
        file_extension: String,
        is_compiled: bool
    }

    impl Platform {
        pub fn new(name: &str, executable: &str, file_extension: &str) -> Self {
            Platform {
                name: String::from(name),
                executable: String::from(executable),
                file_extension: String::from(file_extension),
                is_compiled: false
            }
        }

        pub fn from_config(name: &str) -> Result<Self, String> {
            let executable: String = match get_platform_executable(name) {
                Ok(value) => value,
                Err(value) => return Err(value)
            };

            let file_extension = match get_platform_extension(name) {
                Ok(value) => value,
                Err(value) => return Err(value)
            };
            Result::Ok(Platform {
                name: String::from(name),
                executable,
                file_extension
            })
        }

        pub fn get_name(&self) -> &str {
            self.name.as_str()
        }

        pub fn get_executable(&self) -> &str {
            self.executable.as_str()
        }

        pub fn get_file_extension(&self) -> &str {
            self.file_extension.as_str()
        }
    }
}