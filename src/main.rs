use std::io::{self, Write};

use parser::Parser;
use text_editor_foundation::VirtualEditor;

pub mod action;
pub mod executor;
pub mod parser;

pub struct Editor {
    pub editor: VirtualEditor,

    file: Option<String>,
    pub exit_requested: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            editor: VirtualEditor::new(),
            file: None,
            exit_requested: false,
        }
    }

    pub fn load_file(&mut self, file_name: String) -> io::Result<()> {
        let data = std::fs::read_to_string(&file_name)?;
        self.editor = VirtualEditor::with_text(data);

        self.file = Some(file_name);

        Ok(())
    }

    pub fn save_file(&mut self) {
        if let Some(file_name) = &self.file {
            std::fs::write(file_name, self.editor.text()).unwrap()
        }
    }

    pub fn save_file_as(&mut self, file_name: &String) {
        std::fs::write(file_name, self.editor.text()).unwrap()
    }
}

fn main() {
    println!(
        "rivi {}.{}.{}\n",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );

    let mut editor = Editor::new();

    while !editor.exit_requested {
        let mut command = String::new();

        let coords = editor.editor.cursor();

        print!("({}:{}) > ", coords.y + 1, coords.x + 1);

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut command).unwrap();

        let actions = Parser::parse(command.trim_ascii());

        // println!("{:#?}", actions);

        match actions {
            Ok(actions) => {
                if let Err(e) = executor::execute(actions, &mut editor) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }
}
