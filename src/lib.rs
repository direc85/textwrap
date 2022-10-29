//! The textwrap library provides functions for word wrapping and
//! indenting text.
//!
//! # Wrapping Text
//!
//! Wrapping text can be very useful in command-line programs where
//! you want to format dynamic output nicely so it looks good in a
//! terminal. A quick example:
//!
//! ```
//! # #[cfg(feature = "smawk")] {
//! let text = "textwrap: a small library for wrapping text.";
//! assert_eq!(textwrap::wrap(text, 18),
//!            vec!["textwrap: a",
//!                 "small library for",
//!                 "wrapping text."]);
//! # }
//! ```
//!
//! The [`wrap()`] function returns the individual lines, use
//! [`fill()`] is you want the lines joined with `'\n'` to form a
//! `String`.
//!
//! If you enable the `hyphenation` Cargo feature, you can get
//! automatic hyphenation for a number of languages:
//!
//! ```
//! #[cfg(feature = "hyphenation")] {
//! use hyphenation::{Language, Load, Standard};
//! use textwrap::{wrap, Options, WordSplitter};
//!
//! let text = "textwrap: a small library for wrapping text.";
//! let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
//! let options = Options::new(18).word_splitter(WordSplitter::Hyphenation(dictionary));
//! assert_eq!(wrap(text, &options),
//!            vec!["textwrap: a small",
//!                 "library for wrap-",
//!                 "ping text."]);
//! }
//! ```
//!
//! See also the [`unfill()`] and [`refill()`] functions which allow
//! you to manipulate already wrapped text.
//!
//! ## Wrapping Strings at Compile Time
//!
//! If your strings are known at compile time, please take a look at
//! the procedural macros from the [textwrap-macros] crate.
//!
//! ## Displayed Width vs Byte Size
//!
//! To word wrap text, one must know the width of each word so one can
//! know when to break lines. This library will by default measure the
//! width of text using the _displayed width_, not the size in bytes.
//! The `unicode-width` Cargo feature controls this.
//!
//! This is important for non-ASCII text. ASCII characters such as `a`
//! and `!` are simple and take up one column each. This means that
//! the displayed width is equal to the string length in bytes.
//! However, non-ASCII characters and symbols take up more than one
//! byte when UTF-8 encoded: `é` is `0xc3 0xa9` (two bytes) and `⚙` is
//! `0xe2 0x9a 0x99` (three bytes) in UTF-8, respectively.
//!
//! This is why we take care to use the displayed width instead of the
//! byte count when computing line lengths. All functions in this
//! library handle Unicode characters like this when the
//! `unicode-width` Cargo feature is enabled (it is enabled by
//! default).
//!
//! # Indentation and Dedentation
//!
//! The textwrap library also offers functions for adding a prefix to
//! every line of a string and to remove leading whitespace. As an
//! example, [`indent()`] allows you to turn lines of text into a
//! bullet list:
//!
//! ```
//! let before = "\
//! foo
//! bar
//! baz
//! ";
//! let after = "\
//! * foo
//! * bar
//! * baz
//! ";
//! assert_eq!(textwrap::indent(before, "* "), after);
//! ```
//!
//! Removing leading whitespace is done with [`dedent()`]:
//!
//! ```
//! let before = "
//!     Some
//!       indented
//!         text
//! ";
//! let after = "
//! Some
//!   indented
//!     text
//! ";
//! assert_eq!(textwrap::dedent(before), after);
//! ```
//!
//! # Cargo Features
//!
//! The textwrap library can be slimmed down as needed via a number of
//! Cargo features. This means you only pay for the features you
//! actually use.
//!
//! The full dependency graph, where dashed lines indicate optional
//! dependencies, is shown below:
//!
//! <img src="https://raw.githubusercontent.com/mgeisler/textwrap/master/images/textwrap-0.16.0.svg">
//!
//! ## Default Features
//!
//! These features are enabled by default:
//!
//! * `unicode-linebreak`: enables finding words using the
//!   [unicode-linebreak] crate, which implements the line breaking
//!   algorithm described in [Unicode Standard Annex
//!   #14](https://www.unicode.org/reports/tr14/).
//!
//!   This feature can be disabled if you are happy to find words
//!   separated by ASCII space characters only. People wrapping text
//!   with emojis or East-Asian characters will want most likely want
//!   to enable this feature. See [`WordSeparator`] for details.
//!
//! * `unicode-width`: enables correct width computation of non-ASCII
//!   characters via the [unicode-width] crate. Without this feature,
//!   every [`char`] is 1 column wide, except for emojis which are 2
//!   columns wide. See [`core::display_width()`] for details.
//!
//!   This feature can be disabled if you only need to wrap ASCII
//!   text, or if the functions in [`core`] are used directly with
//!   [`core::Fragment`]s for which the widths have been computed in
//!   other ways.
//!
//! * `smawk`: enables linear-time wrapping of the whole paragraph via
//!   the [smawk] crate. See [`wrap_algorithms::wrap_optimal_fit()`]
//!   for details on the optimal-fit algorithm.
//!
//!   This feature can be disabled if you only ever intend to use
//!   [`wrap_algorithms::wrap_first_fit()`].
//!
//! <!-- begin binary-sizes -->
//!
//! With Rust 1.64.0, the size impact of the above features on your
//! binary is as follows:
//!
//! | Configuration                            |  Binary Size |    Delta |
//! | :---                                     |         ---: |     ---: |
//! | quick-and-dirty implementation           |       289 KB |     — KB |
//! | textwrap without default features        |       305 KB |    16 KB |
//! | textwrap with smawk                      |       317 KB |    28 KB |
//! | textwrap with unicode-width              |       309 KB |    20 KB |
//! | textwrap with unicode-linebreak          |       342 KB |    53 KB |
//!
//! <!-- end binary-sizes -->
//!
//! The above sizes are the stripped sizes and the binary is compiled
//! in release mode with this profile:
//!
//! ```toml
//! [profile.release]
//! lto = true
//! codegen-units = 1
//! ```
//!
//! See the [binary-sizes demo] if you want to reproduce these
//! results.
//!
//! ## Optional Features
//!
//! These Cargo features enable new functionality:
//!
//! * `terminal_size`: enables automatic detection of the terminal
//!   width via the [terminal_size] crate. See
//!   [`Options::with_termwidth()`] for details.
//!
//! * `hyphenation`: enables language-sensitive hyphenation via the
//!   [hyphenation] crate. See the [`word_splitters::WordSplitter`]
//!   trait for details.
//!
//! [unicode-linebreak]: https://docs.rs/unicode-linebreak/
//! [unicode-width]: https://docs.rs/unicode-width/
//! [smawk]: https://docs.rs/smawk/
//! [binary-sizes demo]: https://github.com/mgeisler/textwrap/tree/master/examples/binary-sizes
//! [textwrap-macros]: https://docs.rs/textwrap-macros/
//! [terminal_size]: https://docs.rs/terminal_size/
//! [hyphenation]: https://docs.rs/hyphenation/

#![doc(html_root_url = "https://docs.rs/textwrap/0.16.0")]
#![forbid(unsafe_code)] // See https://github.com/mgeisler/textwrap/issues/210
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::redundant_field_names)]

// Make `cargo test` execute the README doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctest {}

use std::borrow::Cow;

pub mod core;
#[cfg(fuzzing)]
pub mod fuzzing;
pub mod word_splitters;
pub mod wrap_algorithms;

mod columns;
mod indentation;
mod line_ending;
mod options;
mod refill;
#[cfg(feature = "terminal_size")]
mod termwidth;
mod word_separators;

pub use columns::wrap_columns;
pub use indentation::{dedent, indent};
pub use line_ending::LineEnding;
pub use options::Options;
pub use refill::{refill, unfill};
#[cfg(feature = "terminal_size")]
pub use termwidth::termwidth;
pub use word_separators::WordSeparator;
pub use word_splitters::WordSplitter;
pub use wrap_algorithms::WrapAlgorithm;

/// Fill a line of text at a given width.
///
/// The result is a [`String`], complete with newlines between each
/// line. Use [`wrap()`] if you need access to the individual lines.
///
/// The easiest way to use this function is to pass an integer for
/// `width_or_options`:
///
/// ```
/// use textwrap::fill;
///
/// assert_eq!(
///     fill("Memory safety without garbage collection.", 15),
///     "Memory safety\nwithout garbage\ncollection."
/// );
/// ```
///
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{fill, Options};
///
/// let options = Options::new(15)
///     .initial_indent("- ")
///     .subsequent_indent("  ");
/// assert_eq!(
///     fill("Memory safety without garbage collection.", &options),
///     "- Memory safety\n  without\n  garbage\n  collection."
/// );
/// ```
pub fn fill<'a, Opt>(text: &str, width_or_options: Opt) -> String
where
    Opt: Into<Options<'a>>,
{
    let options = width_or_options.into();

    if text.len() < options.width && !text.contains('\n') && options.initial_indent.is_empty() {
        String::from(text.trim_end_matches(' '))
    } else {
        fill_slow_path(text, options)
    }
}

/// Slow path for fill.
///
/// This is taken when `text` is longer than `options.width`.
fn fill_slow_path(text: &str, options: Options<'_>) -> String {
    // This will avoid reallocation in simple cases (no
    // indentation, no hyphenation).
    let mut result = String::with_capacity(text.len());

    let line_ending_str = options.line_ending.as_str();
    for (i, line) in wrap(text, options).iter().enumerate() {
        if i > 0 {
            result.push_str(line_ending_str);
        }
        result.push_str(line);
    }

    result
}

/// Wrap a line of text at a given width.
///
/// The result is a vector of lines, each line is of type [`Cow<'_,
/// str>`](Cow), which means that the line will borrow from the input
/// `&str` if possible. The lines do not have trailing whitespace,
/// including a final `'\n'`. Please use [`fill()`] if you need a
/// [`String`] instead.
///
/// The easiest way to use this function is to pass an integer for
/// `width_or_options`:
///
/// ```
/// use textwrap::wrap;
///
/// let lines = wrap("Memory safety without garbage collection.", 15);
/// assert_eq!(lines, &[
///     "Memory safety",
///     "without garbage",
///     "collection.",
/// ]);
/// ```
///
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15)
///     .initial_indent("- ")
///     .subsequent_indent("  ");
/// let lines = wrap("Memory safety without garbage collection.", &options);
/// assert_eq!(lines, &[
///     "- Memory safety",
///     "  without",
///     "  garbage",
///     "  collection.",
/// ]);
/// ```
///
/// # Optimal-Fit Wrapping
///
/// By default, `wrap` will try to ensure an even right margin by
/// finding breaks which avoid short lines. We call this an
/// “optimal-fit algorithm” since the line breaks are computed by
/// considering all possible line breaks. The alternative is a
/// “first-fit algorithm” which simply accumulates words until they no
/// longer fit on the line.
///
/// As an example, using the first-fit algorithm to wrap the famous
/// Hamlet quote “To be, or not to be: that is the question” in a
/// narrow column with room for only 10 characters looks like this:
///
/// ```
/// # use textwrap::{WrapAlgorithm::FirstFit, Options, wrap};
/// #
/// # let lines = wrap("To be, or not to be: that is the question",
/// #                  Options::new(10).wrap_algorithm(FirstFit));
/// # assert_eq!(lines.join("\n") + "\n", "\
/// To be, or
/// not to be:
/// that is
/// the
/// question
/// # ");
/// ```
///
/// Notice how the second to last line is quite narrow because
/// “question” was too large to fit? The greedy first-fit algorithm
/// doesn’t look ahead, so it has no other option than to put
/// “question” onto its own line.
///
/// With the optimal-fit wrapping algorithm, the previous lines are
/// shortened slightly in order to make the word “is” go into the
/// second last line:
///
/// ```
/// # #[cfg(feature = "smawk")] {
/// # use textwrap::{Options, WrapAlgorithm, wrap};
/// #
/// # let lines = wrap(
/// #     "To be, or not to be: that is the question",
/// #     Options::new(10).wrap_algorithm(WrapAlgorithm::new_optimal_fit())
/// # );
/// # assert_eq!(lines.join("\n") + "\n", "\
/// To be,
/// or not to
/// be: that
/// is the
/// question
/// # "); }
/// ```
///
/// Please see [`WrapAlgorithm`] for details on the choices.
///
/// # Examples
///
/// The returned iterator yields lines of type `Cow<'_, str>`. If
/// possible, the wrapped lines will borrow from the input string. As
/// an example, a hanging indentation, the first line can borrow from
/// the input, but the subsequent lines become owned strings:
///
/// ```
/// use std::borrow::Cow::{Borrowed, Owned};
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15).subsequent_indent("....");
/// let lines = wrap("Wrapping text all day long.", &options);
/// let annotated = lines
///     .iter()
///     .map(|line| match line {
///         Borrowed(text) => format!("[Borrowed] {}", text),
///         Owned(text) => format!("[Owned]    {}", text),
///     })
///     .collect::<Vec<_>>();
/// assert_eq!(
///     annotated,
///     &[
///         "[Borrowed] Wrapping text",
///         "[Owned]    ....all day",
///         "[Owned]    ....long.",
///     ]
/// );
/// ```
///
/// ## Leading and Trailing Whitespace
///
/// As a rule, leading whitespace (indentation) is preserved and
/// trailing whitespace is discarded.
///
/// In more details, when wrapping words into lines, words are found
/// by splitting the input text on space characters. One or more
/// spaces (shown here as “␣”) are attached to the end of each word:
///
/// ```text
/// "Foo␣␣␣bar␣baz" -> ["Foo␣␣␣", "bar␣", "baz"]
/// ```
///
/// These words are then put into lines. The interword whitespace is
/// preserved, unless the lines are wrapped so that the `"Foo␣␣␣"`
/// word falls at the end of a line:
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("Foo   bar baz", 10), vec!["Foo   bar", "baz"]);
/// assert_eq!(wrap("Foo   bar baz", 8), vec!["Foo", "bar baz"]);
/// ```
///
/// Notice how the trailing whitespace is removed in both case: in the
/// first example, `"bar␣"` becomes `"bar"` and in the second case
/// `"Foo␣␣␣"` becomes `"Foo"`.
///
/// Leading whitespace is preserved when the following word fits on
/// the first line. To understand this, consider how words are found
/// in a text with leading spaces:
///
/// ```text
/// "␣␣foo␣bar" -> ["␣␣", "foo␣", "bar"]
/// ```
///
/// When put into lines, the indentation is preserved if `"foo"` fits
/// on the first line, otherwise you end up with an empty line:
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("  foo bar", 8), vec!["  foo", "bar"]);
/// assert_eq!(wrap("  foo bar", 4), vec!["", "foo", "bar"]);
/// ```
pub fn wrap<'a, Opt>(text: &str, width_or_options: Opt) -> Vec<Cow<'_, str>>
where
    Opt: Into<Options<'a>>,
{
    let options: Options = width_or_options.into();
    let line_ending_str = options.line_ending.as_str();

    let mut lines = Vec::new();
    for line in text.split(line_ending_str) {
        wrap_single_line(line, &options, &mut lines);
    }

    lines
}

fn wrap_single_line<'a>(line: &'a str, options: &Options<'_>, lines: &mut Vec<Cow<'a, str>>) {
    let indent = if lines.is_empty() {
        options.initial_indent
    } else {
        options.subsequent_indent
    };
    if line.len() < options.width && indent.is_empty() {
        lines.push(Cow::from(line.trim_end_matches(' ')));
    } else {
        wrap_single_line_slow_path(line, options, lines)
    }
}

/// Wrap a single line of text.
///
/// This is taken when `line` is longer than `options.width`.
fn wrap_single_line_slow_path<'a>(
    line: &'a str,
    options: &Options<'_>,
    lines: &mut Vec<Cow<'a, str>>,
) {
    let initial_width = options
        .width
        .saturating_sub(core::display_width(options.initial_indent));
    let subsequent_width = options
        .width
        .saturating_sub(core::display_width(options.subsequent_indent));
    let line_widths = [initial_width, subsequent_width];

    let words = options.word_separator.find_words(line);
    let split_words = word_splitters::split_words(words, &options.word_splitter);
    let broken_words = if options.break_words {
        let mut broken_words = core::break_words(split_words, line_widths[1]);
        if !options.initial_indent.is_empty() {
            // Without this, the first word will always go into the
            // first line. However, since we break words based on the
            // _second_ line width, it can be wrong to unconditionally
            // put the first word onto the first line. An empty
            // zero-width word fixed this.
            broken_words.insert(0, core::Word::from(""));
        }
        broken_words
    } else {
        split_words.collect::<Vec<_>>()
    };

    let wrapped_words = options.wrap_algorithm.wrap(&broken_words, &line_widths);

    let mut idx = 0;
    for words in wrapped_words {
        let last_word = match words.last() {
            None => {
                lines.push(Cow::from(""));
                continue;
            }
            Some(word) => word,
        };

        // We assume here that all words are contiguous in `line`.
        // That is, the sum of their lengths should add up to the
        // length of `line`.
        let len = words
            .iter()
            .map(|word| word.len() + word.whitespace.len())
            .sum::<usize>()
            - last_word.whitespace.len();

        // The result is owned if we have indentation, otherwise we
        // can simply borrow an empty string.
        let mut result = if lines.is_empty() && !options.initial_indent.is_empty() {
            Cow::Owned(options.initial_indent.to_owned())
        } else if !lines.is_empty() && !options.subsequent_indent.is_empty() {
            Cow::Owned(options.subsequent_indent.to_owned())
        } else {
            // We can use an empty string here since string
            // concatenation for `Cow` preserves a borrowed value when
            // either side is empty.
            Cow::from("")
        };

        result += &line[idx..idx + len];

        if !last_word.penalty.is_empty() {
            result.to_mut().push_str(last_word.penalty);
        }

        lines.push(result);

        // Advance by the length of `result`, plus the length of
        // `last_word.whitespace` -- even if we had a penalty, we need
        // to skip over the whitespace.
        idx += len + last_word.whitespace.len();
    }
}

/// Fill `text` in-place without reallocating the input string.
///
/// This function works by modifying the input string: some `' '`
/// characters will be replaced by `'\n'` characters. The rest of the
/// text remains untouched.
///
/// Since we can only replace existing whitespace in the input with
/// `'\n'` (there is no space for `"\r\n"`), we cannot do hyphenation
/// nor can we split words longer than the line width. We also need to
/// use `AsciiSpace` as the word separator since we need `' '`
/// characters between words in order to replace some of them with a
/// `'\n'`. Indentation is also ruled out. In other words,
/// `fill_inplace(width)` behaves as if you had called [`fill()`] with
/// these options:
///
/// ```
/// # use textwrap::{core, LineEnding, Options, WordSplitter, WordSeparator, WrapAlgorithm};
/// # let width = 80;
/// Options::new(width)
///     .break_words(false)
///     .line_ending(LineEnding::LF)
///     .word_separator(WordSeparator::AsciiSpace)
///     .wrap_algorithm(WrapAlgorithm::FirstFit)
///     .word_splitter(WordSplitter::NoHyphenation);
/// ```
///
/// The wrap algorithm is [`WrapAlgorithm::FirstFit`] since this
/// is the fastest algorithm — and the main reason to use
/// `fill_inplace` is to get the string broken into newlines as fast
/// as possible.
///
/// A last difference is that (unlike [`fill()`]) `fill_inplace` can
/// leave trailing whitespace on lines. This is because we wrap by
/// inserting a `'\n'` at the final whitespace in the input string:
///
/// ```
/// let mut text = String::from("Hello   World!");
/// textwrap::fill_inplace(&mut text, 10);
/// assert_eq!(text, "Hello  \nWorld!");
/// ```
///
/// If we didn't do this, the word `World!` would end up being
/// indented. You can avoid this if you make sure that your input text
/// has no double spaces.
///
/// # Performance
///
/// In benchmarks, `fill_inplace` is about twice as fast as
/// [`fill()`]. Please see the [`linear`
/// benchmark](https://github.com/mgeisler/textwrap/blob/master/benchmarks/linear.rs)
/// for details.
pub fn fill_inplace(text: &mut String, width: usize) {
    let mut indices = Vec::new();

    let mut offset = 0;
    for line in text.split('\n') {
        let words = WordSeparator::AsciiSpace
            .find_words(line)
            .collect::<Vec<_>>();
        let wrapped_words = wrap_algorithms::wrap_first_fit(&words, &[width as f64]);

        let mut line_offset = offset;
        for words in &wrapped_words[..wrapped_words.len() - 1] {
            let line_len = words
                .iter()
                .map(|word| word.len() + word.whitespace.len())
                .sum::<usize>();

            line_offset += line_len;
            // We've advanced past all ' ' characters -- want to move
            // one ' ' backwards and insert our '\n' there.
            indices.push(line_offset - 1);
        }

        // Advance past entire line, plus the '\n' which was removed
        // by the split call above.
        offset += line.len() + 1;
    }

    let mut bytes = std::mem::take(text).into_bytes();
    for idx in indices {
        bytes[idx] = b'\n';
    }
    *text = String::from_utf8(bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};

    #[test]
    fn no_wrap() {
        assert_eq!(wrap("foo", 10), vec!["foo"]);
    }

    #[test]
    fn wrap_simple() {
        assert_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn to_be_or_not() {
        assert_eq!(
            wrap(
                "To be, or not to be, that is the question.",
                Options::new(10).wrap_algorithm(WrapAlgorithm::FirstFit)
            ),
            vec!["To be, or", "not to be,", "that is", "the", "question."]
        );
    }

    #[test]
    fn multiple_words_on_first_line() {
        assert_eq!(wrap("foo bar baz", 10), vec!["foo bar", "baz"]);
    }

    #[test]
    fn long_word() {
        assert_eq!(wrap("foo", 0), vec!["f", "o", "o"]);
    }

    #[test]
    fn long_words() {
        assert_eq!(wrap("foo bar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn max_width() {
        assert_eq!(wrap("foo bar", usize::MAX), vec!["foo bar"]);

        let text = "Hello there! This is some English text. \
                    It should not be wrapped given the extents below.";
        assert_eq!(wrap(text, usize::MAX), vec![text]);
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(wrap("  foo bar", 6), vec!["  foo", "bar"]);
    }

    #[test]
    fn leading_whitespace_empty_first_line() {
        // If there is no space for the first word, the first line
        // will be empty. This is because the string is split into
        // words like [" ", "foobar ", "baz"], which puts "foobar " on
        // the second line. We never output trailing whitespace
        assert_eq!(wrap(" foobar baz", 6), vec!["", "foobar", "baz"]);
    }

    #[test]
    fn trailing_whitespace() {
        // Whitespace is only significant inside a line. After a line
        // gets too long and is broken, the first word starts in
        // column zero and is not indented.
        assert_eq!(wrap("foo     bar     baz  ", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn issue_99() {
        // We did not reset the in_whitespace flag correctly and did
        // not handle single-character words after a line break.
        assert_eq!(
            wrap("aaabbbccc x yyyzzzwww", 9),
            vec!["aaabbbccc", "x", "yyyzzzwww"]
        );
    }

    #[test]
    fn issue_129() {
        // The dash is an em-dash which takes up four bytes. We used
        // to panic since we tried to index into the character.
        let options = Options::new(1).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(wrap("x – x", options), vec!["x", "–", "x"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_eq!(
            wrap(
                "Ｈｅｌｌｏ, Ｗｏｒｌｄ!",
                Options::new(15).word_separator(WordSeparator::AsciiSpace)
            ),
            vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]
        );

        // Wide characters are allowed to break if the
        // unicode-linebreak feature is enabled.
        #[cfg(feature = "unicode-linebreak")]
        assert_eq!(
            wrap(
                "Ｈｅｌｌｏ, Ｗｏｒｌｄ!",
                Options::new(15).word_separator(WordSeparator::UnicodeBreakProperties),
            ),
            vec!["Ｈｅｌｌｏ, Ｗ", "ｏｒｌｄ!"]
        );
    }

    #[test]
    fn empty_line_is_indented() {
        // Previously, indentation was not applied to empty lines.
        // However, this is somewhat inconsistent and undesirable if
        // the indentation is something like a border ("| ") which you
        // want to apply to all lines, empty or not.
        let options = Options::new(10).initial_indent("!!!");
        assert_eq!(fill("", &options), "!!!");
    }

    #[test]
    fn indent_single_line() {
        let options = Options::new(10).initial_indent(">>>"); // No trailing space
        assert_eq!(fill("foo", &options), ">>>foo");
    }

    #[test]
    fn indent_first_emoji() {
        let options = Options::new(10).initial_indent("👉👉");
        assert_eq!(
            wrap("x x x x x x x x x x x x x", &options),
            vec!["👉👉x x x", "x x x x x", "x x x x x"]
        );
    }

    #[test]
    fn indent_multiple_lines() {
        let options = Options::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(
            wrap("foo bar baz", &options),
            vec!["* foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn only_initial_indent_multiple_lines() {
        let options = Options::new(10).initial_indent("  ");
        assert_eq!(wrap("foo\nbar\nbaz", &options), vec!["  foo", "bar", "baz"]);
    }

    #[test]
    fn only_subsequent_indent_multiple_lines() {
        let options = Options::new(10).subsequent_indent("  ");
        assert_eq!(
            wrap("foo\nbar\nbaz", &options),
            vec!["foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn indent_break_words() {
        let options = Options::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrap("foobarbaz", &options), vec!["* foo", "  bar", "  baz"]);
    }

    #[test]
    fn initial_indent_break_words() {
        // This is a corner-case showing how the long word is broken
        // according to the width of the subsequent lines. The first
        // fragment of the word no longer fits on the first line,
        // which ends up being pure indentation.
        let options = Options::new(5).initial_indent("-->");
        assert_eq!(wrap("foobarbaz", &options), vec!["-->", "fooba", "rbaz"]);
    }

    #[test]
    fn hyphens() {
        assert_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-", &options), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let options = Options::new(5).break_words(false);
        assert_eq!(
            wrap("The --foo-bar flag.", &options),
            vec!["The", "--foo-", "bar", "flag."]
        );
    }

    #[test]
    fn repeated_hyphens() {
        let options = Options::new(4).break_words(false);
        assert_eq!(wrap("foo--bar", &options), vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foo(-)bar", &options), vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-baz", &options), vec!["foobar-", "baz"]);
    }

    #[test]
    fn multiple_unbroken_words_issue_193() {
        let options = Options::new(3).break_words(false);
        assert_eq!(
            wrap("small large tiny", &options),
            vec!["small", "large", "tiny"]
        );
        assert_eq!(
            wrap("small  large   tiny", &options),
            vec!["small", "large", "tiny"]
        );
    }

    #[test]
    fn very_narrow_lines_issue_193() {
        let options = Options::new(1).break_words(false);
        assert_eq!(wrap("fooo x y", &options), vec!["fooo", "x", "y"]);
        assert_eq!(wrap("fooo   x     y", &options), vec!["fooo", "x", "y"]);
    }

    #[test]
    fn simple_hyphens() {
        let options = Options::new(8).word_splitter(WordSplitter::HyphenSplitter);
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo bar-", "baz"]);
    }

    #[test]
    fn no_hyphenation() {
        let options = Options::new(8).word_splitter(WordSplitter::NoHyphenation);
        assert_eq!(wrap("foo bar-baz", &options), vec!["foo", "bar-baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_double_hyphenation() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Internatio", "nalization"]
        );

        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("Internationalization", &options),
            vec!["Interna-", "tionaliza-", "tion"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_issue_158() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("participation is the key to success", &options),
            vec!["participat", "ion is", "the key to", "success"]
        );

        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("participation is the key to success", &options),
            vec!["partici-", "pation is", "the key to", "success"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn split_len_hyphenation() {
        // Test that hyphenation takes the width of the whitespace
        // into account.
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(15).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("garbage   collection", &options),
            vec!["garbage   col-", "lection"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn borrowed_lines() {
        // Lines that end with an extra hyphen are owned, the final
        // line is borrowed.
        use std::borrow::Cow::{Borrowed, Owned};
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        let lines = wrap("Internationalization", &options);
        assert_eq!(lines, vec!["Interna-", "tionaliza-", "tion"]);
        if let Borrowed(s) = lines[0] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Borrowed(s) = lines[1] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Owned(ref s) = lines[2] {
            assert!(false, "should not have been owned: {:?}", s);
        }
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(8).break_words(false);
        assert_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffinated"]
        );

        let options = options.word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("over-caffinated", &options),
            vec!["over-", "caffi-", "nated"]
        );
    }

    #[test]
    fn break_words() {
        assert_eq!(wrap("foobarbaz", 3), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn break_words_wide_characters() {
        // Even the poor man's version of `ch_width` counts these
        // characters as wide.
        let options = Options::new(5).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(wrap("Ｈｅｌｌｏ", options), vec!["Ｈｅ", "ｌｌ", "ｏ"]);
    }

    #[test]
    fn break_words_zero_width() {
        assert_eq!(wrap("foobar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn break_long_first_word() {
        assert_eq!(wrap("testx y", 4), vec!["test", "x y"]);
    }

    #[test]
    fn break_words_line_breaks() {
        assert_eq!(fill("ab\ncdefghijkl", 5), "ab\ncdefg\nhijkl");
        assert_eq!(fill("abcdefgh\nijkl", 5), "abcde\nfgh\nijkl");
    }

    #[test]
    fn break_words_empty_lines() {
        assert_eq!(
            fill("foo\nbar", &Options::new(2).break_words(false)),
            "foo\nbar"
        );
    }

    #[test]
    fn preserve_line_breaks() {
        assert_eq!(fill("", 80), "");
        assert_eq!(fill("\n", 80), "\n");
        assert_eq!(fill("\n\n\n", 80), "\n\n\n");
        assert_eq!(fill("test\n", 80), "test\n");
        assert_eq!(fill("test\n\na\n\n", 80), "test\n\na\n\n");
        assert_eq!(
            fill(
                "1 3 5 7\n1 3 5 7",
                Options::new(7).wrap_algorithm(WrapAlgorithm::FirstFit)
            ),
            "1 3 5 7\n1 3 5 7"
        );
        assert_eq!(
            fill(
                "1 3 5 7\n1 3 5 7",
                Options::new(5).wrap_algorithm(WrapAlgorithm::FirstFit)
            ),
            "1 3 5\n7\n1 3 5\n7"
        );
    }

    #[test]
    fn preserve_line_breaks_with_whitespace() {
        assert_eq!(fill("  ", 80), "");
        assert_eq!(fill("  \n  ", 80), "\n");
        assert_eq!(fill("  \n \n  \n ", 80), "\n\n\n");
    }

    #[test]
    fn non_breaking_space() {
        let options = Options::new(5).break_words(false);
        assert_eq!(fill("foo bar baz", &options), "foo bar baz");
    }

    #[test]
    fn non_breaking_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(fill("foo‑bar‑baz", &options), "foo‑bar‑baz");
    }

    #[test]
    fn fill_simple() {
        assert_eq!(fill("foo bar baz", 10), "foo bar\nbaz");
    }

    #[test]
    fn fill_colored_text() {
        // The words are much longer than 6 bytes, but they remain
        // intact after filling the text.
        let green_hello = "\u{1b}[0m\u{1b}[32mHello\u{1b}[0m";
        let blue_world = "\u{1b}[0m\u{1b}[34mWorld!\u{1b}[0m";
        assert_eq!(
            fill(&(String::from(green_hello) + " " + blue_world), 6),
            String::from(green_hello) + "\n" + blue_world
        );
    }

    #[test]
    fn fill_unicode_boundary() {
        // https://github.com/mgeisler/textwrap/issues/390
        fill("\u{1b}!Ͽ", 10);
    }

    #[test]
    fn fill_inplace_empty() {
        let mut text = String::from("");
        fill_inplace(&mut text, 80);
        assert_eq!(text, "");
    }

    #[test]
    fn fill_inplace_simple() {
        let mut text = String::from("foo bar baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\nbaz");
    }

    #[test]
    fn fill_inplace_multiple_lines() {
        let mut text = String::from("Some text to wrap over multiple lines");
        fill_inplace(&mut text, 12);
        assert_eq!(text, "Some text to\nwrap over\nmultiple\nlines");
    }

    #[test]
    fn fill_inplace_long_word() {
        let mut text = String::from("Internationalization is hard");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "Internationalization\nis hard");
    }

    #[test]
    fn fill_inplace_no_hyphen_splitting() {
        let mut text = String::from("A well-chosen example");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "A\nwell-chosen\nexample");
    }

    #[test]
    fn fill_inplace_newlines() {
        let mut text = String::from("foo bar\n\nbaz\n\n\n");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\n\nbaz\n\n\n");
    }

    #[test]
    fn fill_inplace_newlines_reset_line_width() {
        let mut text = String::from("1 3 5\n1 3 5 7 9\n1 3 5 7 9 1 3");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "1 3 5\n1 3 5 7 9\n1 3 5 7 9\n1 3");
    }

    #[test]
    fn fill_inplace_leading_whitespace() {
        let mut text = String::from("  foo bar baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "  foo bar\nbaz");
    }

    #[test]
    fn fill_inplace_trailing_whitespace() {
        let mut text = String::from("foo bar baz  ");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo bar\nbaz  ");
    }

    #[test]
    fn fill_inplace_interior_whitespace() {
        // To avoid an unwanted indentation of "baz", it is important
        // to replace the final ' ' with '\n'.
        let mut text = String::from("foo  bar    baz");
        fill_inplace(&mut text, 10);
        assert_eq!(text, "foo  bar   \nbaz");
    }
}
