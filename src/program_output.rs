pub mod program_output {
    #[derive(Debug)]
    pub struct ProgramOutput{
        stdout: String,
        stderr: String,
        exit_code: u32
    }

    impl ProgramOutput {
        pub fn new(out: Vec<u8>, err: Vec<u8>, exit_code: u32) -> ProgramOutput{
            ProgramOutput {
                stdout: String::from_utf8(out).unwrap(),
                stderr: String::from_utf8(err).unwrap(),
                exit_code
            }
        }

        pub fn get_stdout(&self) -> String {
            self.stdout.clone()
        }

        pub fn get_stderr(&self) -> String {
            self.stderr.clone()
        }

        pub fn get_exit_code(&self) -> u32 {
            self.exit_code
        }
    }
}