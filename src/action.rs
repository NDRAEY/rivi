use std::any::Any;

#[derive(Debug)]
pub enum Action {
    Show,
    ShowCurrentLine,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveToBeginning,
    MoveToEnd,
    MoveTo { line: usize, column: usize },
    Insert(String),   // Content,
    InsertLineBelow,
    InsertLineAbove,
    RemoveCharLeft,
    RemoveCharRight,
    ReplaceCurrentLine(String),    // Content
    Load(String),    // File path
    SaveCurrentFile,
    Save(String),    // File path
    Help,
    Exit
}