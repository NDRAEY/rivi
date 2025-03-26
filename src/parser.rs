use core::slice::Iter;
use std::io::Cursor;

use crate::action::Action;

pub struct Parser;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedCommand(String),
    TooFewArguments(String),
    InvalidArgument(String)
}

impl Parser {
    fn chop_string(string: &str) -> Vec<String> {
        let mut result = vec![];
        let mut current_command: String = String::new();
        let mut collecting_raw = false;

        for i in string.chars() {
            if !collecting_raw && i == ' ' {
                if !current_command.is_empty() {
                    result.push(current_command.clone());
                    current_command.clear();
                }

                continue;
            }

            if i == '\"' {
                collecting_raw = !collecting_raw;
                continue;
            }

            current_command.push(i);
        }

        if !current_command.is_empty() {
            result.push(current_command.clone());
            current_command.clear();
        }

        result
    }

    pub fn collect_until_end(iterator: &mut Iter<String>) -> String {
        let data = iterator
            .clone()
            .map(String::clone)
            .collect::<Vec<String>>()
            .join(" ");

        for _ in iterator.by_ref() {}

        data
    }

    pub fn parse(command: &str) -> Result<Vec<Action>, ParserError> {
        let mut actions = Vec::new();
        let parts = Self::chop_string(command);
        let mut parts_iter = parts.iter();

        while let Some(i) = &parts_iter.next() {
            match i.as_str() {
                "x" | "exit" => {
                    actions.push(Action::Exit);
                }
                "h" | "help" => {
                    actions.push(Action::Help);
                }
                "l" | "load" => {
                    let name = parts_iter.next();

                    if name.is_none() {
                        return Err(ParserError::TooFewArguments(String::from(*i)));
                    }

                    actions.push(Action::Load(name.unwrap().clone()));
                }
                "s" | "save" => {
                    let name = parts_iter.next();

                    match name {
                        Some(x) => actions.push(Action::Save(x.clone())),
                        None => actions.push(Action::SaveCurrentFile),
                    };
                }
                "=" => {
                    let data = Self::collect_until_end(&mut parts_iter);

                    actions.push(Action::ReplaceCurrentLine(data));
                }
                "@!" | "show" => {
                    actions.push(Action::Show);
                }
                "@" | "showline" => {
                    actions.push(Action::ShowCurrentLine);
                }
                "v=" => {
                    actions.push(Action::InsertLineBelow);
                }
                "^=" => {
                    actions.push(Action::InsertLineAbove);
                }
                "lb" => {
                    // Move cursor to beginning of the line
                    actions.push(Action::MoveToBeginning);
                }
                "le" => {
                    // Move cursor to end of the line
                    actions.push(Action::MoveToEnd);
                }
                "." => {
                    // Inserts data into current position
                    let data = Self::collect_until_end(&mut parts_iter);

                    actions.push(Action::Insert(data));
                }
                "m" | "move" => {
                    let line_number = parts_iter.next().map(|x| x.parse::<usize>());
                    let column_number = parts_iter.next().map(|x| x.parse::<usize>());

                    match (line_number, column_number) {
                        (Some(_line), Some(_column)) => {
                            if let Err(e) = _line {
                                return Err(ParserError::InvalidArgument(String::from(e.to_string())));
                            }

                            if let Err(e) = _column {
                                return Err(ParserError::InvalidArgument(String::from(e.to_string())));
                            }

                            let (line, column) = (_line.unwrap(), _column.unwrap());
                            
                            actions.push(Action::MoveTo{ line, column });
                        }
                        _ => return Err(ParserError::TooFewArguments(String::from(*i))),
                    }
                }
                "u" | "i" | "o" | "p" => {
                    // u - left
                    // i - right
                    // o - up
                    // p - down
                    let direction = match i.as_str() {
                        "u" => Some(Action::MoveLeft),
                        "i" => Some(Action::MoveRight),
                        "o" => Some(Action::MoveUp),
                        "p" => Some(Action::MoveDown),
                        _ => None
                    }.unwrap();

                    actions.push(direction);
                }
                _ => {
                    return Err(ParserError::UnexpectedCommand(String::from(*i)));
                }
            }
        }

        Ok(actions)
    }
}
