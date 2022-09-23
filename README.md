[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/splitty.svg
[l1]: https://crates.io/crates/splitty

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/splitty/badge.svg
[l3]: https://docs.rs/splitty/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# splitty

A no-std string splitter for which spaces between quotes aren't separators.

Quotes not starting or ending a substring are handled as ordinary characters.

```rust
use splitty::*;

let cmd = "xterm -e \"vi /some/path\"";

let mut token = split_unquoted_char(cmd, ' ')
    .unwrap_quotes(true);

assert_eq!(token.next(), Some("xterm"));
assert_eq!(token.next(), Some("-e"));
assert_eq!(token.next(), Some("vi /some/path"));
```
