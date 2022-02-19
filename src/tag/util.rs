//! # Natural Language Toolkit: Tagger Utilities
//!
//! Copyright (C) 2001-2022 NLTK Project
//! Author: Edward Loper <edloper@gmail.com>
//!         Steven Bird <stevenbird1@gmail.com>
//! URL: <https://www.nltk.org/>
//! For license information, see LICENSE.TXT

/// Given the string representation of a tagged token, return the
/// corresponding tuple representation.  The rightmost occurrence of
/// *sep* in *s* will be used to divide *s* into a word string and
/// a tag string.  If *sep* does not occur in *s*, return (s, None).
pub fn str2tuple(s: &str, sep: &str) -> (&str, &str) {
    todo!()
}

/// Given the tuple representation of a tagged token, return the
/// corresponding string representation.  This representation is
/// formed by concatenating the token's word string, followed by the
/// separator, followed by the token's tag.  (If the tag is None,
/// then just return the bare word string.)
pub fn tuple2str(token: (&str, &str), sep: &str) -> &str {
    todo!()
}

/// Given a tagged sentence, return an untagged version of that
/// sentence.  I.e., return a list containing the first element
/// of each tuple in *tagged_sentence*.
pub fn untag(token: &(&str, &str)) -> Vec<&str> {
    todo!()
}
