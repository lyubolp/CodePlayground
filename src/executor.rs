pub mod executor{
    use crate::language::language::{Language};
    use std::fs;
    use crate::config::config::{get_work_dir};
    use crate::program_output::program_output::ProgramOutput;


    pub struct Executor {
        code: String,
        language: Box<dyn Language>
    }

    impl Executor {
        pub fn new(code: &str, platform: Box<dyn Language>) -> Self {
            Executor {
                code: String::from(code),
                language: platform
            }
        }
        pub fn run(&self, code: String) -> Result<ProgramOutput, String> {
            match self.save_code_to_temp_file(code) {
                Ok(code_path) => self.language.run(code_path.as_str()),
                Err(err) => Err(err)
            }
        }

        pub fn update_code(& mut self, new_code: &str) -> &Self {
            if self.code != new_code {
                self.code = String::from(new_code)
            }
            self
        }

        fn save_code_to_temp_file(&self, code: String) -> Result<String, String> {
            let code_file_path: String = String::from(get_work_dir().unwrap() + self.generate_filename().as_str());
            match fs::write(code_file_path.clone(), code.clone()) {
                Ok(_) => Ok(code_file_path.clone()),
                Err(_) => Err(String::from("Can't create file"))
            }
        }


        fn generate_filename(&self) -> String {
            let mut result = String::from("/code");
            result.push_str(self.language.get_language_information().get_file_extension());
            result
        }
    }

}