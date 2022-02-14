//! Tokenizer Interface

use super::util::{string_span_tokenize, Token};

/// A processing interface for tokenizing a string.
/// must define [Tokenizer::tokenize] or ``tokenize_sents()`` (or both).
pub trait TokenizerI<'a> {
    /// Return a tokenized copy of sent
    fn tokenize(&self, _sent: &'a str) -> Vec<&'a str>;
    ///Identify the tokens using integer offsets [Token],
    /// where ``_sent[start_i:end_i]`` is the corresponding token.
    fn span_tokenize(&self, _sent: &str) -> Vec<Token>;
    /// Apply [TokenizerI::tokenize] to each element of sentence
    fn tokenize_sents(&self, sents: &[&'a str]) -> Vec<Vec<&'a str>> {
        sents.iter().map(|s| self.tokenize(s)).collect()
    }
    ///Apply [TokenizerI::span_tokenize] to each sentence.
    fn span_tokenize_sents(&self, sents: &[&'a str]) -> Vec<Vec<Token>> {
        sents.iter().map(|s| self.span_tokenize(s)).collect()
    }
}

///A tokenizer that divides a string into substrings by splitting on the specified string
pub trait StringTokenizer<'a>: TokenizerI<'a> {
    #[allow(non_upper_case_globals)]
    const _string: &'a str;
    fn tokenize(&self, sent: &'a str) -> Vec<&'a str> {
        sent.split(Self::_string).collect()
    }
    fn span_tokenize(&self, sent: &str) -> Vec<Token> {
        string_span_tokenize(sent, Self::_string).unwrap()
    }
}
