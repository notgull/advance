// Boost Software License - Version 1.0 - August 17th, 2003
//
// Permission is hereby granted, free of charge, to any person or organization
// obtaining a copy of the software and accompanying documentation covered by
// this license (the "Software") to use, reproduce, display, distribute,
// execute, and transmit the Software, and to prepare derivative works of the
// Software, and to permit third-parties to whom the Software is furnished to
// do so, all subject to the following:
//
// The copyright notices in the Software and this entire statement, including
// the above license grant, this restriction and the following disclaimer,
// must be included in all copies of the Software, in whole or in part, and
// all derivative works of the Software, unless such copies or derivative
// works are solely in the form of machine-executable object code generated by
// a source language processor.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT
// SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE
// FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Provides a function for advancing an [`IoSlice`] by a given number of bytes.
//!
//! Unfortunately, [`IoSlice::advance()`] is currently a nightly-only function.
//! The [`advance`] function in this crate does the same thing, but can be used
//! on Rust going back to 1.36.0.
//!
//! [`IoSlice`]: https://doc.rust-lang.org/std/io/struct.IoSlice.html
//! [`IoSlice::advance()`]: https://doc.rust-lang.org/std/io/struct.IoSlice.html#method.advance
//! [`advance`]: fn.advance.html

use std::io::IoSlice;

/// Advances the given [`IoSlice`] by `n` bytes.
///
/// This is equivalent to [`IoSlice::advance()`], but it can be used on Stable
/// Rust.
///
/// ## Example
///
/// ```rust
/// use advance::advance;
/// use std::io::IoSlice;
///
/// let mut slice = IoSlice::new(&[1, 2, 3, 4, 5]);
/// advance(&mut slice, 2);
/// assert_eq!(&*slice, &[3, 4, 5]);
/// ```
///
/// [`IoSlice`]: https://doc.rust-lang.org/std/io/struct.IoSlice.html
/// [`IoSlice::advance()`]: https://doc.rust-lang.org/stable/std/io/struct.IoSlice.html#method.advance
pub fn advance(slice: &mut IoSlice<'_>, len: usize) {
    // SAFETY: we know that both of these slices will have the same lifetime
    let ptr = slice
        .get(len..)
        .expect("Tried to advance past end of slice") as *const [u8];
    *slice = IoSlice::new(unsafe { &*ptr });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advance_works() {
        let mut slice = IoSlice::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        advance(&mut slice, 3);
        assert_eq!(&*slice, &[4, 5, 6, 7, 8, 9, 10]);
    }
}
