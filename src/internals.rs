//! Natural Language Toolkit: Internal utility functions
//!
//! Copyright (C) 2001-2022 NLTK Project
//! Author: Steven Bird <stevenbird1@gmail.com>
//!         Edward Loper <edloper@gmail.com>
//!         Nitin Madnani <nmadnani@ets.org>
//! URL: <https://www.nltk.org/>
//! For license information, see LICENSE.TXT

//****************************************************************************************************************************
//  Unused/UnImplemented Fuctions
//****************************************************************************************************************************

/// # UnImplemented!
///
/// Java isn't used here!
#[allow(dead_code)]
pub fn config_java() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Java isn't used here!
#[allow(dead_code)]
pub fn java() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Java isn't used here!
#[allow(dead_code)]
pub fn find_jar_iter() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Java isn't used here!
#[allow(dead_code)]
pub fn find_jar() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Java isn't used here!
#[allow(dead_code)]
pub fn find_jars_within_path() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Functions/Methods can NOT be overriden in Rust!
#[allow(dead_code)]
pub fn overridden() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Unused in this crate Rust!
#[allow(dead_code)]
pub fn deprecated() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// All text within Rust are UTF-8 encoded
#[allow(dead_code)]
pub fn _decode_stdoutdata() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Useless within Rust
#[allow(dead_code)]
pub fn import_from_stdlib() -> ! {
    unimplemented!()
}

//****************************************************************************************************************************
//  Error Types
//****************************************************************************************************************************

use std::fmt;

/// Error types used within the Internals module
#[derive(Debug)]
pub enum InternalError {
    /// Exception raised by read_* functions when they fail.
    ReadError,
    ///
    ValueError,
}

impl std::error::Error for InternalError {}
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalError::ReadError => write!(f, "Unexpected Read Error"),
            InternalError::ValueError => write!(f, "Invalid string!"),
        }
    }
}
