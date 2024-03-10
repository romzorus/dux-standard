pub mod apt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Module {
    None, // Used for new() methods, initializations and errors
    Apt
}