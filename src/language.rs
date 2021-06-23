pub mod language {
    use crate::config::config::{get_platform_executable, get_platform_extension, get_platform_is_compiled};
    use crate::program_output::program_output::ProgramOutput;
    use std::process::Command;

    pub fn execute_command(command: &str, args: Vec<&str>) -> Result<ProgramOutput, String> {
        let output = Command::new(command)
            .args(args)
            .output();

        match output {
            Ok(process) => {
                //TODO - Added exit code to the struct
                Ok(ProgramOutput::new(process.stdout, process.stderr, 0))
            },
            Err(_) => Err(String::from("Can't execute the command"))
        }
    }

    pub trait Language {
        fn from_config() -> Result<Self, String> where Self: Sized;
        fn get_language_information(&self) -> &LanguageInformation;
        fn run(&self, code_path: &str) -> Result<ProgramOutput, String>;
    }

    #[derive(Debug, Clone)]
    pub struct LanguageInformation {
        name: String,
        executable: String,
        file_extension: String,
        is_compiled: bool
    }

    impl LanguageInformation {
        pub fn from_config(name: &str) -> Result<Self, String> {
            let executable: String = match get_platform_executable(name) {
                Ok(value) => value,
                Err(value) => return Err(value)
            };

            let file_extension = match get_platform_extension(name) {
                Ok(value) => value,
                Err(value) => return Err(value)
            };

            let is_compiled = match get_platform_is_compiled(name) {
                Ok(value) => value,
                Err(value) => return Err(value)
            };

            Result::Ok(LanguageInformation {
                name: String::from(name),
                executable,
                file_extension,
                is_compiled
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

        pub fn get_is_compiled(&self) -> bool {
            self.is_compiled
        }
    }
}