//! The Legality Principle is a language agnostic principle maintaining that syllable
//! onsets and codas (the beginning and ends of syllables not including the vowel)
//! are only legal if they are found as word onsets or codas in the language. The English
//! word ''admit'' must then be syllabified as ''ad-mit'' since ''dm'' is not found
//! word-initially in the English language (Bartlett et al.). This principle was first proposed
//! in Daniel Kahn's 1976 dissertation, ''Syllable-based generalizations in English phonology''.
//!
//! Kahn further argues that there is a ''strong tendency to syllabify in such a way that
//! initial clusters are of maximal length, consistent with the general constraints on
//! word-initial consonant clusters.'' Consequently, in addition to being legal onsets,
//! the longest legal onset is preferable---''Onset Maximization''.
//!
//! The default implementation assumes an English vowel set, but the `vowels` attribute
//! can be set to IPA or any other alphabet's vowel set for the use-case.
//! Both a valid set of vowels as well as a text corpus of words in the language
//! are necessary to determine legal onsets and subsequently syllabify words.
//!
//! The legality principle with onset maximization is a universal syllabification algorithm,
//! but that does not mean it performs equally across languages. Bartlett et al. (2009)
//! is a good benchmark for English accuracy if utilizing IPA (pg. 311).
//!
//! # References:
//!
//! - Otto Jespersen. 1904. Lehrbuch der Phonetik.
//!   Leipzig, Teubner. Chapter 13, Silbe, pp. 185-203.
//! - Theo Vennemann, ''On the Theory of Syllabic Phonology,'' 1972, p. 11.
//! - Daniel Kahn, ''Syllable-based generalizations in English phonology'', (PhD diss., MIT, 1976).
//! - Elisabeth Selkirk. 1984. On the major class features and syllable theory.
//!   In Aronoff & Oehrle (eds.) Language Sound Structure: Studies in Phonology.
//!   Cambridge, MIT Press. pp. 107-136.
//! - Jeremy Goslin and Ulrich Frauenfelder. 2001. A comparison of theoretical and human syllabification. Language and Speech, 44:409–436.
//! - Susan Bartlett, et al. 2009. On the Syllabification of Phonemes.
//!   In HLT-NAACL. pp. 308-316.
//! - Christopher Hench. 2017. Resonances in Middle High German: New Methodologies in Prosody. UC Berkeley.

use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

use super::api::TokenizerI;

// The default vowels for in the english language
const VOWELS: &str = "aeiouy";

/// Syllabifies words based on the Legality Principle and Onset Maximization.
#[derive(Debug)]
pub struct LegalitySyllableTokenizer<'a> {
    source_text: Vec<&'a str>,
    vowels: &'a str,
    threshold: f32,
}
impl<'a> TokenizerI<'a> for LegalitySyllableTokenizer<'_> {
    /// Apply the Legality Principle in combination with
    /// Onset Maximization to return a list of syllables.
    fn tokenize(&self, _sent: &'a str) -> Vec<&'a str> {
        todo!()
    }

    fn span_tokenize(&self, _sent: &str) -> Vec<super::util::Token> {
        todo!()
    }
}

impl<'a> LegalitySyllableTokenizer<'a> {
    /// Gathers all onsets and then return only those above the frequency threshold
    pub fn find_legal_onsets(self, _words: Vec<&str>) -> HashMap<&str, usize> {
        todo!()
    }

    /// Returns consonant cluster of word, i.e. all characters until the first vowel.
    /// If the word starts with a vowel, will return [None]
    pub fn onset(self, word: &str) -> Option<&str> {
        let mut index: Option<usize> = None;
        for (i, c) in word.grapheme_indices(true) {
            if self.vowels.contains(c) {
                break;
            } else {
                index = i.into();
            }
        }

        if let Some(..) = index {
            let (result, _) = word.split_at(index.unwrap());
            Some(result)
        } else {
            None
        }
    }

    /// Initializes an instance of the [LegalitySyllableTokenizer] struct
    /// tokenized_source_text: List of valid tokens in the language
    /// language_vowels: Valid vowels in language or IPA representation, defaults to "aeiouy"
    /// legal_frequency_threshold: Lowest frequency of all onsets to be considered a legal onset, defaults to 0.001
    pub fn new<P1, P2>(
        tokenized_source_text: &[&'a str],
        language_vowels: P1,
        legal_frequency_threshold: P2,
    ) -> Self
    where
        P1: Into<Option<&'a str>>,
        P2: Into<Option<f32>>,
    {
        Self {
            source_text: Vec::from(tokenized_source_text),
            vowels: language_vowels.into().unwrap_or(VOWELS),
            threshold: legal_frequency_threshold.into().unwrap_or(0.001),
        }
    }
}
