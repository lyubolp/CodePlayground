pub mod ui {
    use crate::program_output::program_output::ProgramOutput;

    use gtk::prelude::*;
    use gtk::main_quit;
    use crate::executor::executor::Executor;
    use crate::language::language::Language;

    use crate::languages::python;

    pub struct UI {
        builder: gtk::Builder,
        window: gtk::Window,
        input_field: gtk::TextView,
        output_field: gtk::TextView,
        run_button: gtk::MenuItem,
    }

    // impl UI {
    //     pub fn create() -> UI {
    //         if gtk::init().is_err() {
    //             panic!("Failed to initialize GTK.");
    //         }
    //
    //         let glade_src = include_str!("ui.glade");
    //
    //         let builder: gtk::Builder = gtk::Builder::new_from_string(glade_src);
    //         let result = UI {
    //             builder: builder.clone(),
    //             window: builder.get_object("window").unwrap(),
    //             input_field: builder.get_object("input").unwrap(),
    //             output_field: builder.get_object("output").unwrap(),
    //             run_button: builder.get_object("run").unwrap(),
    //         };
    //
    //         result
    //     }
    //
    //     pub fn main(&self) {
    //         self.run_button.connect_activate(|_| {
    //             let input_buffer = self.input_field.get_buffer().unwrap();
    //
    //             // let input_string: String = input_buffer.get_text(&input_buffer.get_start_iter(),
    //             //                                                  &input_buffer.get_end_iter(),
    //             //                                                  true)
    //             //     .unwrap()
    //             //     .to_string();
    //
    //         });
    //
    //         self.window.connect_delete_event(move |_, _| {
    //             main_quit();
    //             Inhibit(false)
    //         });
    //         self.window.show_all();
    //
    //         gtk::main();
    //     }
    //
    //     fn run(&self) {
    //         let mut executor: Executor = Executor::new("", Box::new(python::python::Python::from_config().unwrap()));
    //         //let input_buffer = self.input_field.get_buffer().unwrap();
    //
    //         // let input_string: String = input_buffer.get_text(&input_buffer.get_start_iter(),
    //         //                                                  &input_buffer.get_end_iter(),
    //         //                                                  true)
    //         //     .unwrap()
    //         //     .to_string();
    //
    //         let input_string: String = String::from("print(5+3)");
    //
    //         println!("text in the input: {}", input_string);
    //
    //         executor.update_code(input_string.as_str());
    //
    //         let program_output: ProgramOutput = executor
    //             .update_code(input_string.as_str())
    //             .run()
    //             .unwrap();
    //
    //         println!("stdout: {}\nstderr:{}", program_output.get_stdout(), program_output.get_stderr());
    //
    //         //self.output_field.get_buffer().unwrap().set_text(program_output.get_stdout().as_str());
    //     }
    // }
    pub fn draw_ui() {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("ui.glade");

        let builder: gtk::Builder = gtk::Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("window").unwrap();
        let input_field: gtk::TextView = builder.get_object("input").unwrap();
        let output_field: gtk::TextView = builder.get_object("output").unwrap();
        let run_button: gtk::MenuItem = builder.get_object("run").unwrap();


        run_button.connect_activate(move |_| {
            let mut executor: Executor = Executor::new("", Box::new(python::python::Python::from_config().unwrap()));
            let input_buffer = input_field.get_buffer().unwrap();

            let input_string: String = input_buffer.get_text(&input_buffer.get_start_iter(),
                                                             &input_buffer.get_end_iter(),
                                                             true)
                .unwrap()
                .to_string();

            println!("text in the input: {}", input_string);

            executor.update_code(input_string.as_str());

            let program_output: ProgramOutput = executor
                .update_code(input_string.as_str())
                .run()
                .unwrap();

            println!("stdout: {}\nstderr:{}", program_output.get_stdout(), program_output.get_stderr());

            output_field.get_buffer().unwrap().set_text(program_output.get_stdout().as_str());
        });

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        window.show_all();

        gtk::main();
    }
}