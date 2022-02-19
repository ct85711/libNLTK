#![warn(missing_docs)]
//! # Metadata
//!
//! The Natural Language Toolkit (NLTK) is an open source Python library
//! for Natural Language Processing.  A free online book is available.
//! (If you use the library for academic research, please cite the book.)
//!
//! Steven Bird, Ewan Klein, and Edward Loper (2009).
//! Natural Language Processing with Python.  O'Reilly Media Inc.
//! <https://www.nltk.org/book/>
//!
//! # Overview
//!
//! This crate is an attempt to porting the python's NLTK in Rust.
//! This is in no way supported by NLTK and is NOT responsible of any damages.
//! Individual characters are going to be in Graphemes segments according
//! to the [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/) rules.
//!
//! # Copyright
//! NLTK is copyrighted and owned by the NLTK project.
//! Distributed and Licensed under the Apache License, Version 2.0.
//! This Library is licensed under NLTK python's License; and Credit for the
//! Various Sections to their respective Authors/Maintainers

extern crate regex;

pub mod collections;
pub mod internals;
pub mod probability;
pub mod tag;
pub mod tokenize;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
