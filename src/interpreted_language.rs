pub mod interpreted_language{
    use crate::language::language::{Language, LanguageInformation};
    use crate::program_output::program_output::ProgramOutput;
    use std::process::Command;

    pub struct InterpretedLanguage{
        info: LanguageInformation
    }

    impl InterpretedLanguage{
        pub fn new(name: &str, executable: &str, file_extension: &str, is_compiled: bool) -> Self {
            InterpretedLanguage {
                info: LanguageInformation::new(name, executable, file_extension, is_compiled)
            }
        }
    }

    impl Language for InterpretedLanguage {
        fn from_config(name: &str) -> Result<Self, String> {
            let language_info_from_config: Result<LanguageInformation, String> = LanguageInformation::from_config(name);

            return match language_info_from_config {
                Ok(info) => Ok(InterpretedLanguage{ info }),
                Err(err) => Err(err)
            }
        }

        fn get_language_information(&self) -> &LanguageInformation {
            &self.info
        }

        fn run(&self, code_path: &str) -> Result<ProgramOutput, String> {
            let executable: &str = self.info.get_executable();
            let output = Command::new(executable)
                .arg(code_path)
                .output();

            match output {
                Ok(process) => {
                    //TODO - Added exit code to the struct
                    Ok(ProgramOutput::new(process.stdout, process.stderr, 0))
                },
                Err(_) => Err(String::from("Can't execute the code"))
            }
        }
    }
}