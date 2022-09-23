/// A string splitter splitting on a specific char unless it's
///  part of a substring starting and ending with a quote (`"`).
/// Quotes around a substring are removed if required.
pub struct SplitUnquotedChar<'s> {
    src: &'s str,
    unwrap_quotes: bool,
    delimitor: char,
}

impl<'s> SplitUnquotedChar<'s> {
    /// Create a new token iterator
    ///
    /// ```
    /// let cmd = r#"Type "rhit -p blog""#;
    ///
    /// let mut split = splitty::SplitUnquotedChar::new(cmd, ' ');
    /// assert_eq!(split.next(), Some("Type"));
    /// assert_eq!(split.next(), Some("\"rhit -p blog\""));
    /// assert_eq!(split.next(), None);
    ///
    /// let mut split = splitty::SplitUnquotedChar::new(cmd, ' ')
    ///     .unwrap_quotes(true);
    /// assert_eq!(split.next(), Some("Type"));
    /// assert_eq!(split.next(), Some("rhit -p blog"));
    /// assert_eq!(split.next(), None);
    /// ```
    pub fn new(src: &'s str, delimitor: char) -> Self {
        Self {
            src,
            unwrap_quotes: false,
            delimitor,
        }
    }
    /// Set whether token starting and ending with a quote
    /// should have them removed
    pub fn unwrap_quotes(&self, b: bool) -> Self {
        Self {
            src: self.src,
            unwrap_quotes: b,
            delimitor: self.delimitor,
        }
    }
}

impl<'s> Iterator for SplitUnquotedChar<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        // we ignore spaces at the start
        self.src = self.src.trim_start();
        let mut char_indices = self.src.char_indices();
        if let Some((_, c0)) = char_indices.next() {
            let mut previous = c0;
            for (bi, c) in self.src.char_indices() {
                if c == self.delimitor {
                    if c0 == '"' {
                        if bi == 1 || previous != '"' {
                            previous = c;
                            continue;
                        }
                        // the first and last quotes aren't part of the
                        // returned token
                        let token = if self.unwrap_quotes {
                            &self.src[1..bi - 1]
                        } else {
                            self.src
                        };
                        self.src = &self.src[bi..];
                        return Some(token);
                    }
                    let token = &self.src[..bi];
                    self.src = &self.src[bi..];
                    return Some(token);
                }
                previous = c;
            }
            let unwrap = self.unwrap_quotes && c0 == '"' && previous == '"' && self.src.len() > 1;
            let token = if unwrap {
                &self.src[1..self.src.len() - 1]
            } else {
                self.src
            };
            self.src = &self.src[0..0];
            Some(token)
        } else {
            None
        }
    }
}

/// Return a new iterator of the the whitespace separated tokens
/// of the given string, taking quotes into account
pub fn split_unquoted_whitespace(src: & str) -> SplitUnquotedChar {
    SplitUnquotedChar::new(src, ' ')
}

/// Return a new iterator of the the `delimitor` separated tokens
/// of the given string, taking quotes into account
pub fn split_unquoted_char(src: &str, delimitor: char) -> SplitUnquotedChar {
    SplitUnquotedChar::new(src, delimitor)
}

#[cfg(test)]
mod split_unquoted_whitespace_test {

    use super::*;

    macro_rules! t {
        ($src:literal -> [$($token:literal),* $(,)?]) => {
            let mut split = SplitUnquotedChar::new($src, ' ')
                .unwrap_quotes(true);
            $(
                assert_eq!(split.next(), Some($token));
            )*
            assert_eq!(split.next(), None);
        }
    }

    #[test]
    fn test_split_unquoted_whitespace() {
        t!("" -> []);
        t!("    " -> []);
        t!(" a    试bc d  " -> ["a", "试bc", "d"]);
        t!("e^iπ^ = 1" -> ["e^iπ^", "=", "1"]);
        t!("1234" -> ["1234"]);
        t!("1234\"" -> ["1234\""]);
        t!(r#"""# -> [r#"""#]);
        t!(r#""a""# -> [r#"a"#]);
        t!(r#" " "# -> [r#"" "#]);
        t!(r#"a  "deux mots" b"# -> ["a", "deux mots", "b"]);
        t!(r#" " ""# -> [" "]);
        t!(r#" a  "2 * 试" x"x "z "# -> ["a", "2 * 试", "x\"x", "\"z "]);
        t!(r#"""""# -> ["\""]);
        t!(r#""""""# -> ["\"\""]);
    }
}
