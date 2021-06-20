# historify

convert rust source code's identify to Historical characters.

Currently work in progress.

## Example

FIXME update examples.

input

```rust
fn main() {
    println!("Hello, world!");
}
```

output

```rust
#![allow(uncommon_codepoints)]
mod 𓍯𓃀𓊃𓎼𓎡𓃋𓂧𓆑𓅓𓂋𓋴𓏘 {
    #![allow(unused_imports)]
    use std::borrow::ToOwned as 𓎡𓂋𓎡𓋴𓅱𓃎𓃍𓃋𓍯𓅓𓆑𓋴𓄿;
    use std::boxed::Box as 𓇋𓆓𓎡𓋴𓎡𓋴𓏘;
    use std::clone::Clone as 𓇋𓈖𓅱𓎼𓃎𓃋𓏏𓆑;
    use std::cmp::{
        Eq as 𓇋𓆑𓇋𓏘, Ord as 𓆓𓃍𓊃𓎼𓇋, PartialEq as 𓎡𓃀𓏘𓎡𓋴𓇋𓃍𓂧𓆓𓅓𓆑𓅱𓇋𓎡𓃌𓇋, PartialOrd as 𓎡𓃀𓏘𓎡𓋴𓇋𓃍𓂧𓆓𓅓𓆑𓅱𓇋𓃎𓃌𓏏𓇋,
    };
    use std::convert::{
        AsMut as 𓇋𓆑𓊃𓅱𓃊𓃍𓃭𓅱, AsRef as 𓇋𓆑𓊃𓆑𓇋𓊃𓃭𓎼, From as 𓇋𓊃𓊃𓎼𓃎𓃋𓇋, Into as 𓆓𓆑𓎡𓋴𓎛𓇋𓃋𓇋
    };
    use std::default::Default as 𓇋𓂋𓋴𓅱𓅓𓇋𓃭𓆑𓈖𓂋𓃊𓄿;
    use std::iter::{
        DoubleEndedIterator as 𓇋𓂋𓎡𓋴𓎡𓋴𓎡𓇋𓏏𓅓𓅓𓆑𓎡𓅱𓃌𓊃𓂧𓆑𓅓𓂋𓇋𓎡𓋴𓇋𓊃𓃭𓋴𓅓𓆑𓃊𓎼𓃎𓃌𓏘,
        ExactSizeIterator as 𓇋𓆑𓃌𓎼𓎡𓇋𓃋𓅱𓎡𓈖𓅱𓎡𓋴𓅱𓊃𓎡𓆓𓍯𓂋𓋴𓎡𓋴𓇋𓇋𓃭𓅱𓈖𓃍𓊃𓄿, Extend as 𓇋𓆑𓃌𓎛𓇋𓊃𓃭𓍯𓅓𓏘,
        IntoIterator as 𓆓𓆑𓎡𓋴𓎛𓇋𓃋𓃊𓆓𓍯𓂋𓋴𓎡𓋴𓇋𓇋𓃭𓅱𓈖𓃍𓊃𓄿, Iterator as 𓆓𓆑𓃊𓎼𓎡𓃌𓏏𓃀𓍯𓂋𓎡𓋴𓎡𓋴𓇋,
    };
    use std::marker::{
        Copy as 𓇋𓈖𓎡𓋴𓎡𓋴𓄿𓃎𓇋, Send as 𓎡𓈖𓋴𓅱𓃌𓊃𓄿, Sized as 𓎡𓈖𓅱𓎡𓋴𓅱𓊃𓃭𓇋, Sync as 𓎡𓈖𓃌𓅱𓃌𓇋𓇋, Unpin as 𓎡𓆑𓎡𓋴𓎛𓄿𓃊𓃭𓍯,
    };
    use std::mem::drop as 𓅓𓂋𓊃𓎼𓃎𓃌𓄿;
    use std::ops::{
        Drop as 𓇋𓂋𓊃𓎼𓃎𓃌𓄿, Fn as 𓇋𓊃𓎡𓋴𓄿, FnMut as 𓇋𓊃𓎡𓋴𓇋𓃊𓃍𓃭𓅱, FnOnce as 𓇋𓊃𓎡𓋴𓇋𓃎𓃋𓏏𓂧𓅓𓅱
    };
    use std::option::Option::{
        self as 𓆓𓃍𓇋𓎛𓇋𓃊𓃭𓏤𓈖𓇋, None as 𓆓𓊃𓎡𓋴𓅱𓃌𓊃𓇋, Some as 𓎡𓈖𓎡𓋴𓅱𓃊𓊃𓇋
    };
    use std::result::Result::{
        self as 𓎡𓆓𓋴𓎡𓋴𓎼𓃍𓃭𓅓𓍯𓏘, Err as 𓇋𓆑𓊃𓎛𓇋, Ok as 𓆓𓃍𓆑𓏘
    };
    use std::string::{
        String as 𓎡𓈖𓃊𓎛𓇋𓃊𓃭𓍯𓅓𓃌, ToString as 𓎡𓂋𓎡𓋴𓆑𓎼𓃍𓂧𓋴𓈖𓆑𓎡𓋴𓎼𓍯
    };
    use std::vec::Vec as 𓎡𓊃𓋴𓅱𓎼;
}
#[allow(unused_imports)]
use std::{
    assert as 𓅓𓆑𓊃𓎡𓋴𓎼𓊃𓃭𓋴𓍯𓏘, assert_eq as 𓅓𓆑𓊃𓎡𓋴𓎼𓊃𓃭𓋴𓍯𓂋𓏤𓅱𓎡𓃌𓇋, assert_ne as 𓅓𓆑𓊃𓎡𓋴𓎼𓊃𓃭𓋴𓍯𓂋𓏤𓅱𓃌𓊃𓇋,
    cfg as 𓅓𓈖𓏏𓎼𓍯, column as 𓅓𓈖𓎡𓋴𓅱𓇋𓃍𓃭𓈖𓈖𓇋, compile_error as 𓅓𓈖𓎡𓋴𓅱𓃊𓃌𓂧𓆓𓈖𓂋𓋴𓆑𓃎𓊃𓃭𓋴𓍯𓆓𓎡𓋴𓎡𓋴𓇋,
    concat as 𓅓𓈖𓎡𓋴𓅱𓃌𓇋𓃋𓃀𓍯𓏘, dbg as 𓅓𓂋𓂋𓎼𓍯, debug_assert as 𓅓𓂋𓋴𓅱𓇋𓃍𓃭𓎛𓃭𓃍𓏘𓎡𓋴𓎼𓃌𓃋𓆑𓍯𓆓𓃊𓄿,
    debug_assert_eq as 𓅓𓂋𓋴𓅱𓇋𓃍𓃭𓎛𓃭𓃍𓏘𓎡𓋴𓎼𓃌𓃋𓆑𓍯𓆓𓃊𓆑𓃎𓊃𓃭𓂋, debug_assert_ne as 𓅓𓂋𓋴𓅱𓇋𓃍𓃭𓎛𓃭𓃍𓏘𓎡𓋴𓎼𓃌𓃋𓆑𓍯𓆓𓃊𓆑𓃎𓃋𓏏𓆑,
    env as 𓅓𓆑𓎡𓋴𓎛𓅓, eprint as 𓅓𓆑𓇋𓎛𓇋𓃊𓃭𓍯𓍯𓏘, eprintln as 𓅓𓆑𓇋𓎛𓇋𓃊𓃭𓍯𓍯𓂋𓅱𓎼𓃌, file as 𓅓𓊃𓅱𓅱𓇋𓊃𓇋,
    format as 𓅓𓊃𓎡𓋴𓎡𓋴𓇋𓃋𓃭𓃀𓍯𓏘, format_args as 𓅓𓊃𓎡𓋴𓎡𓋴𓇋𓃋𓃭𓃀𓍯𓂋𓏤𓅱𓎡𓃌𓏏𓎛𓍯𓅓, include as 𓈖𓆑𓎡𓋴𓎼𓎼𓃋𓂧𓆑𓅓𓂋𓋴𓏘,
    include_bytes as 𓈖𓆑𓎡𓋴𓎼𓎼𓃋𓂧𓆑𓅓𓂋𓋴𓆑𓃎𓇋𓏏𓊃𓍯𓂋𓋴𓎡𓋴𓎼, include_str as 𓈖𓆑𓎡𓋴𓎼𓎼𓃋𓂧𓆑𓅓𓂋𓋴𓆑𓃎𓃌𓃋𓅱𓍯𓇋,
    is_x86_feature_detected as 𓈖𓆑𓊃𓆑𓃎𓃎𓃀𓇋𓎼𓊃𓏤𓅱𓅓𓊃𓃭𓃀𓍯𓂋𓃊𓎡𓋴𓇋𓊃𓎡𓃏𓅓𓂋𓋴𓎡𓋴𓇋𓊃𓃭𓂧𓍯𓂋𓋴𓅱𓇋, line as 𓈖𓂋𓅱𓅱𓃌𓊃𓇋,
    matches as 𓈖𓆑𓏘𓎡𓋴𓇋𓇋𓃋𓇋𓅓𓆑𓊃𓏘, module_path as 𓈖𓆑𓎡𓋴𓅱𓇋𓃍𓃭𓅓𓅓𓆑𓏤𓎡𓋴𓄿𓇋𓃭𓅱𓈖𓄿, option_env as 𓈖𓃍𓇋𓎛𓇋𓃊𓃭𓏤𓈖𓊃𓏤𓅱𓎡𓃋𓏏𓅱,
    panic as 𓍯𓃀𓏘𓅱𓃌𓃊𓃭𓂧, print as 𓍯𓃀𓊃𓎼𓋴𓃋𓏏𓅱, println as 𓍯𓃀𓊃𓎼𓋴𓃋𓏏𓅱𓈖𓂋𓎡𓋴𓄿, stringify as 𓍯𓈖𓃊𓎛𓇋𓃊𓃭𓍯𓅓𓃍𓅱𓅱𓅓𓃎𓇋,
    thread_local as 𓍯𓂋𓅱𓎛𓇋𓊃𓃭𓃀𓅓𓂋𓏤𓅱𓇋𓃋𓃋𓂧𓅓𓆑𓅱𓄿, todo as 𓍯𓂋𓎡𓋴𓅱𓇋𓃋𓇋,
    unimplemented as 𓍯𓆑𓎡𓋴𓎼𓋴𓃋𓃭𓏘𓈖𓂋𓋴𓅱𓃊𓊃𓃭𓍯𓍯𓂋𓋴𓅱𓇋, unreachable as 𓍯𓆑𓎡𓋴𓎛𓇋𓊃𓃭𓃀𓅓𓈖𓅱𓎼𓎡𓇋𓏏𓅓𓅓𓅱, vec as 𓍯𓊃𓋴𓅱𓎼,
    write as 𓍯𓃍𓊃𓎼𓋴𓃍𓂧𓆑, writeln as 𓍯𓃍𓊃𓎼𓋴𓃍𓂧𓆑𓈖𓂋𓎡𓋴𓄿,
};
#[allow(unused_imports)]
use 𓍯𓃀𓊃𓎼𓎡𓃋𓂧𓆑𓅓𓂋𓋴𓏘::*;
fn main() {
    𓍯𓃀𓊃𓎼𓋴𓃋𓏏𓅱𓈖𓂋𓎡𓋴𓄿!("Hello, world!");
}
```

[play.rust-lang.org](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3336ba2aa991fd46fe82881f975a160a)

## usage

```
$ # from file
$ historical file.rs
...
$
$ # from stdin
$ cat file.rs|historical
...
$
$ # format
$ historical file.rs|rustfmt
...
$
```
