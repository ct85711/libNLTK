//! Classes for Lazy operations

use std::borrow::{Borrow, Cow};
use std::collections::{btree_map, hash_map, BTreeMap, HashMap};
use std::hash::Hash;
use std::iter::{Iterator, Skip};
use std::ops::Add;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

/// An abstract base class for read-only sequences whose values are
///  computed as needed.  Lazy sequences act like tuples -- they can be
///  indexed, sliced, and iterated over; but they may not be modified.
///
///  The most common application of lazy sequences in NLTK is for
///  corpus view objects, which provide access to the contents of a
///  corpus without loading the entire corpus into memory, by loading
///  pieces of the corpus from disk as needed.
///
///  The result of modifying a mutable element of a lazy sequence is
///  undefined.  In particular, the modifications made to the element
///  may or may not persist, depending on whether and when the lazy
///  sequence caches that element's value or reconstructs it from
///  scratch.
///
///  Subclasses are required to define two methods: ``len()``
///  and ``iterate_from()``.
pub trait AbstractLazySequence {
    /// Return the number of tokens in the corpus file underlying this
    /// corpus view.
    fn len(&self) -> usize;

    /// Returns true the underlying corpus is empty
    fn is_empty(&self) -> bool;

    //fn iterate_from(&self, start: usize) -> !;
}

/// A subsequence produced by slicing a lazy sequence.  This slice
/// keeps a reference to its source sequence, and generates its values
/// by looking them up in the source sequence.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct LazySubsequence<'a> {
    source: Cow<'a, str>,
}
impl AbstractLazySequence for LazySubsequence<'_> {
    fn len(&self) -> usize {
        self.source.as_ref().graphemes(true).count()
    }

    fn is_empty(&self) -> bool {
        self.source.as_ref().is_empty()
    }
}
impl<'a> LazySubsequence<'a> {
    /// Return an iterator that generates the tokens in the corpus
    /// file underlying this corpus view, starting at the token number
    /// ``start``.  If ``start>=len(self)``, then this iterator will
    /// generate no tokens.
    pub fn iterate_from(&self, start: usize) -> Skip<Graphemes> {
        self.source.as_ref().graphemes(true).into_iter().skip(start)
    }

    /// Construct a new slice from a given underlying sequence.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self { source: s.into() }
    }
    /// Return a list concatenating self with itself ``count`` times.
    pub fn repeat(self, count: usize) -> Self {
        Self {
            source: self.source.as_ref().repeat(count).into(),
        }
    }
}
impl Add for LazySubsequence<'_> {
    type Output = Self;

    /// Return a list concatenating self with other.
    fn add(self, other: Self) -> Self {
        Self {
            source: self.source + other.source,
        }
    }
}

/// A lazy sequence whose elements are tuples, each containing the i-th
/// element from each of the argument sequences.  The returned list is
/// truncated in length to the length of the shortest argument sequence. The
/// tuples are constructed lazily -- i.e., when you read a value from the
/// list, ``LazyZip`` will calculate that value by forming a tuple from
/// the i-th element of each of the argument sequences.
#[derive(Debug)]
pub struct LazyZip<K, V> {
    map: HashMap<K, V>,
}
impl<K, V> LazyZip<K, V>
where
    K: Eq + Hash + Copy,
    V: Copy,
{
    /// Sets up a new copy of LazyZip taking in 2 Arrays
    ///
    /// Values are in as an pair(key,value) of the ith value from each array
    /// In case the 2 array's lengths mismatch, the values are taken up to the smallest of the 2
    /// All other left over values are ignored/tossed out
    pub fn new(first: &[K], sec: &[V]) -> Self {
        let mut new_self: LazyZip<K, V> = LazyZip {
            map: HashMap::new(),
        };
        let list_size = if first.len() < sec.len() {
            first.len()
        } else {
            sec.len()
        };
        for p in 0..list_size {
            new_self.map.insert(first[p], sec[p]);
        }
        new_self
    }

    /// Returns an iterator over the map
    pub fn iter(&self) -> hash_map::Iter<'_, K, V> {
        self.map.iter()
    }

    /// Returns an iterator over the map starting from the `start`
    pub fn iter_from(&self, start: usize) -> Skip<hash_map::Iter<'_, K, V>> {
        self.map.iter().skip(start)
    }
}

/// Ordered Dictionary
#[derive(Debug)]
pub struct OrderedDict<K, V> {
    dict: BTreeMap<K, V>,
}
impl<K, V> OrderedDict<K, V> {
    /// Insert The Key-Value pair into the Map
    ///
    /// If the Key was not present, `None` is returned
    ///
    /// If the key was present, the associated Value is updated and the old `Value` is returned
    pub fn insert(&mut self, key: K, val: V) -> Option<V>
    where
        K: Ord,
    {
        self.dict.insert(key, val)
    }

    /// Remove the Key-Value pair based on the Key.
    pub fn del<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.dict.remove(key)
    }
    /// Returns the number of elements in the Ordered Dict
    pub fn len(&self) -> usize {
        self.dict.len()
    }

    ///Returns if the Ordered Dict is empty or not
    pub fn is_empty(&self) -> bool {
        self.dict.is_empty()
    }

    /// Returns an Iterator over the Key,Value Pairs in the Ordered Dict
    pub fn items(&mut self) -> btree_map::Iter<'_, K, V> {
        self.dict.iter()
    }

    /// Returns an Iterator over the Keys in the Ordered Dict
    pub fn keys(&self) -> btree_map::Keys<'_, K, V> {
        self.dict.keys()
    }

    /// Returns an Iterator over the Values in order of the Keys in the Ordered Dict
    pub fn values(&self) -> btree_map::Values<'_, K, V> {
        self.dict.values()
    }

    /// Retrieves the value for the given Key without removing it from the Ordered Dict
    pub fn getitem(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        self.dict.get(key)
    }
}
