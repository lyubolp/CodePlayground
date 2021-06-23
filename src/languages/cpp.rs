pub mod cpp {
    use crate::language::language::{LanguageInformation, Language, execute_command};
    use crate::program_output::program_output::ProgramOutput;
    use crate::compiled_language::compiled_language::CompiledLanguage;
    use crate::config::config::get_work_dir;

    pub struct Cpp {
        info: LanguageInformation,
        executable_path: String,
    }

    impl Language for Cpp {
        fn from_config() -> Result<Self, String> where Self: Sized {
            let language_info_from_config: Result<LanguageInformation, String> = LanguageInformation::from_config("cpp");

            let executable_path: String = get_work_dir().unwrap() + "/a.out";
            return match language_info_from_config {
                Ok(info) => Ok(
                    Cpp {
                        info,
                        executable_path,
                    }),
                Err(err) => Err(err)
            };
        }

        fn get_language_information(&self) -> &LanguageInformation {
            &self.info
        }

        fn run(&self, code_path: &str) -> Result<ProgramOutput, String> {
            match self.compile(code_path) {
                Ok(_) => execute_command(self.get_executable_path(), vec!()),
                Err(err) => Err(err)
            }
        }
    }

    impl CompiledLanguage for Cpp {
        fn get_executable_path(&self) -> &str {
            self.executable_path.as_str()
        }

        fn compile(&self, code_path: &str) -> Result<ProgramOutput, String> {
            execute_command(self.get_language_information().get_executable(),
                            vec![code_path,
                                 "-o",
                                 self.get_executable_path()])
        }
    }
}