//! Tokenizer Utilities

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use std::fmt;

/// Represets the sequence of a `(starting, ending)` tuple
pub type Token = (usize, usize);

/// Return the offsets of the tokens in *sent*, as a sequence of `(start, end)`
/// tuples, by splitting the string at each occurrence of *sep*.
///
/// Returns Either an Vec<[Token]> or an
///
/// [TokenizeError::ParseError] if unable to parse the string with the given seperator
///
/// [TokenizeError::MismatchError] if [Token] boundary would exceed beyond the input string's length
///
/// # Example
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::string_span_tokenize;
/// let s = "Good muffins cost $3.88\nin New York.  Please buy me two of them.\n\nThanks.";
/// let expected_result = vec![(0, 4), (5, 12), (13, 17), (18, 26), (27, 30), (31, 36), (37, 37),(38, 44), (45, 48), (49, 51), (52, 55), (56, 58), (59, 73)];
/// let result = string_span_tokenize(s, " ").unwrap();
/// # assert_eq!(result,expected_result);
/// ```
pub fn string_span_tokenize(sent: &str, sep: &str) -> Result<Vec<Token>, TokenizeError> {
    let temp = sent.split(sep).collect::<Vec<_>>();
    if temp.is_empty() {
        return Err(TokenizeError::ParseError);
    }
    println!("{:?}", temp);

    let mut result = Vec::new();
    let mut pos: usize = 0;

    for m in temp {
        println!("{}", m);
        let end = pos + m.graphemes(true).count();
        if end > sent.graphemes(true).count() {
            return Err(TokenizeError::MismatchError);
        }
        result.push((pos, end));
        pos += m.graphemes(true).count() + 1;
    }

    Ok(result)
}

/// Return the offsets of the tokens in *sent*, as a sequence of ``(start, end)``
/// tuples, by splitting the string at each successive match of *regexp*.
///
/// # Example
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::regexp_span_tokenize;
/// let sentence = "The plane, bound for St Petersburg, crashed in Egypt's Sinai desert just 23 minutes after take-off from Sharm el-Sheikh on Saturday.";
/// let expected = vec![(0, 3),(4, 10),(11, 16),(17, 20),(21, 23),(24, 35),(36, 43),(44, 46),(47, 54),(55, 60),(61, 67),(68, 72),(73, 75),(76, 83),(84, 89),(90, 98),(99, 103),(104, 109),(110, 119),(120, 122),(123, 132)];
/// let result = regexp_span_tokenize(sentence, r"\s+");
/// # assert_eq!(result,expected);
/// ```
pub fn regexp_span_tokenize(sent: &str, regexp: &str) -> Vec<Token> {
    let re = Regex::new(regexp).unwrap();
    let mut result: Vec<Token> = Vec::new();
    let mut index: usize = 0;
    let matches = re.find_iter(sent);
    for m in matches {
        result.push((index, m.start()));
        index = m.end();
    }
    result.push((index, sent.len()));

    result
}
#[test]
fn test_regexp_span_tokenize() {
    let expected = vec![
        (0, 4),
        (5, 12),
        (13, 17),
        (18, 23),
        (24, 26),
        (27, 30),
        (31, 36),
        (38, 44),
        (45, 48),
        (49, 51),
        (52, 55),
        (56, 58),
        (59, 64),
        (66, 73),
    ];
    let sentence = "Good muffins cost $3.88\nin New York.  Please buy me two of them.\n\nThanks.";
    let result = regexp_span_tokenize(sentence, r"\s+");
    assert_eq!(result, expected);
}

/// Return a sequence of relative spans, given a sequence of spans.
///
/// # Example
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::regexp_span_tokenize;
/// # use lib_nltk::tokenize::util::spans_to_relative;
/// let sentence = "Good muffins cost $3.88\nin New York.  Please buy me two of them.\n\nThanks.";
/// let expected = vec![(0, 4), (1, 7), (1, 4), (1, 5), (1, 2), (1, 3), (1, 5), (2, 6),(1, 3), (1, 2), (1, 3), (1, 2), (1, 5), (2, 7)];
/// let result = spans_to_relative(regexp_span_tokenize(sentence, r"\s").as_ref());
/// # assert_eq!(result,expected);
/// ```
pub fn spans_to_relative(token_span: &[Token]) -> Vec<Token> {
    let mut prev: usize = 0;
    let mut result: Vec<Token> = Vec::new();
    //had an situation in that the input vector has entrys that the tuple was empty/both same number
    //even though it the tests ran correctly so we are just going to skip over any empty entry
    for (left, right) in token_span {
        if left == right {
            continue;
        }
        result.push((left - prev, right - left));
        prev = *right;
        println!("left {}, right {}, prev {}", left, right, prev);
    }

    result
}

/// Python port of Moses' code to check for CJK character.
/// In internationalization, CJK characters is a collective term for the Chinese, Japanese, and Korean languages,
/// all of which include Chinese characters and derivatives in their writing systems, sometimes paired with other scripts.
///
///  # Examples
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::is_cjk;
/// let result = is_cjk('\u{33fe}');
/// assert!(result);
/// ```
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::is_cjk;
/// let result = is_cjk('G');
/// assert_eq!(result,false);
/// ```
pub fn is_cjk(a_char: char) -> bool {
    matches!(a_char as u32, 4352..=4607 | 11904..=42191 | 43072..=43135 | 44032..=55215 | 63744..=64255 | 65072..=65103 | 65381..=65500 | 131072..=196607)
}

//Some additional tests on is_cjk to check some of the boundaries on the ranges; to make sure everything was correctly pointed.
#[test]
fn test_is_cjk() {
    assert!(!is_cjk(' '));
    assert!(is_cjk('\u{AC00}'));
    assert!(!is_cjk('\u{FE2F}'));
    assert!(is_cjk('\u{04CD3}'));
    assert!(!is_cjk('\u{0A880}'));
}

/// This function transforms the input text into an "escaped" version suitable
/// for well-formed XML formatting.
///
/// ## Escaped characters:
/// "'": "&apos;",
/// '"': "&quot;",
/// "|": "&#124;",
/// "[": "&#91;",
/// "]": "&#93;",
/// "&", "&amp;"
/// ">", "&gt;"
/// "<", "&lt;"
///
/// # Example
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::xml_escape;
/// let s = r#")| & < > ' " ] ["#;
/// # let expected_result = r")&#124; &amp; &lt; &gt; &apos; &quot; &#93; &#91;";
/// let result = xml_escape(s);
/// # assert_eq!(result,expected_result);
/// ```

pub fn xml_escape(text: &str) -> String {
    //Probably have a better way, but this should work for now
    text.replace('&', r"&amp;")
        .replace('>', r"&gt;")
        .replace('\'', r"&apos;")
        .replace('"', r"&quot;")
        .replace('|', r"&#124;")
        .replace('[', r"&#91;")
        .replace(']', r"&#93;")
        .replace('<', r"&lt;")
}

/// This function transforms the "escaped" version suitable
/// for well-formed XML formatting into humanly-readable string.
///
/// ## Escaped characters:
/// "'": "&apos;",
/// '"': "&quot;",
/// "|": "&#124;",
/// "[": "&#91;",
/// "]": "&#93;",
/// "&", "&amp;"
/// ">", "&gt;"
/// "<", "&lt;"
///
/// # Example
///
/// ```rust
/// # extern crate lib_nltk;
/// # use lib_nltk::tokenize::util::xml_unescape;
/// let s = r")&#124; &amp; &lt; &gt; &apos; &quot; &#93; &#91;";
/// # let expected_result = r#")| & < > ' " ] ["#;
/// let result = xml_unescape(s);
/// # assert_eq!(result,expected_result);
/// ```
pub fn xml_unescape(text: &str) -> String {
    //Probably have a better way, but this should work for now
    text.replace(r"&amp;", "&")
        .replace(r"&gt;", ">")
        .replace(r"&apos;", "'")
        .replace(r"&quot;", "\"")
        .replace(r"&#124;", "|")
        .replace(r"&#91;", "[")
        .replace(r"&#93;", "]")
        .replace(r"&lt;", "<")
}

/// This module attempt to find the offsets of the tokens in *sent*, as a sequence
/// of ``(start, end)`` tuples, given the tokens and the source string.
///
/// Returns Either an Vec<[Token]> or a [TokenizeError::MismatchError]
///
/// # Example
///
/// ```rust
/// # extern crate  lib_nltk;
/// # use lib_nltk::tokenize::util::align_tokens;
/// let sent = "Hello World";
/// let token_list = vec!("Hello","World");
/// let result = align_tokens(token_list.clone(),sent).unwrap();
/// let expected = vec![(0,5),(6,11)];
/// # //ensure get tok the same number of tokens
/// assert_eq!(token_list.len(),result.len());
/// assert_eq!(result,expected);
/// ```
pub fn align_tokens(tokens: Vec<&str>, _sent: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut token_span = Vec::new();
    let mut sentence = String::from(_sent);
    let mut index: usize = 0;
    //make sure the tokens list assembled together matches the original string
    //if this doesn't work, trying to determine the token spans isn't going to work either
    if tokens.join(" ") != _sent {
        return Err(TokenizeError::MismatchError);
    }

    for word in tokens {
        let wsize = word.graphemes(true).count();
        token_span.push((index, index + wsize));
        sentence = sentence.split_off(word.len());
        index += wsize + 1;
    }
    Ok(token_span)
}

// An additional test on the align_tokens method, giving it a much longer input string.
#[test]
fn test_align_tokens() {
    let sentence = "The plane, bound for St Petersburg, crashed in Egypt's Sinai desert just 23 minutes after take-off from Sharm el-Sheikh on Saturday.";
    let expected = vec![
        (0, 3),
        (4, 10),
        (11, 16),
        (17, 20),
        (21, 23),
        (24, 35),
        (36, 43),
        (44, 46),
        (47, 54),
        (55, 60),
        (61, 67),
        (68, 72),
        (73, 75),
        (76, 83),
        (84, 89),
        (90, 98),
        (99, 103),
        (104, 109),
        (110, 119),
        (120, 122),
        (123, 132),
    ];
    let result = align_tokens(sentence.split_whitespace().collect(), sentence).unwrap();
    assert_eq!(result, expected);
}

//****************************************************************************************************************************
//  Error Types
//****************************************************************************************************************************

/// Error types used within the Tokenize module
#[derive(Debug)]
pub enum TokenizeError {
    /// Indicates there's an issue with athe parsing string
    ParseError,
    /// Indicates an issue where the input Vector and input String mismatch resulting in a case,
    /// there's no way the output can ever be valid
    MismatchError,
}

impl std::error::Error for TokenizeError {}
impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenizeError::ParseError => write!(f, "Parsing Error"),
            TokenizeError::MismatchError => write!(f, "Mismatch Length Error"),
        }
    }
}
