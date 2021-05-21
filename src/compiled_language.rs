pub mod compiled_language {
    use crate::language::language::{LanguageInformation, Language};
    use crate::program_output::program_output::ProgramOutput;
    use std::process::Command;

    pub struct CompiledLanguage{
        info: LanguageInformation
    }

    impl CompiledLanguage {
        fn compile(&self, code_path: &str) -> Result<String, String> {
            let executable: &str = self.info.get_executable();
            let output = Command::new(executable)
                .arg(code_path)
                .output();

            match output {
                //TODO - This will only work for C++
                Ok(process) => Ok(String::from("a.out")),
                Err(_) => Err(String::from("Can't compile the code"))
            }
        }
    }
    impl Language for CompiledLanguage {
        fn from_config(name: &str) -> Result<Self, String> where Self: Sized {
            let language_info_from_config: Result<LanguageInformation, String> = LanguageInformation::from_config(name);

            return match language_info_from_config {
                Ok(info) => Ok(CompiledLanguage{ info }),
                Err(err) => Err(err)
            }
        }

        fn get_language_information(&self) -> &LanguageInformation {
            &self.info
        }

        fn run(&self, code_path: &str) -> Result<ProgramOutput, String> {
            //TODO - Bad code below
            let executable: String = self.compile(code_path).unwrap();
            let output = Command::new(executable.as_str())
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