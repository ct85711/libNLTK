//! Natural Language Toolkit: NLTK's very own tokenizer.
//!
//! Copyright (C) 2001-2022 NLTK Project
//! Author: Liling Tan
//!         Tom Aarsen <> (modifications)
//! URL: <https://www.nltk.org>
//! For license information, see LICENSE.TXT

use super::{api::TokenizerI, util::Token};

use lazy_static::lazy_static;
use regex::RegexSet;

// MacIntyreContractions
//List of contractions adapted from Robert MacIntyre's tokenizer.
lazy_static! {
    static ref CONTRACTIONS2: RegexSet = RegexSet::new(&[
        r"(?i)\b(can)(not)\b",
        r"(?i)\b(d)('ye)\b",
        r"(?i)\b(gim)(me)\b",
        r"(?i)\b(gon)(na)\b",
        r"(?i)\b(got)(ta)\b",
        r"(?i)\b(lem)(me)\b",
        r"(?i)\b(more)('n)\b",
        r"(?i)\b(wan)(na)\b"
    ])
    .unwrap();
    static ref CONTRACTIONS3: RegexSet =
        RegexSet::new(&[r"(?i) ('t)(is)\b", r"(?i) ('t)(was)\b"]).unwrap();
    static ref CONTRACTIONS4: RegexSet =
        RegexSet::new(&[r"(?i)\b(whad)(dd)(ya)\b", r"(?i)\b(wha)(t)(cha)\b"]).unwrap();
}

/// The NLTK tokenizer that has improved upon the TreebankWordTokenizer.
///
/// This is the method that is invoked by ``word_tokenize()``.  It assumes that the
/// text has already been segmented into sentences, e.g. using ``sent_tokenize()``.
///
/// The tokenizer is "destructive" such that the regexes applied will munge the
/// input string to a state beyond re-construction. It is possible to apply
/// `TreebankWordDetokenizer.detokenize` to the tokenized outputs of
/// `NLTKDestructiveWordTokenizer.tokenize` but there's no guarantees to
/// revert to the original string.
pub struct NLTKWordTokenizer;
impl<'a> TokenizerI<'a> for NLTKWordTokenizer {
    /// Return a tokenized copy of `text`.
    fn tokenize(&self, _sent: &'a str) -> Vec<&'a str> {
        todo!()
    }

    /// Returns the spans of the tokens in ``text``.
    /// Uses the post-hoc nltk.tokens.align_tokens to return the offset spans.
    fn span_tokenize(&self, _sent: &str) -> Vec<Token> {
        todo!()
    }
}
