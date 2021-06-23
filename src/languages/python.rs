pub mod python {
    use crate::language::language::{LanguageInformation, Language, execute_command};
    use crate::interpreted_language::interpreted_language::InterpretedLanguage;
    use crate::program_output::program_output::ProgramOutput;

    pub struct Python{
        info: LanguageInformation
    }

    impl Language for Python {
        fn from_config() -> Result<Self, String> where Self: Sized {
            let language_info_from_config: Result<LanguageInformation, String> = LanguageInformation::from_config("python3");

            return match language_info_from_config {
                Ok(info) => Ok(Python{ info }),
                Err(err) => Err(err)
            }
        }

        fn get_language_information(&self) -> &LanguageInformation {
            &self.info
        }

        fn run(&self, code_path: &str) -> Result<ProgramOutput, String> {
            execute_command(self.info.get_executable(), vec![code_path])
        }
    }
    impl InterpretedLanguage for Python {

    }
}