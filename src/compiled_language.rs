pub mod compiled_language {
    use crate::language::language::{Language};
    use crate::program_output::program_output::ProgramOutput;

    pub trait CompiledLanguage: Language {
        fn get_executable_path(&self) -> &str;
        fn compile(&self, code_path: &str) -> Result<ProgramOutput, String>;
    }
}