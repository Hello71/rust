// Copyright 2013-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Composable external iteration.
//!
//! If you've found yourself with a collection of some kind, and needed to
//! perform an operation on the elements of said collection, you'll quickly run
//! into 'iterators'. Iterators are heavily used in idiomatic Rust code, so
//! it's worth becoming familiar with them.
//!
//! Before explaining more, let's talk about how this module is structured:
//!
//! # Organization
//!
//! This module is largely organized by type:
//!
//! * [Traits] are the core portion: these traits define what kind of iterators
//!   exist and what you can do with them. The methods of these traits are worth
//!   putting some extra study time into.
//! * [Functions] provide some helpful ways to create some basic iterators.
//! * [Structs] are often the return types of the various methods on this
//!   module's traits. You'll usually want to look at the method that creates
//!   the `struct`, rather than the `struct` itself. For more detail about why,
//!   see '[Implementing Iterator](#implementing-iterator)'.
//!
//! [Traits]: #traits
//! [Functions]: #functions
//! [Structs]: #structs
//!
//! That's it! Let's dig into iterators.
//!
//! # Iterator
//!
//! The heart and soul of this module is the [`Iterator`] trait. The core of
//! [`Iterator`] looks like this:
//!
//! ```
//! trait Iterator {
//!     type Item;
//!     fn next(&mut self) -> Option<Self::Item>;
//! }
//! ```
//!
//! An iterator has a method, [`next`], which when called, returns an
//! [`Option`]`<Item>`. [`next`] will return `Some(Item)` as long as there
//! are elements, and once they've all been exhausted, will return `None` to
//! indicate that iteration is finished. Individual iterators may choose to
//! resume iteration, and so calling [`next`] again may or may not eventually
//! start returning `Some(Item)` again at some point.
//!
//! [`Iterator`]'s full definition includes a number of other methods as well,
//! but they are default methods, built on top of [`next`], and so you get
//! them for free.
//!
//! Iterators are also composable, and it's common to chain them together to do
//! more complex forms of processing. See the [Adapters](#adapters) section
//! below for more details.
//!
//! [`Iterator`]: trait.Iterator.html
//! [`next`]: trait.Iterator.html#tymethod.next
//! [`Option`]: ../../std/option/enum.Option.html
//!
//! # The three forms of iteration
//!
//! There are three common methods which can create iterators from a collection:
//!
//! * `iter()`, which iterates over `&T`.
//! * `iter_mut()`, which iterates over `&mut T`.
//! * `into_iter()`, which iterates over `T`.
//!
//! Various things in the standard library may implement one or more of the
//! three, where appropriate.
//!
//! # Implementing Iterator
//!
//! Creating an iterator of your own involves two steps: creating a `struct` to
//! hold the iterator's state, and then `impl`ementing [`Iterator`] for that
//! `struct`. This is why there are so many `struct`s in this module: there is
//! one for each iterator and iterator adapter.
//!
//! Let's make an iterator named `Counter` which counts from `1` to `5`:
//!
//! ```
//! // First, the struct:
//!
//! /// An iterator which counts from one to five
//! struct Counter {
//!     count: usize,
//! }
//!
//! // we want our count to start at one, so let's add a new() method to help.
//! // This isn't strictly necessary, but is convenient. Note that we start
//! // `count` at zero, we'll see why in `next()`'s implementation below.
//! impl Counter {
//!     fn new() -> Counter {
//!         Counter { count: 0 }
//!     }
//! }
//!
//! // Then, we implement `Iterator` for our `Counter`:
//!
//! impl Iterator for Counter {
//!     // we will be counting with usize
//!     type Item = usize;
//!
//!     // next() is the only required method
//!     fn next(&mut self) -> Option<usize> {
//!         // increment our count. This is why we started at zero.
//!         self.count += 1;
//!
//!         // check to see if we've finished counting or not.
//!         if self.count < 6 {
//!             Some(self.count)
//!         } else {
//!             None
//!         }
//!     }
//! }
//!
//! // And now we can use it!
//!
//! let mut counter = Counter::new();
//!
//! let x = counter.next().unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().unwrap();
//! println!("{}", x);
//! ```
//!
//! This will print `1` through `5`, each on their own line.
//!
//! Calling `next()` this way gets repetitive. Rust has a construct which can
//! call `next()` on your iterator, until it reaches `None`. Let's go over that
//! next.
//!
//! # for Loops and IntoIterator
//!
//! Rust's `for` loop syntax is actually sugar for iterators. Here's a basic
//! example of `for`:
//!
//! ```
//! let values = vec![1, 2, 3, 4, 5];
//!
//! for x in values {
//!     println!("{}", x);
//! }
//! ```
//!
//! This will print the numbers one through five, each on their own line. But
//! you'll notice something here: we never called anything on our vector to
//! produce an iterator. What gives?
//!
//! There's a trait in the standard library for converting something into an
//! iterator: [`IntoIterator`]. This trait has one method, [`into_iter`],
//! which converts the thing implementing [`IntoIterator`] into an iterator.
//! Let's take a look at that `for` loop again, and what the compiler converts
//! it into:
//!
//! [`IntoIterator`]: trait.IntoIterator.html
//! [`into_iter`]: trait.IntoIterator.html#tymethod.into_iter
//!
//! ```
//! let values = vec![1, 2, 3, 4, 5];
//!
//! for x in values {
//!     println!("{}", x);
//! }
//! ```
//!
//! Rust de-sugars this into:
//!
//! ```
//! let values = vec![1, 2, 3, 4, 5];
//! {
//!     let result = match IntoIterator::into_iter(values) {
//!         mut iter => loop {
//!             let next;
//!             match iter.next() {
//!                 Some(val) => next = val,
//!                 None => break,
//!             };
//!             let x = next;
//!             let () = { println!("{}", x); };
//!         },
//!     };
//!     result
//! }
//! ```
//!
//! First, we call `into_iter()` on the value. Then, we match on the iterator
//! that returns, calling [`next`] over and over until we see a `None`. At
//! that point, we `break` out of the loop, and we're done iterating.
//!
//! There's one more subtle bit here: the standard library contains an
//! interesting implementation of [`IntoIterator`]:
//!
//! ```ignore (only-for-syntax-highlight)
//! impl<I: Iterator> IntoIterator for I
//! ```
//!
//! In other words, all [`Iterator`]s implement [`IntoIterator`], by just
//! returning themselves. This means two things:
//!
//! 1. If you're writing an [`Iterator`], you can use it with a `for` loop.
//! 2. If you're creating a collection, implementing [`IntoIterator`] for it
//!    will allow your collection to be used with the `for` loop.
//!
//! # Adapters
//!
//! Functions which take an [`Iterator`] and return another [`Iterator`] are
//! often called 'iterator adapters', as they're a form of the 'adapter
//! pattern'.
//!
//! Common iterator adapters include [`map`], [`take`], and [`filter`].
//! For more, see their documentation.
//!
//! [`map`]: trait.Iterator.html#method.map
//! [`take`]: trait.Iterator.html#method.take
//! [`filter`]: trait.Iterator.html#method.filter
//!
//! # Laziness
//!
//! Iterators (and iterator [adapters](#adapters)) are *lazy*. This means that
//! just creating an iterator doesn't _do_ a whole lot. Nothing really happens
//! until you call [`next`]. This is sometimes a source of confusion when
//! creating an iterator solely for its side effects. For example, the [`map`]
//! method calls a closure on each element it iterates over:
//!
//! ```
//! # #![allow(unused_must_use)]
//! let v = vec![1, 2, 3, 4, 5];
//! v.iter().map(|x| println!("{}", x));
//! ```
//!
//! This will not print any values, as we only created an iterator, rather than
//! using it. The compiler will warn us about this kind of behavior:
//!
//! ```text
//! warning: unused result which must be used: iterator adaptors are lazy and
//! do nothing unless consumed
//! ```
//!
//! The idiomatic way to write a [`map`] for its side effects is to use a
//! `for` loop instead:
//!
//! ```
//! let v = vec![1, 2, 3, 4, 5];
//!
//! for x in &v {
//!     println!("{}", x);
//! }
//! ```
//!
//! [`map`]: trait.Iterator.html#method.map
//!
//! The two most common ways to evaluate an iterator are to use a `for` loop
//! like this, or using the [`collect`] method to produce a new collection.
//!
//! [`collect`]: trait.Iterator.html#method.collect
//!
//! # Infinity
//!
//! Iterators do not have to be finite. As an example, an open-ended range is
//! an infinite iterator:
//!
//! ```
//! let numbers = 0..;
//! ```
//!
//! It is common to use the [`take`] iterator adapter to turn an infinite
//! iterator into a finite one:
//!
//! ```
//! let numbers = 0..;
//! let five_numbers = numbers.take(5);
//!
//! for number in five_numbers {
//!     println!("{}", number);
//! }
//! ```
//!
//! This will print the numbers `0` through `4`, each on their own line.
//!
//! Bear in mind that methods on infinite iterators, even those for which a
//! result can be determined mathematically in finite time, may not terminate.
//! Specifically, methods such as [`min`], which in the general case require
//! traversing every element in the iterator, are likely not to return
//! successfully for any infinite iterators.
//!
//! ```no_run
//! let ones = std::iter::repeat(1);
//! let least = ones.min().unwrap(); // Oh no! An infinite loop!
//! // `ones.min()` causes an infinite loop, so we won't reach this point!
//! println!("The smallest number one is {}.", least);
//! ```
//!
//! [`take`]: trait.Iterator.html#method.take
//! [`min`]: trait.Iterator.html#method.min

#![stable(feature = "rust1", since = "1.0.0")]

use cmp;
use fmt;
use iter_private::TrustedRandomAccess;
use ops::Try;
use usize;
use intrinsics;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::iterator::Iterator;

#[unstable(feature = "step_trait",
           reason = "likely to be replaced by finer-grained traits",
           issue = "42168")]
pub use self::range::Step;

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::sources::{Repeat, repeat};
#[unstable(feature = "iterator_repeat_with", issue = "48169")]
pub use self::sources::{RepeatWith, repeat_with};
#[stable(feature = "iter_empty", since = "1.2.0")]
pub use self::sources::{Empty, empty};
#[stable(feature = "iter_once", since = "1.2.0")]
pub use self::sources::{Once, once};

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::traits::{FromIterator, IntoIterator, DoubleEndedIterator, Extend};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::traits::{ExactSizeIterator, Sum, Product};
#[unstable(feature = "fused", issue = "35602")]
pub use self::traits::FusedIterator;
#[unstable(feature = "trusted_len", issue = "37572")]
pub use self::traits::TrustedLen;

mod iterator;
mod range;
mod sources;
mod traits;

/// Transparent newtype used to implement foo methods in terms of try_foo.
/// Important until #43278 is fixed; might be better as `Result<T, !>` later.
struct AlwaysOk<T>(pub T);

impl<T> Try for AlwaysOk<T> {
    type Ok = T;
    type Error = !;
    #[inline]
    fn into_result(self) -> Result<Self::Ok, Self::Error> { Ok(self.0) }
    #[inline]
    fn from_error(v: Self::Error) -> Self { v }
    #[inline]
    fn from_ok(v: Self::Ok) -> Self { AlwaysOk(v) }
}

/// Used to make try_fold closures more like normal loops
#[derive(PartialEq)]
enum LoopState<C, B> {
    Continue(C),
    Break(B),
}

impl<C, B> Try for LoopState<C, B> {
    type Ok = C;
    type Error = B;
    #[inline]
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            LoopState::Continue(y) => Ok(y),
            LoopState::Break(x) => Err(x),
        }
    }
    #[inline]
    fn from_error(v: Self::Error) -> Self { LoopState::Break(v) }
    #[inline]
    fn from_ok(v: Self::Ok) -> Self { LoopState::Continue(v) }
}

impl<C, B> LoopState<C, B> {
    #[inline]
    fn break_value(self) -> Option<B> {
        match self {
            LoopState::Continue(..) => None,
            LoopState::Break(x) => Some(x),
        }
    }
}

impl<R: Try> LoopState<R::Ok, R> {
    #[inline]
    fn from_try(r: R) -> Self {
        match Try::into_result(r) {
            Ok(v) => LoopState::Continue(v),
            Err(v) => LoopState::Break(Try::from_error(v)),
        }
    }
    #[inline]
    fn into_try(self) -> R {
        match self {
            LoopState::Continue(v) => Try::from_ok(v),
            LoopState::Break(v) => v,
        }
    }
}

/// A double-ended iterator with the direction inverted.
///
/// This `struct` is created by the [`rev`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`rev`]: trait.Iterator.html#method.rev
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Rev<T> {
    iter: T
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Rev<I> where I: DoubleEndedIterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> { self.iter.next_back() }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        self.iter.try_rfold(init, f)
    }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, f)
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    {
        self.iter.rfind(predicate)
    }

    #[inline]
    fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
        P: FnMut(Self::Item) -> bool
    {
        self.iter.position(predicate)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Rev<I> where I: DoubleEndedIterator {
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> { self.iter.next() }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        self.iter.try_fold(init, f)
    }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, f)
    }

    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool
    {
        self.iter.find(predicate)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Rev<I>
    where I: ExactSizeIterator + DoubleEndedIterator
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Rev<I>
    where I: FusedIterator + DoubleEndedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I> TrustedLen for Rev<I>
    where I: TrustedLen + DoubleEndedIterator {}

/// An iterator that clones the elements of an underlying iterator.
///
/// This `struct` is created by the [`cloned`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`cloned`]: trait.Iterator.html#method.cloned
/// [`Iterator`]: trait.Iterator.html
#[stable(feature = "iter_cloned", since = "1.1.0")]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Cloned<I> {
    it: I,
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: Iterator<Item=&'a T>, T: Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.it.next().cloned()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        self.it.try_fold(init, move |acc, elt| f(acc, elt.clone()))
    }

    fn fold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.it.fold(init, move |acc, elt| f(acc, elt.clone()))
    }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> DoubleEndedIterator for Cloned<I>
    where I: DoubleEndedIterator<Item=&'a T>, T: Clone
{
    fn next_back(&mut self) -> Option<T> {
        self.it.next_back().cloned()
    }

    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        self.it.try_rfold(init, move |acc, elt| f(acc, elt.clone()))
    }

    fn rfold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.it.rfold(init, move |acc, elt| f(acc, elt.clone()))
    }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
impl<'a, I, T: 'a> ExactSizeIterator for Cloned<I>
    where I: ExactSizeIterator<Item=&'a T>, T: Clone
{
    fn len(&self) -> usize {
        self.it.len()
    }

    fn is_empty(&self) -> bool {
        self.it.is_empty()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<'a, I, T: 'a> FusedIterator for Cloned<I>
    where I: FusedIterator<Item=&'a T>, T: Clone
{}

#[doc(hidden)]
default unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Clone
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
        self.it.get_unchecked(i).clone()
    }

    #[inline]
    fn may_have_side_effect() -> bool { true }
}

#[doc(hidden)]
unsafe impl<'a, I, T: 'a> TrustedRandomAccess for Cloned<I>
    where I: TrustedRandomAccess<Item=&'a T>, T: Copy
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
        *self.it.get_unchecked(i)
    }

    #[inline]
    fn may_have_side_effect() -> bool { false }
}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<'a, I, T: 'a> TrustedLen for Cloned<I>
    where I: TrustedLen<Item=&'a T>,
          T: Clone
{}

/// An iterator that repeats endlessly.
///
/// This `struct` is created by the [`cycle`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`cycle`]: trait.Iterator.html#method.cycle
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Cycle<I> {
    orig: I,
    iter: I,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Cycle<I> where I: Clone + Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.iter.next() {
            None => { self.iter = self.orig.clone(); self.iter.next() }
            y => y
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // the cycle iterator is either empty or infinite
        match self.orig.size_hint() {
            sz @ (0, Some(0)) => sz,
            (0, _) => (0, None),
            _ => (usize::MAX, None)
        }
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Cycle<I> where I: Clone + Iterator {}

/// An iterator for stepping iterators by a custom amount.
///
/// This `struct` is created by the [`step_by`] method on [`Iterator`]. See
/// its documentation for more.
///
/// [`step_by`]: trait.Iterator.html#method.step_by
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[unstable(feature = "iterator_step_by",
           reason = "unstable replacement of Range::step_by",
           issue = "27741")]
#[derive(Clone, Debug)]
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

#[unstable(feature = "iterator_step_by",
           reason = "unstable replacement of Range::step_by",
           issue = "27741")]
impl<I> Iterator for StepBy<I> where I: Iterator {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first_take {
            self.first_take = false;
            self.iter.next()
        } else {
            self.iter.nth(self.step)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let inner_hint = self.iter.size_hint();

        if self.first_take {
            let f = |n| if n == 0 { 0 } else { 1 + (n-1)/(self.step+1) };
            (f(inner_hint.0), inner_hint.1.map(f))
        } else {
            let f = |n| n / (self.step+1);
            (f(inner_hint.0), inner_hint.1.map(f))
        }
    }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if self.first_take {
            self.first_take = false;
            let first = self.iter.next();
            if n == 0 {
                return first;
            }
            n -= 1;
        }
        // n and self.step are indices, we need to add 1 to get the amount of elements
        // When calling `.nth`, we need to subtract 1 again to convert back to an index
        // step + 1 can't overflow because `.step_by` sets `self.step` to `step - 1`
        let mut step = self.step + 1;
        // n + 1 could overflow
        // thus, if n is usize::MAX, instead of adding one, we call .nth(step)
        if n == usize::MAX {
            self.iter.nth(step - 1);
        } else {
            n += 1;
        }

        // overflow handling
        loop {
            let mul = n.checked_mul(step);
            if unsafe { intrinsics::likely(mul.is_some()) } {
                return self.iter.nth(mul.unwrap() - 1);
            }
            let div_n = usize::MAX / n;
            let div_step = usize::MAX / step;
            let nth_n = div_n * n;
            let nth_step = div_step * step;
            let nth = if nth_n > nth_step {
                step -= div_n;
                nth_n
            } else {
                n -= div_step;
                nth_step
            };
            self.iter.nth(nth - 1);
        }
    }
}

// StepBy can only make the iterator shorter, so the len will still fit.
#[unstable(feature = "iterator_step_by",
           reason = "unstable replacement of Range::step_by",
           issue = "27741")]
impl<I> ExactSizeIterator for StepBy<I> where I: ExactSizeIterator {}

/// An iterator that strings two iterators together.
///
/// This `struct` is created by the [`chain`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`chain`]: trait.Iterator.html#method.chain
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Chain<A, B> {
    a: A,
    b: B,
    state: ChainState,
}

// The iterator protocol specifies that iteration ends with the return value
// `None` from `.next()` (or `.next_back()`) and it is unspecified what
// further calls return. The chain adaptor must account for this since it uses
// two subiterators.
//
//  It uses three states:
//
//  - Both: `a` and `b` are remaining
//  - Front: `a` remaining
//  - Back: `b` remaining
//
//  The fourth state (neither iterator is remaining) only occurs after Chain has
//  returned None once, so we don't need to store this state.
#[derive(Clone, Debug)]
enum ChainState {
    // both front and back iterator are remaining
    Both,
    // only front is remaining
    Front,
    // only back is remaining
    Back,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> Iterator for Chain<A, B> where
    A: Iterator,
    B: Iterator<Item = A::Item>
{
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> {
        match self.state {
            ChainState::Both => match self.a.next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = ChainState::Back;
                    self.b.next()
                }
            },
            ChainState::Front => self.a.next(),
            ChainState::Back => self.b.next(),
        }
    }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn count(self) -> usize {
        match self.state {
            ChainState::Both => self.a.count() + self.b.count(),
            ChainState::Front => self.a.count(),
            ChainState::Back => self.b.count(),
        }
    }

    fn try_fold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let mut accum = init;
        match self.state {
            ChainState::Both | ChainState::Front => {
                accum = self.a.try_fold(accum, &mut f)?;
                if let ChainState::Both = self.state {
                    self.state = ChainState::Back;
                }
            }
            _ => { }
        }
        if let ChainState::Back = self.state {
            accum = self.b.try_fold(accum, &mut f)?;
        }
        Try::from_ok(accum)
    }

    fn fold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut accum = init;
        match self.state {
            ChainState::Both | ChainState::Front => {
                accum = self.a.fold(accum, &mut f);
            }
            _ => { }
        }
        match self.state {
            ChainState::Both | ChainState::Back => {
                accum = self.b.fold(accum, &mut f);
            }
            _ => { }
        }
        accum
    }

    #[inline]
    fn nth(&mut self, mut n: usize) -> Option<A::Item> {
        match self.state {
            ChainState::Both | ChainState::Front => {
                for x in self.a.by_ref() {
                    if n == 0 {
                        return Some(x)
                    }
                    n -= 1;
                }
                if let ChainState::Both = self.state {
                    self.state = ChainState::Back;
                }
            }
            ChainState::Back => {}
        }
        if let ChainState::Back = self.state {
            self.b.nth(n)
        } else {
            None
        }
    }

    #[inline]
    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item> where
        P: FnMut(&Self::Item) -> bool,
    {
        match self.state {
            ChainState::Both => match self.a.find(&mut predicate) {
                None => {
                    self.state = ChainState::Back;
                    self.b.find(predicate)
                }
                v => v
            },
            ChainState::Front => self.a.find(predicate),
            ChainState::Back => self.b.find(predicate),
        }
    }

    #[inline]
    fn last(self) -> Option<A::Item> {
        match self.state {
            ChainState::Both => {
                // Must exhaust a before b.
                let a_last = self.a.last();
                let b_last = self.b.last();
                b_last.or(a_last)
            },
            ChainState::Front => self.a.last(),
            ChainState::Back => self.b.last()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = a_lower.saturating_add(b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None
        };

        (lower, upper)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> DoubleEndedIterator for Chain<A, B> where
    A: DoubleEndedIterator,
    B: DoubleEndedIterator<Item=A::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<A::Item> {
        match self.state {
            ChainState::Both => match self.b.next_back() {
                elt @ Some(..) => elt,
                None => {
                    self.state = ChainState::Front;
                    self.a.next_back()
                }
            },
            ChainState::Front => self.a.next_back(),
            ChainState::Back => self.b.next_back(),
        }
    }

    fn try_rfold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let mut accum = init;
        match self.state {
            ChainState::Both | ChainState::Back => {
                accum = self.b.try_rfold(accum, &mut f)?;
                if let ChainState::Both = self.state {
                    self.state = ChainState::Front;
                }
            }
            _ => { }
        }
        if let ChainState::Front = self.state {
            accum = self.a.try_rfold(accum, &mut f)?;
        }
        Try::from_ok(accum)
    }

    fn rfold<Acc, F>(self, init: Acc, mut f: F) -> Acc
        where F: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut accum = init;
        match self.state {
            ChainState::Both | ChainState::Back => {
                accum = self.b.rfold(accum, &mut f);
            }
            _ => { }
        }
        match self.state {
            ChainState::Both | ChainState::Front => {
                accum = self.a.rfold(accum, &mut f);
            }
            _ => { }
        }
        accum
    }

}

// Note: *both* must be fused to handle double-ended iterators.
#[unstable(feature = "fused", issue = "35602")]
impl<A, B> FusedIterator for Chain<A, B>
    where A: FusedIterator,
          B: FusedIterator<Item=A::Item>,
{}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A, B> TrustedLen for Chain<A, B>
    where A: TrustedLen, B: TrustedLen<Item=A::Item>,
{}

/// An iterator that iterates two other iterators simultaneously.
///
/// This `struct` is created by the [`zip`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`zip`]: trait.Iterator.html#method.zip
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Zip<A, B> {
    a: A,
    b: B,
    // index and len are only used by the specialized version of zip
    index: usize,
    len: usize,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> Iterator for Zip<A, B> where A: Iterator, B: Iterator
{
    type Item = (A::Item, B::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        ZipImpl::next(self)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        ZipImpl::size_hint(self)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> DoubleEndedIterator for Zip<A, B> where
    A: DoubleEndedIterator + ExactSizeIterator,
    B: DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<(A::Item, B::Item)> {
        ZipImpl::next_back(self)
    }
}

// Zip specialization trait
#[doc(hidden)]
trait ZipImpl<A, B> {
    type Item;
    fn new(a: A, b: B) -> Self;
    fn next(&mut self) -> Option<Self::Item>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn next_back(&mut self) -> Option<Self::Item>
        where A: DoubleEndedIterator + ExactSizeIterator,
              B: DoubleEndedIterator + ExactSizeIterator;
}

// General Zip impl
#[doc(hidden)]
impl<A, B> ZipImpl<A, B> for Zip<A, B>
    where A: Iterator, B: Iterator
{
    type Item = (A::Item, B::Item);
    default fn new(a: A, b: B) -> Self {
        Zip {
            a,
            b,
            index: 0, // unused
            len: 0, // unused
        }
    }

    #[inline]
    default fn next(&mut self) -> Option<(A::Item, B::Item)> {
        self.a.next().and_then(|x| {
            self.b.next().and_then(|y| {
                Some((x, y))
            })
        })
    }

    #[inline]
    default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
        where A: DoubleEndedIterator + ExactSizeIterator,
              B: DoubleEndedIterator + ExactSizeIterator
    {
        let a_sz = self.a.len();
        let b_sz = self.b.len();
        if a_sz != b_sz {
            // Adjust a, b to equal length
            if a_sz > b_sz {
                for _ in 0..a_sz - b_sz { self.a.next_back(); }
            } else {
                for _ in 0..b_sz - a_sz { self.b.next_back(); }
            }
        }
        match (self.a.next_back(), self.b.next_back()) {
            (Some(x), Some(y)) => Some((x, y)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }

    #[inline]
    default fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = cmp::min(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => Some(cmp::min(x,y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None
        };

        (lower, upper)
    }
}

#[doc(hidden)]
impl<A, B> ZipImpl<A, B> for Zip<A, B>
    where A: TrustedRandomAccess, B: TrustedRandomAccess
{
    fn new(a: A, b: B) -> Self {
        let len = cmp::min(a.len(), b.len());
        Zip {
            a,
            b,
            index: 0,
            len,
        }
    }

    #[inline]
    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        if self.index < self.len {
            let i = self.index;
            self.index += 1;
            unsafe {
                Some((self.a.get_unchecked(i), self.b.get_unchecked(i)))
            }
        } else if A::may_have_side_effect() && self.index < self.a.len() {
            // match the base implementation's potential side effects
            unsafe {
                self.a.get_unchecked(self.index);
            }
            self.index += 1;
            None
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len - self.index;
        (len, Some(len))
    }

    #[inline]
    fn next_back(&mut self) -> Option<(A::Item, B::Item)>
        where A: DoubleEndedIterator + ExactSizeIterator,
              B: DoubleEndedIterator + ExactSizeIterator
    {
        // Adjust a, b to equal length
        if A::may_have_side_effect() {
            let sz = self.a.len();
            if sz > self.len {
                for _ in 0..sz - cmp::max(self.len, self.index) {
                    self.a.next_back();
                }
            }
        }
        if B::may_have_side_effect() {
            let sz = self.b.len();
            if sz > self.len {
                for _ in 0..sz - self.len {
                    self.b.next_back();
                }
            }
        }
        if self.index < self.len {
            self.len -= 1;
            let i = self.len;
            unsafe {
                Some((self.a.get_unchecked(i), self.b.get_unchecked(i)))
            }
        } else {
            None
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<A, B> ExactSizeIterator for Zip<A, B>
    where A: ExactSizeIterator, B: ExactSizeIterator {}

#[doc(hidden)]
unsafe impl<A, B> TrustedRandomAccess for Zip<A, B>
    where A: TrustedRandomAccess,
          B: TrustedRandomAccess,
{
    unsafe fn get_unchecked(&mut self, i: usize) -> (A::Item, B::Item) {
        (self.a.get_unchecked(i), self.b.get_unchecked(i))
    }

    fn may_have_side_effect() -> bool {
        A::may_have_side_effect() || B::may_have_side_effect()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<A, B> FusedIterator for Zip<A, B>
    where A: FusedIterator, B: FusedIterator, {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<A, B> TrustedLen for Zip<A, B>
    where A: TrustedLen, B: TrustedLen,
{}

/// An iterator that maps the values of `iter` with `f`.
///
/// This `struct` is created by the [`map`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`map`]: trait.Iterator.html#method.map
/// [`Iterator`]: trait.Iterator.html
///
/// # Notes about side effects
///
/// The [`map`] iterator implements [`DoubleEndedIterator`], meaning that
/// you can also [`map`] backwards:
///
/// ```rust
/// let v: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x + 1).rev().collect();
///
/// assert_eq!(v, [4, 3, 2]);
/// ```
///
/// [`DoubleEndedIterator`]: trait.DoubleEndedIterator.html
///
/// But if your closure has state, iterating backwards may act in a way you do
/// not expect. Let's go through an example. First, in the forward direction:
///
/// ```rust
/// let mut c = 0;
///
/// for pair in vec!['a', 'b', 'c'].into_iter()
///                                .map(|letter| { c += 1; (letter, c) }) {
///     println!("{:?}", pair);
/// }
/// ```
///
/// This will print "('a', 1), ('b', 2), ('c', 3)".
///
/// Now consider this twist where we add a call to `rev`. This version will
/// print `('c', 1), ('b', 2), ('a', 3)`. Note that the letters are reversed,
/// but the values of the counter still go in order. This is because `map()` is
/// still being called lazily on each item, but we are popping items off the
/// back of the vector now, instead of shifting them from the front.
///
/// ```rust
/// let mut c = 0;
///
/// for pair in vec!['a', 'b', 'c'].into_iter()
///                                .map(|letter| { c += 1; (letter, c) })
///                                .rev() {
///     println!("{:?}", pair);
/// }
/// ```
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Map<I, F> {
    iter: I,
    f: F,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Map")
            .field("iter", &self.iter)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_fold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R where
        Self: Sized, G: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_fold(init, move |acc, elt| g(acc, f(elt)))
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.fold(init, move |acc, elt| g(acc, f(elt)))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for Map<I, F> where
    F: FnMut(I::Item) -> B,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        self.iter.next_back().map(&mut self.f)
    }

    fn try_rfold<Acc, G, R>(&mut self, init: Acc, mut g: G) -> R where
        Self: Sized, G: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_rfold(init, move |acc, elt| g(acc, f(elt)))
    }

    fn rfold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.rfold(init, move |acc, elt| g(acc, f(elt)))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: ExactSizeIterator, F> ExactSizeIterator for Map<I, F>
    where F: FnMut(I::Item) -> B
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
    where F: FnMut(I::Item) -> B {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<B, I, F> TrustedLen for Map<I, F>
    where I: TrustedLen,
          F: FnMut(I::Item) -> B {}

#[doc(hidden)]
unsafe impl<B, I, F> TrustedRandomAccess for Map<I, F>
    where I: TrustedRandomAccess,
          F: FnMut(I::Item) -> B,
{
    unsafe fn get_unchecked(&mut self, i: usize) -> Self::Item {
        (self.f)(self.iter.get_unchecked(i))
    }
    #[inline]
    fn may_have_side_effect() -> bool { true }
}

/// An iterator that filters the elements of `iter` with `predicate`.
///
/// This `struct` is created by the [`filter`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`filter`]: trait.Iterator.html#method.filter
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Filter<I, P> {
    iter: I,
    predicate: P,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for Filter<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter")
            .field("iter", &self.iter)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        for x in &mut self.iter {
            if (self.predicate)(&x) {
                return Some(x);
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    // this special case allows the compiler to make `.filter(_).count()`
    // branchless. Barring perfect branch prediction (which is unattainable in
    // the general case), this will be much faster in >90% of cases (containing
    // virtually all real workloads) and only a tiny bit slower in the rest.
    //
    // Having this specialization thus allows us to write `.filter(p).count()`
    // where we would otherwise write `.map(|x| p(x) as usize).sum()`, which is
    // less readable and also less backwards-compatible to Rust before 1.10.
    //
    // Using the branchless version will also simplify the LLVM byte code, thus
    // leaving more budget for LLVM optimizations.
    #[inline]
    fn count(mut self) -> usize {
        let mut count = 0;
        for x in &mut self.iter {
            count += (self.predicate)(&x) as usize;
        }
        count
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let predicate = &mut self.predicate;
        self.iter.try_fold(init, move |acc, item| if predicate(&item) {
            fold(acc, item)
        } else {
            Try::from_ok(acc)
        })
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut predicate = self.predicate;
        self.iter.fold(init, move |acc, item| if predicate(&item) {
            fold(acc, item)
        } else {
            acc
        })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, P> DoubleEndedIterator for Filter<I, P>
    where P: FnMut(&I::Item) -> bool,
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        for x in self.iter.by_ref().rev() {
            if (self.predicate)(&x) {
                return Some(x);
            }
        }
        None
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let predicate = &mut self.predicate;
        self.iter.try_rfold(init, move |acc, item| if predicate(&item) {
            fold(acc, item)
        } else {
            Try::from_ok(acc)
        })
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut predicate = self.predicate;
        self.iter.rfold(init, move |acc, item| if predicate(&item) {
            fold(acc, item)
        } else {
            acc
        })
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I: FusedIterator, P> FusedIterator for Filter<I, P>
    where P: FnMut(&I::Item) -> bool {}

/// An iterator that uses `f` to both filter and map elements from `iter`.
///
/// This `struct` is created by the [`filter_map`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`filter_map`]: trait.Iterator.html#method.filter_map
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for FilterMap<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FilterMap")
            .field("iter", &self.iter)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        for x in self.iter.by_ref() {
            if let Some(y) = (self.f)(x) {
                return Some(y);
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_fold(init, move |acc, item| match f(item) {
            Some(x) => fold(acc, x),
            None => Try::from_ok(acc),
        })
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.fold(init, move |acc, item| match f(item) {
            Some(x) => fold(acc, x),
            None => acc,
        })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B>,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        for x in self.iter.by_ref().rev() {
            if let Some(y) = (self.f)(x) {
                return Some(y);
            }
        }
        None
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_rfold(init, move |acc, item| match f(item) {
            Some(x) => fold(acc, x),
            None => Try::from_ok(acc),
        })
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.rfold(init, move |acc, item| match f(item) {
            Some(x) => fold(acc, x),
            None => acc,
        })
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<B, I: FusedIterator, F> FusedIterator for FilterMap<I, F>
    where F: FnMut(I::Item) -> Option<B> {}

/// An iterator that yields the current count and the element during iteration.
///
/// This `struct` is created by the [`enumerate`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`enumerate`]: trait.Iterator.html#method.enumerate
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Enumerate<I> where I: Iterator {
    type Item = (usize, <I as Iterator>::Item);

    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so enumerating more than
    /// `usize::MAX` elements either produces the wrong result or panics. If
    /// debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// Might panic if the index of the element overflows a `usize`.
    #[inline]
    #[rustc_inherit_overflow_checks]
    fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
        self.iter.next().map(|a| {
            let ret = (self.count, a);
            // Possible undefined overflow.
            self.count += 1;
            ret
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn nth(&mut self, n: usize) -> Option<(usize, I::Item)> {
        self.iter.nth(n).map(|a| {
            let i = self.count + n;
            self.count = i + 1;
            (i, a)
        })
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let count = &mut self.count;
        self.iter.try_fold(init, move |acc, item| {
            let acc = fold(acc, (*count, item));
            *count += 1;
            acc
        })
    }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut count = self.count;
        self.iter.fold(init, move |acc, item| {
            let acc = fold(acc, (count, item));
            count += 1;
            acc
        })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Enumerate<I> where
    I: ExactSizeIterator + DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
        self.iter.next_back().map(|a| {
            let len = self.iter.len();
            // Can safely add, `ExactSizeIterator` promises that the number of
            // elements fits into a `usize`.
            (self.count + len, a)
        })
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        // Can safely add and subtract the count, as `ExactSizeIterator` promises
        // that the number of elements fits into a `usize`.
        let mut count = self.count + self.iter.len();
        self.iter.try_rfold(init, move |acc, item| {
            count -= 1;
            fold(acc, (count, item))
        })
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        // Can safely add and subtract the count, as `ExactSizeIterator` promises
        // that the number of elements fits into a `usize`.
        let mut count = self.count + self.iter.len();
        self.iter.rfold(init, move |acc, item| {
            count -= 1;
            fold(acc, (count, item))
        })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Enumerate<I> where I: ExactSizeIterator {
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[doc(hidden)]
unsafe impl<I> TrustedRandomAccess for Enumerate<I>
    where I: TrustedRandomAccess
{
    unsafe fn get_unchecked(&mut self, i: usize) -> (usize, I::Item) {
        (self.count + i, self.iter.get_unchecked(i))
    }

    fn may_have_side_effect() -> bool {
        I::may_have_side_effect()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Enumerate<I> where I: FusedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I> TrustedLen for Enumerate<I>
    where I: TrustedLen,
{}


/// An iterator with a `peek()` that returns an optional reference to the next
/// element.
///
/// This `struct` is created by the [`peekable`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`peekable`]: trait.Iterator.html#method.peekable
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Peekable<I: Iterator> {
    iter: I,
    /// Remember a peeked value, even if it was None.
    peeked: Option<Option<I::Item>>,
}

// Peekable must remember if a None has been seen in the `.peek()` method.
// It ensures that `.peek(); .peek();` or `.peek(); .next();` only advances the
// underlying iterator at most once. This does not by itself make the iterator
// fused.
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator> Iterator for Peekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.iter.next(),
        }
    }

    #[inline]
    #[rustc_inherit_overflow_checks]
    fn count(mut self) -> usize {
        match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.iter.count(),
            None => self.iter.count(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        // FIXME(#6393): merge these when borrow-checking gets better.
        if n == 0 {
            match self.peeked.take() {
                Some(v) => v,
                None => self.iter.nth(n),
            }
        } else {
            match self.peeked.take() {
                Some(None) => None,
                Some(Some(_)) => self.iter.nth(n - 1),
                None => self.iter.nth(n),
            }
        }
    }

    #[inline]
    fn last(mut self) -> Option<I::Item> {
        let peek_opt = match self.peeked.take() {
            Some(None) => return None,
            Some(v) => v,
            None => None,
        };
        self.iter.last().or(peek_opt)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = match self.peeked {
            Some(None) => return (0, Some(0)),
            Some(Some(_)) => 1,
            None => 0,
        };
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = hi.and_then(|x| x.checked_add(peek_len));
        (lo, hi)
    }

    #[inline]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R where
        Self: Sized, F: FnMut(B, Self::Item) -> R, R: Try<Ok=B>
    {
        let acc = match self.peeked.take() {
            Some(None) => return Try::from_ok(init),
            Some(Some(v)) => f(init, v)?,
            None => init,
        };
        self.iter.try_fold(acc, f)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let acc = match self.peeked {
            Some(None) => return init,
            Some(Some(v)) => fold(init, v),
            None => init,
        };
        self.iter.fold(acc, fold)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator> ExactSizeIterator for Peekable<I> {}

#[unstable(feature = "fused", issue = "35602")]
impl<I: FusedIterator> FusedIterator for Peekable<I> {}

impl<I: Iterator> Peekable<I> {
    /// Returns a reference to the next() value without advancing the iterator.
    ///
    /// Like [`next`], if there is a value, it is wrapped in a `Some(T)`.
    /// But if the iteration is over, `None` is returned.
    ///
    /// [`next`]: trait.Iterator.html#tymethod.next
    ///
    /// Because `peek()` returns a reference, and many iterators iterate over
    /// references, there can be a possibly confusing situation where the
    /// return value is a double reference. You can see this effect in the
    /// examples below.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let xs = [1, 2, 3];
    ///
    /// let mut iter = xs.iter().peekable();
    ///
    /// // peek() lets us see into the future
    /// assert_eq!(iter.peek(), Some(&&1));
    /// assert_eq!(iter.next(), Some(&1));
    ///
    /// assert_eq!(iter.next(), Some(&2));
    ///
    /// // The iterator does not advance even if we `peek` multiple times
    /// assert_eq!(iter.peek(), Some(&&3));
    /// assert_eq!(iter.peek(), Some(&&3));
    ///
    /// assert_eq!(iter.next(), Some(&3));
    ///
    /// // After the iterator is finished, so is `peek()`
    /// assert_eq!(iter.peek(), None);
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn peek(&mut self) -> Option<&I::Item> {
        if self.peeked.is_none() {
            self.peeked = Some(self.iter.next());
        }
        match self.peeked {
            Some(Some(ref value)) => Some(value),
            Some(None) => None,
            _ => unreachable!(),
        }
    }
}

/// An iterator that rejects elements while `predicate` is true.
///
/// This `struct` is created by the [`skip_while`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`skip_while`]: trait.Iterator.html#method.skip_while
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct SkipWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for SkipWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SkipWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for SkipWhile<I, P>
    where P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let flag = &mut self.flag;
        let pred = &mut self.predicate;
        self.iter.find(move |x| {
            if *flag || !pred(x) {
                *flag = true;
                true
            } else {
                false
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, mut init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if !self.flag {
            match self.next() {
                Some(v) => init = fold(init, v)?,
                None => return Try::from_ok(init),
            }
        }
        self.iter.try_fold(init, fold)
    }

    #[inline]
    fn fold<Acc, Fold>(mut self, mut init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        if !self.flag {
            match self.next() {
                Some(v) => init = fold(init, v),
                None => return init,
            }
        }
        self.iter.fold(init, fold)
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I, P> FusedIterator for SkipWhile<I, P>
    where I: FusedIterator, P: FnMut(&I::Item) -> bool {}

/// An iterator that only accepts elements while `predicate` is true.
///
/// This `struct` is created by the [`take_while`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`take_while`]: trait.Iterator.html#method.take_while
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct TakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, P> fmt::Debug for TakeWhile<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TakeWhile")
            .field("iter", &self.iter)
            .field("flag", &self.flag)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, P> Iterator for TakeWhile<I, P>
    where P: FnMut(&I::Item) -> bool
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    Some(x)
                } else {
                    self.flag = true;
                    None
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if self.flag {
            Try::from_ok(init)
        } else {
            let flag = &mut self.flag;
            let p = &mut self.predicate;
            self.iter.try_fold(init, move |acc, x|{
                if p(&x) {
                    LoopState::from_try(fold(acc, x))
                } else {
                    *flag = true;
                    LoopState::Break(Try::from_ok(acc))
                }
            }).into_try()
        }
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I, P> FusedIterator for TakeWhile<I, P>
    where I: FusedIterator, P: FnMut(&I::Item) -> bool {}

/// An iterator that skips over `n` elements of `iter`.
///
/// This `struct` is created by the [`skip`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`skip`]: trait.Iterator.html#method.skip
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Skip<I> {
    iter: I,
    n: usize
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Skip<I> where I: Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.n == 0 {
            self.iter.next()
        } else {
            let old_n = self.n;
            self.n = 0;
            self.iter.nth(old_n)
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        // Can't just add n + self.n due to overflow.
        if self.n == 0 {
            self.iter.nth(n)
        } else {
            let to_skip = self.n;
            self.n = 0;
            // nth(n) skips n+1
            if self.iter.nth(to_skip-1).is_none() {
                return None;
            }
            self.iter.nth(n)
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count().saturating_sub(self.n)
    }

    #[inline]
    fn last(mut self) -> Option<I::Item> {
        if self.n == 0 {
            self.iter.last()
        } else {
            let next = self.next();
            if next.is_some() {
                // recurse. n should be 0.
                self.last().or(next)
            } else {
                None
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        let lower = lower.saturating_sub(self.n);
        let upper = upper.map(|x| x.saturating_sub(self.n));

        (lower, upper)
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let n = self.n;
        self.n = 0;
        if n > 0 {
            // nth(n) skips n+1
            if self.iter.nth(n - 1).is_none() {
                return Try::from_ok(init);
            }
        }
        self.iter.try_fold(init, fold)
    }

    #[inline]
    fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        if self.n > 0 {
            // nth(n) skips n+1
            if self.iter.nth(self.n - 1).is_none() {
                return init;
            }
        }
        self.iter.fold(init, fold)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Skip<I> where I: ExactSizeIterator {}

#[stable(feature = "double_ended_skip_iterator", since = "1.9.0")]
impl<I> DoubleEndedIterator for Skip<I> where I: DoubleEndedIterator + ExactSizeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len() > 0 {
            self.iter.next_back()
        } else {
            None
        }
    }

    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let mut n = self.len();
        if n == 0 {
            Try::from_ok(init)
        } else {
            self.iter.try_rfold(init, move |acc, x| {
                n -= 1;
                let r = fold(acc, x);
                if n == 0 { LoopState::Break(r) }
                else { LoopState::from_try(r) }
            }).into_try()
        }
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Skip<I> where I: FusedIterator {}

/// An iterator that only iterates over the first `n` iterations of `iter`.
///
/// This `struct` is created by the [`take`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`take`]: trait.Iterator.html#method.take
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Take<I> {
    iter: I,
    n: usize
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Take<I> where I: Iterator{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.n != 0 {
            self.n -= 1;
            self.iter.next()
        } else {
            None
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        if self.n > n {
            self.n -= n + 1;
            self.iter.nth(n)
        } else {
            if self.n > 0 {
                self.iter.nth(self.n - 1);
                self.n = 0;
            }
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        let lower = cmp::min(lower, self.n);

        let upper = match upper {
            Some(x) if x < self.n => Some(x),
            _ => Some(self.n)
        };

        (lower, upper)
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if self.n == 0 {
            Try::from_ok(init)
        } else {
            let n = &mut self.n;
            self.iter.try_fold(init, move |acc, x| {
                *n -= 1;
                let r = fold(acc, x);
                if *n == 0 { LoopState::Break(r) }
                else { LoopState::from_try(r) }
            }).into_try()
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Take<I> where I: ExactSizeIterator {}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Take<I> where I: FusedIterator {}

#[unstable(feature = "trusted_len", issue = "37572")]
unsafe impl<I: TrustedLen> TrustedLen for Take<I> {}

/// An iterator to maintain state while iterating another iterator.
///
/// This `struct` is created by the [`scan`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`scan`]: trait.Iterator.html#method.scan
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Scan<I, St, F> {
    iter: I,
    f: F,
    state: St,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, St: fmt::Debug, F> fmt::Debug for Scan<I, St, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Scan")
            .field("iter", &self.iter)
            .field("state", &self.state)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<B, I, St, F> Iterator for Scan<I, St, F> where
    I: Iterator,
    F: FnMut(&mut St, I::Item) -> Option<B>,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().and_then(|a| (self.f)(&mut self.state, a))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the scan function
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let state = &mut self.state;
        let f = &mut self.f;
        self.iter.try_fold(init, move |acc, x| {
            match f(state, x) {
                None => LoopState::Break(Try::from_ok(acc)),
                Some(x) => LoopState::from_try(fold(acc, x)),
            }
        }).into_try()
    }
}

/// An iterator that maps each element to an iterator, and yields the elements
/// of the produced iterators.
///
/// This `struct` is created by the [`flat_map`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`flat_map`]: trait.Iterator.html#method.flat_map
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct FlatMap<I, U: IntoIterator, F> {
    iter: I,
    f: F,
    frontiter: Option<U::IntoIter>,
    backiter: Option<U::IntoIter>,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, U: IntoIterator, F> fmt::Debug for FlatMap<I, U, F>
    where U::IntoIter: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FlatMap")
            .field("iter", &self.iter)
            .field("frontiter", &self.frontiter)
            .field("backiter", &self.backiter)
            .finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, U: IntoIterator, F> Iterator for FlatMap<I, U, F>
    where F: FnMut(I::Item) -> U,
{
    type Item = U::Item;

    #[inline]
    fn next(&mut self) -> Option<U::Item> {
        loop {
            if let Some(ref mut inner) = self.frontiter {
                if let Some(x) = inner.by_ref().next() {
                    return Some(x)
                }
            }
            match self.iter.next().map(&mut self.f) {
                None => return self.backiter.as_mut().and_then(|it| it.next()),
                next => self.frontiter = next.map(IntoIterator::into_iter),
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (flo, fhi) = self.frontiter.as_ref().map_or((0, Some(0)), |it| it.size_hint());
        let (blo, bhi) = self.backiter.as_ref().map_or((0, Some(0)), |it| it.size_hint());
        let lo = flo.saturating_add(blo);
        match (self.iter.size_hint(), fhi, bhi) {
            ((0, Some(0)), Some(a), Some(b)) => (lo, a.checked_add(b)),
            _ => (lo, None)
        }
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, mut init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if let Some(ref mut front) = self.frontiter {
            init = front.try_fold(init, &mut fold)?;
        }
        self.frontiter = None;

        {
            let f = &mut self.f;
            let frontiter = &mut self.frontiter;
            init = self.iter.try_fold(init, |acc, x| {
                let mut mid = f(x).into_iter();
                let r = mid.try_fold(acc, &mut fold);
                *frontiter = Some(mid);
                r
            })?;
        }
        self.frontiter = None;

        if let Some(ref mut back) = self.backiter {
            init = back.try_fold(init, &mut fold)?;
        }
        self.backiter = None;

        Try::from_ok(init)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.frontiter.into_iter()
            .chain(self.iter.map(self.f).map(U::into_iter))
            .chain(self.backiter)
            .fold(init, |acc, iter| iter.fold(acc, &mut fold))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, U, F> DoubleEndedIterator for FlatMap<I, U, F> where
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
    U::IntoIter: DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<U::Item> {
        loop {
            if let Some(ref mut inner) = self.backiter {
                if let Some(y) = inner.next_back() {
                    return Some(y)
                }
            }
            match self.iter.next_back().map(&mut self.f) {
                None => return self.frontiter.as_mut().and_then(|it| it.next_back()),
                next => self.backiter = next.map(IntoIterator::into_iter),
            }
        }
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, mut init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if let Some(ref mut back) = self.backiter {
            init = back.try_rfold(init, &mut fold)?;
        }
        self.backiter = None;

        {
            let f = &mut self.f;
            let backiter = &mut self.backiter;
            init = self.iter.try_rfold(init, |acc, x| {
                let mut mid = f(x).into_iter();
                let r = mid.try_rfold(acc, &mut fold);
                *backiter = Some(mid);
                r
            })?;
        }
        self.backiter = None;

        if let Some(ref mut front) = self.frontiter {
            init = front.try_rfold(init, &mut fold)?;
        }
        self.frontiter = None;

        Try::from_ok(init)
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.frontiter.into_iter()
            .chain(self.iter.map(self.f).map(U::into_iter))
            .chain(self.backiter)
            .rfold(init, |acc, iter| iter.rfold(acc, &mut fold))
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I, U, F> FusedIterator for FlatMap<I, U, F>
    where I: FusedIterator, U: IntoIterator, F: FnMut(I::Item) -> U {}

/// An iterator that yields `None` forever after the underlying iterator
/// yields `None` once.
///
/// This `struct` is created by the [`fuse`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`fuse`]: trait.Iterator.html#method.fuse
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Fuse<I> {
    iter: I,
    done: bool
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> FusedIterator for Fuse<I> where I: Iterator {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> Iterator for Fuse<I> where I: Iterator {
    type Item = <I as Iterator>::Item;

    #[inline]
    default fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.done {
            None
        } else {
            let next = self.iter.next();
            self.done = next.is_none();
            next
        }
    }

    #[inline]
    default fn nth(&mut self, n: usize) -> Option<I::Item> {
        if self.done {
            None
        } else {
            let nth = self.iter.nth(n);
            self.done = nth.is_none();
            nth
        }
    }

    #[inline]
    default fn last(self) -> Option<I::Item> {
        if self.done {
            None
        } else {
            self.iter.last()
        }
    }

    #[inline]
    default fn count(self) -> usize {
        if self.done {
            0
        } else {
            self.iter.count()
        }
    }

    #[inline]
    default fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            (0, Some(0))
        } else {
            self.iter.size_hint()
        }
    }

    #[inline]
    default fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if self.done {
            Try::from_ok(init)
        } else {
            let acc = self.iter.try_fold(init, fold)?;
            self.done = true;
            Try::from_ok(acc)
        }
    }

    #[inline]
    default fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        if self.done {
            init
        } else {
            self.iter.fold(init, fold)
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I> DoubleEndedIterator for Fuse<I> where I: DoubleEndedIterator {
    #[inline]
    default fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
        if self.done {
            None
        } else {
            let next = self.iter.next_back();
            self.done = next.is_none();
            next
        }
    }

    #[inline]
    default fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        if self.done {
            Try::from_ok(init)
        } else {
            let acc = self.iter.try_rfold(init, fold)?;
            self.done = true;
            Try::from_ok(acc)
        }
    }

    #[inline]
    default fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        if self.done {
            init
        } else {
            self.iter.rfold(init, fold)
        }
    }
}

unsafe impl<I> TrustedRandomAccess for Fuse<I>
    where I: TrustedRandomAccess,
{
    unsafe fn get_unchecked(&mut self, i: usize) -> I::Item {
        self.iter.get_unchecked(i)
    }

    fn may_have_side_effect() -> bool {
        I::may_have_side_effect()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I> Iterator for Fuse<I> where I: FusedIterator {
    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.iter.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        self.iter.nth(n)
    }

    #[inline]
    fn last(self) -> Option<I::Item> {
        self.iter.last()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        self.iter.try_fold(init, fold)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, fold)
    }
}

#[unstable(feature = "fused", reason = "recently added", issue = "35602")]
impl<I> DoubleEndedIterator for Fuse<I>
    where I: DoubleEndedIterator + FusedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
        self.iter.next_back()
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        self.iter.try_rfold(init, fold)
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.rfold(init, fold)
    }
}


#[stable(feature = "rust1", since = "1.0.0")]
impl<I> ExactSizeIterator for Fuse<I> where I: ExactSizeIterator {
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

/// An iterator that calls a function with a reference to each element before
/// yielding it.
///
/// This `struct` is created by the [`inspect`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`inspect`]: trait.Iterator.html#method.inspect
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct Inspect<I, F> {
    iter: I,
    f: F,
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<I: fmt::Debug, F> fmt::Debug for Inspect<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Inspect")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I: Iterator, F> Inspect<I, F> where F: FnMut(&I::Item) {
    #[inline]
    fn do_inspect(&mut self, elt: Option<I::Item>) -> Option<I::Item> {
        if let Some(ref a) = elt {
            (self.f)(a);
        }

        elt
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, F> Iterator for Inspect<I, F> where F: FnMut(&I::Item) {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let next = self.iter.next();
        self.do_inspect(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn try_fold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_fold(init, move |acc, item| { f(&item); fold(acc, item) })
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.fold(init, move |acc, item| { f(&item); fold(acc, item) })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator, F> DoubleEndedIterator for Inspect<I, F>
    where F: FnMut(&I::Item),
{
    #[inline]
    fn next_back(&mut self) -> Option<I::Item> {
        let next = self.iter.next_back();
        self.do_inspect(next)
    }

    #[inline]
    fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, mut fold: Fold) -> R where
        Self: Sized, Fold: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc>
    {
        let f = &mut self.f;
        self.iter.try_rfold(init, move |acc, item| { f(&item); fold(acc, item) })
    }

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
        where Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut f = self.f;
        self.iter.rfold(init, move |acc, item| { f(&item); fold(acc, item) })
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator, F> ExactSizeIterator for Inspect<I, F>
    where F: FnMut(&I::Item)
{
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

#[unstable(feature = "fused", issue = "35602")]
impl<I: FusedIterator, F> FusedIterator for Inspect<I, F>
    where F: FnMut(&I::Item) {}
