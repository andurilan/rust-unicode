#![cfg_attr(test, deny(missing_docs, warnings))]
#![forbid(unused_variables)]

use std::{fmt, mem};
use std::borrow::{Borrow, ToOwned};
use std::default::Default;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo};

/// An owned sequence of Unicode scalar values, equivalent to `String`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UString(Vec<char>);

/// A Unicode string slice, equivalent to `str`.
#[derive(PartialEq, Eq, Hash)]
pub struct UStr([char]);

impl UString {
    /// Appends a character to the end of the string.
    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }

    pub fn remove(&mut self, idx: usize) -> char {
        self.0.remove(idx)
    }
}

impl UStr {
    /// Returns the number of Unicode scalar values in this string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Retrieves the first character from a `&UStr` and returns it.
    ///
    /// This does not allocate a new string; instead, it returns a slice that points one character beyond the character that was shifted.
    ///
    /// If the slice does not contain any characters, None is returned instead.
    pub fn slice_shift_char(&self) -> Option<(char, &UStr)> {
        if self.len() == 0 {
            None
        } else {
            let ch = self[0];
            let next_s = &self[1..];
            Some((ch, next_s))
        }
    }

    pub fn starts_with<'a, T: AsRef<UStr>>(&self, prefix: &'a T) -> bool {
        self.0.starts_with(&prefix.as_ref().0)
    }

    pub fn ends_with<'a, T: AsRef<UStr>>(&self, suffix: &'a T) -> bool {
        self.0.ends_with(&suffix.as_ref().0)
    }
}

impl AsRef<UStr> for UString {
    fn as_ref(&self) -> &UStr {
        &self
    }
}

impl Borrow<UStr> for UString {
    fn borrow(&self) -> &UStr {
        &self
    }
}

impl Default for UString {
    fn default() -> UString {
        UString(Vec::new())
    }
}

impl Deref for UString {
    type Target = UStr;

    fn deref(&self) -> &UStr {
        unsafe {
            mem::transmute(&*self.0)
            // just so you can be clear what it is:
            //mem::transmute::<&[char], &UStr>(&*self.0)
        }
    }
}

impl From<String> for UString {
    fn from(string: String) -> UString {
        UString(string.chars().collect())
    }
}

impl From<Vec<char>> for UString {
    fn from(chars: Vec<char>) -> UString {
        UString(chars)
    }
}

impl<'a> From<&'a str> for UString {
    fn from(string: &'a str) -> UString {
        UString(string.chars().collect())
    }
}

impl<'a> From<&'a [char]> for UString {
    fn from(slice: &[char]) -> UString {
        UString(slice.to_vec())
    }
}

impl From<char> for UString {
    fn from(c: char) -> UString {
        UString(vec![c])
    }
}

impl From<UString> for String {
    fn from(string: UString) -> String {
        String::from_iter(string)
    }
}

impl<'a> From<&'a UString> for String {
    fn from(string: &UString) -> String {
        String::from_iter(string.into_iter().cloned())
    }
}

impl<'a> From<&'a UStr> for String {
    fn from(slice: &UStr) -> String {
        String::from_iter(slice.into_iter().cloned())
    }
}

impl<C: Into<char>> FromIterator<C> for UString {
    fn from_iter<T>(iterator: T) -> UString where T: IntoIterator<Item=C> {
        UString(iterator.into_iter().map(C::into).collect())
    }
}

impl Hash for UString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Index<usize> for UStr {
    type Output = char;

    fn index(&self, idx: usize) -> &char {
        &self.0[idx]
    }
}

impl Index<Range<usize>> for UStr {
    type Output = UStr;

    fn index(&self, idx: Range<usize>) -> &UStr {
        unsafe { mem::transmute(&self.0[idx]) }
    }
}

impl Index<RangeFrom<usize>> for UStr {
    type Output = UStr;

    fn index(&self, idx: RangeFrom<usize>) -> &UStr {
        unsafe { mem::transmute(&self.0[idx]) }
    }
}

impl Index<RangeFull> for UStr {
    type Output = UStr;

    fn index(&self, _: RangeFull) -> &UStr {
        self
    }
}

impl Index<RangeTo<usize>> for UStr {
    type Output = UStr;

    fn index(&self, idx: RangeTo<usize>) -> &UStr {
        unsafe { mem::transmute(&self.0[idx]) }
    }
}

impl ToOwned for UStr {
    type Owned = UString;

    fn to_owned(&self) -> UString {
        UString::from(self.0.to_owned())
    }
}

impl fmt::Display for UStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in &self.0 {
            try!(write!(f, "{}", c));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Iter<'a>(&'a [char]);

#[derive(Debug)]
pub struct IntoIter(Vec<char>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<&'a char> {
        if self.0.len() > 0 {
            let result = &self.0[0];
            self.0 = &self.0[1..];
            Some(result)
        } else {
            None
        }
    }
}

impl Iterator for IntoIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.0.len() > 0 {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for UString {
    type Item = char;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter(self.0)
    }
}

impl<'a> IntoIterator for &'a UString {
    type Item = &'a char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        Iter(&self.0)
    }
}

impl<'a> IntoIterator for &'a UStr {
    type Item = &'a char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        Iter(&self.0)
    }
}
