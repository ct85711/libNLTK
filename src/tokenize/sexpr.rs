//! S-Expression Tokenizer
//!
//! ``SExprTokenizer`` is used to find parenthesized expressions in a
//! string.  In particular, it divides a string into a sequence of
//! substrings that are either parenthesized expressions (including any
//! nested parenthesized expressions), or other whitespace-separated
//! tokens.
//!
//!  **example**
//!
//! By default, `SExprTokenizer` will raise a ``ValueError`` exception if
//! used to tokenize an expression with non-matching parentheses:
//!
//!  **example**
//!
//! The ``strict`` argument can be set to False to allow for
//! non-matching parentheses.  Any unmatched close parentheses will be
//! listed as their own s-expression; and the last partial sexpr with
//! unmatched open parentheses will be listed as its own sexpr:
//!
//!  **example**
//!
//! The characters used for open and close parentheses may be customized
//! using the ``parens`` argument to the `SExprTokenizer` constructor:
//!
//!  **example**
//!
//! The s-expression tokenizer is also available as a function:
//!
//!  **example**

use super::api::TokenizerI;

/// A tokenizer that divides strings into s-expressions.
/// An s-expresion can be either:
///
/// - a parenthesized expression, including any nested parenthesized
///   expressions, or
/// - a sequence of non-whitespace non-parenthesis characters.
///
/// For example, the string ``(a (b c)) d e (f)`` consists of four
/// s-expressions: ``(a (b c))``, ``d``, ``e``, and ``(f)``.
///
/// By default, the characters ``(`` and ``)`` are treated as open and
/// close parentheses, but alternative strings may be specified.
#[derive(Debug)]
pub struct SExprTokenizer<'a> {
    _paren: &'a str,
    _strict: bool,
}
impl<'a> TokenizerI<'a> for SExprTokenizer<'_> {
    /// Return a list of s-expressions extracted from *text*.
    /// For example:
    ///
    ///  **example**
    ///
    /// All parentheses are assumed to mark s-expressions.
    /// (No special processing is done to exclude parentheses that occur
    /// inside strings, or following backslash characters.)
    ///
    /// If the given expression contains non-matching parentheses,
    /// then the behavior of the tokenizer depends on the ``strict``
    /// parameter to the constructor.  If ``strict`` is ``True``, then
    /// raise a ``ValueError``.  If ``strict`` is ``False``, then any
    /// unmatched close parentheses will be listed as their own
    /// s-expression; and the last partial s-expression with unmatched open
    /// parentheses will be listed as its own s-expression:
    ///
    /// **example**
    fn tokenize(&self, _sent: &'a str) -> Vec<&'a str> {
        todo!()
    }

    fn span_tokenize(&self, _sent: &str) -> Vec<super::util::Token> {
        todo!()
    }
}
impl Default for SExprTokenizer<'_> {
    fn default() -> Self {
        SExprTokenizer {
            _paren: r"()",
            _strict: true,
        }
    }
}
