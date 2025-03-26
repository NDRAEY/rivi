use crate::{action::Action, Editor};

pub fn show_help() {
    let entries: &[(&[&str], &str)] = &[
        (&["exit"], "Exit the program."),
        (&["help"], "Show this help message."),
        (&["l [filename]", "load [filename]"], "Load a file from the disk."),
        (&["s", "save"], "Save the current file to the disk."),
        (&["s [filename]", "save [filename]"], "Save data at specified path to the disk."),
        (&["show"], "Show the entire file."),
        (&["showline"], "Show the current line."),
        (&["v="], "Insert a line below the current one."),
        (&["^="], "Insert a line above the current one."),
        (&["="], "Replace the current line with new data."),
        (&["lb"], "Move the cursor to the beginning of the line."),
        (&["le"], "Move the cursor to the end of the line."),
        (&["."], "Inserts data at the current position"),
        (&["u"], "Moves cursor left"),
        (&["i"], "Moves cursor right"),
        (&["o"], "Moves cursor up"),
        (&["p"], "Moves cursor down"),
    ];

    let longest_command_length = {
        let mut result = 0;
        for &(commands, _) in entries {
            for command in commands {
                if command.len() > result {
                    result = command.len();
                }
            }
        }

        result
    };

    for &(commands, help_string) in entries {
        for (index, &command) in commands.iter().enumerate() {
            if index == 0 {
                println!("{command:>longest_command_length$} | {}", help_string);
            } else {
                println!("{command:>longest_command_length$} | ");
            }
        }
    }
}

pub fn execute(commands: Vec<Action>, editor: &mut Editor) -> Result<(), String> {
    for i in commands {
        match i {
            Action::Exit => {
                editor.exit_requested = true;
            },
            Action::Load(path) => {
                if let Err(e) = editor.load_file(path) {
                    return Err(e.to_string());
                }
            },
            Action::ReplaceCurrentLine(text) => {
                editor.editor.delete_line();
                editor.editor.insert_str(&text);
            },
            Action::Save(path) => {
                editor.save_file_as(&path);
            },
            Action::SaveCurrentFile => {
                editor.save_file();
            },
            Action::Show => {
                let lines: Vec<&str> = editor.editor.text().split("\n").collect();
                let lines_count = lines.len();
                let digit_length = format!("{}", lines_count).len();

                for (i, line) in lines.iter().enumerate() {
                    print!("{:>width$} | ", i + 1, width = digit_length);
                    println!("{}", line);
                }
            }
            Action::ShowCurrentLine => {
                let line_nr = editor.editor.cursor().y + 1;
                let digit_length = format!("{}", line_nr).len();

                println!("{} | {}", line_nr, editor.editor.get_line_at_cursor());
                
                let offset = editor.editor.cursor().x;
                
                println!("{}^", str::repeat(" ", offset + digit_length + 3));
            }
            Action::InsertLineBelow => {
                editor.editor.move_to_line_end();
                editor.editor.insert_char('\n');
                editor.editor.move_to_line_begin();
                editor.editor.move_down();
            }
            Action::InsertLineAbove => {
                editor.editor.move_to_line_begin();
                editor.editor.insert_char('\n');
            }
            Action::MoveTo { line, column } => {
                editor.editor.move_to_position(line, column);
            }
            Action::MoveDown => {
                editor.editor.move_down();
            }
            Action::MoveUp => {
                editor.editor.move_up();
            }
            Action::MoveLeft => {
                editor.editor.move_left();
            }
            Action::MoveRight => {
                editor.editor.move_right();
            }
            Action::MoveToBeginning => {
                editor.editor.move_to_line_begin();
            }
            Action::MoveToEnd => {
                editor.editor.move_to_line_end();
            }
            Action::Help => show_help(),
            _ => todo!("Implement {:?}", i)
        }
    }

    Ok(())
}