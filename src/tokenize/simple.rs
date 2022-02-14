//! Simple Tokenizers
//!
//! These tokenizers divide strings into substrings using the string
//! ``split()`` method.
//! When tokenizing using a particular delimiter string, use
//! the string ``split()`` method directly, as this is more efficient.
//!
//! The simple tokenizers are *not* available as separate functions;
//! instead, you should just use the string ``split()`` method directly:
//!
//!    **example**
//!
//! The simple tokenizers are mainly useful because they follow the
//! standard ``TokenizerI`` interface, and so can be used with any code
//! that expects a tokenizer.  For example, these tokenizers can be used
//! to specify the tokenization conventions when building a `CorpusReader`.

use unicode_segmentation::UnicodeSegmentation;

use super::api::{StringTokenizer, TokenizerI};
use super::util::{string_span_tokenize, Token};

/// Tokenize a string using the space character as a delimiter, which is the same as ``s.split(' ')``.
pub struct SpaceTokenizer;
impl StringTokenizer<'_> for SpaceTokenizer {
    #[allow(non_upper_case_globals)]
    const _string: &'static str = " ";
}
impl<'a> TokenizerI<'a> for SpaceTokenizer {
    fn tokenize(&self, sent: &'a str) -> Vec<&'a str> {
        sent.split(Self::_string).collect()
    }

    fn span_tokenize(&self, sent: &str) -> Vec<Token> {
        string_span_tokenize(sent, Self::_string).unwrap()
    }
}

/// Tokenize a string use the tab character as a delimiter, the same as ``s.split('\t')``.
pub struct TabTokenizer;
impl StringTokenizer<'_> for TabTokenizer {
    #[allow(non_upper_case_globals)]
    const _string: &'static str = r"\t";
}
impl<'a> TokenizerI<'a> for TabTokenizer {
    fn tokenize(&self, sent: &'a str) -> Vec<&'a str> {
        sent.split(Self::_string).collect()
    }

    fn span_tokenize(&self, sent: &str) -> Vec<Token> {
        string_span_tokenize(sent, Self::_string).unwrap()
    }
}

/// Tokenize a string into individual characters.  If this functionality is ever required directly, use [char].
/// All characters are assumed to be in UTF-8 encoding as is default in Rust
pub struct CharTokenizer;
impl<'a> TokenizerI<'a> for CharTokenizer {
    /// Split the sentence into individual characters
    /// This is only splitting alphanumeric characters, control characters and emoji's are stripped out
    fn tokenize(&self, sent: &'a str) -> Vec<&'a str> {
        sent.split(char::is_alphanumeric).collect()
    }

    /// Tokenize the individual characters in the string into an Vector of the character ranges
    /// The character ranges are based on the length of the character
    /// All ranges is always be between 1 and 4, inclusive
    fn span_tokenize(&self, sent: &str) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        let mut index = 0;
        for c in sent.graphemes(true) {
            result.push((index, index + c.len()));
            index += c.len();
        }
        result
    }
}

/// Tokenize a string into its lines, optionally discarding blank lines.
/// This is similar to ``s.split('\n')`` and ``s.lines()``.
#[derive(Debug)]
pub struct LineTokenizer;
impl<'a> TokenizerI<'a> for LineTokenizer {
    fn tokenize(&self, sent: &'a str) -> Vec<&'a str> {
        sent.lines().collect()
    }

    fn span_tokenize(&self, sent: &str) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        let mut index = 0;
        for l in sent.lines() {
            result.push((index, index + l.len()));
            index += l.len();
        }
        result
    }
}
