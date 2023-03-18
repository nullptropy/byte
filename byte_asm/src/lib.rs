#![allow(dead_code, unused_imports, unused_variables)]

pub mod lexer;
pub mod error;

pub type ScannerResult<T> = Result<T, error::ScannerError>;