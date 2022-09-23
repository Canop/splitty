//! A no-std string splitter for which spaces between
//! quotes aren't separators.
//!
//!
//! ```
//! use splitty::*;
//!
//! let cmd = "xterm -e \"vi /some/path\"";
//!
//! let mut token = split_unquoted_char(cmd, ' ')
//!     .unwrap_quotes(true);
//!
//! assert_eq!(token.next(), Some("xterm"));
//! assert_eq!(token.next(), Some("-e"));
//! assert_eq!(token.next(), Some("vi /some/path"));
//! assert_eq!(token.next(), None);
//! ```
//!
//! Quotes not starting or ending a substring are handled as ordinary characters.
//!
//! Splitty has a limited set of features but is tested for corner-cases:
//!
//! ```
//! use splitty::*;
//!
//! let cmd = r#" a  "2 * 试" x"x "z "#;
//!
//! let mut token = split_unquoted_whitespace(cmd)
//!     .unwrap_quotes(true);
//!
//! assert_eq!(token.next(), Some("a"));
//! assert_eq!(token.next(), Some("2 * 试"));
//! assert_eq!(token.next(), Some("x\"x"));
//! assert_eq!(token.next(), Some("\"z "));
//! assert_eq!(token.next(), None);
//! ```

#![no_std]

mod split_unquoted_char;

pub use split_unquoted_char::*;
