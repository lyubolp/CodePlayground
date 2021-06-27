pub mod ui {
    use crate::program_output::program_output::ProgramOutput;

    use gtk::prelude::*;
    use gtk::main_quit;
    use crate::executor::executor::Executor;


    pub fn draw_ui(executor: Executor) {
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
            let input_buffer = input_field.get_buffer().unwrap();

            let input_string: String = input_buffer.get_text(&input_buffer.get_start_iter(),
                                                             &input_buffer.get_end_iter(),
                                                             true)
                .unwrap()
                .to_string();

            println!("text in the input: {}", input_string);

            let program_output: ProgramOutput = executor
                .run(input_string)
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