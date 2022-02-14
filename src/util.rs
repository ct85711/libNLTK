//! Natural Language Toolkit: Utility functions
//! Copyright (C) 2001-2022 NLTK Project
//! Author: Steven Bird <stevenbird1@gmail.com>
//!         Eric Kafe <kafe.eric@gmail.com> (acyclic closures)
//! URL: <https://www.nltk.org/>
//! For license information, see LICENSE.TXT

//****************************************************************************************************************************
//  Unused/UnImplemented Fuctions
//****************************************************************************************************************************

/// # UnImplemented!
///
/// Checks if this function is run within an idle thread.
///
/// In Rust, you threads do not contain parent info.
///
/// So any Threading control will be done outside libNLTK
#[allow(dead_code)]
pub fn in_idle() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Given a byte string, attempt to decode it.
/// Tries the standard 'UTF8' and 'latin-1' encodings,
/// Plus several gathered from locale information.
///
/// All text within Rust are UTF-8 encoded, so pointless to check the encoding
#[allow(dead_code)]
pub fn guess_encoding() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Cleaning HTML is to be done outside this crate
#[allow(dead_code)]
pub fn clean_html() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Cleaning URL is to be done outside this crate
#[allow(dead_code)]
pub fn clean_url() -> ! {
    unimplemented!()
}
/// # UnImplemented!
///
/// Threading tasks are to be done outside this crate
#[allow(dead_code)]
pub fn parallelize_preprocess() -> ! {
    unimplemented!()
}

//****************************************************************************************************************************
//
//****************************************************************************************************************************
