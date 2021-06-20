# historify

convert rust source code's identify to Historical characters.

Currently work in progress.

## Example

input

```rust
fn main() {
    println!("Hello, world!");
}
```

output

```rust
#![allow(uncommon_codepoints)]
mod ᚼᚧᚩᛀᚻᚧᚵᛄᚹᚰᛝᛝ {
    #![allow(unused_imports)]
    use std::borrow::ToOwned as ᚵᚦᛚᚯᚽᛓᛖᛀᚹᚠᛝᛝ;
    use std::boxed::Box as ᚰᛀᛚᛕ;
    use std::clone::Clone as ᚰᛓᛎᛌᚻᛀᚴᛝ;
    use std::cmp::{
        Eq as ᚱᚷᚤᛝ, Ord as ᚳᛔᚩᛄ, PartialEq as ᚴᚦᚥᛏᚽᚦᛀᛁᚻᚤᚵᛎ, PartialOrd as ᚴᚦᚥᛏᚽᚦᛀᛁᚻᚤᛚᛏᚹᚠᛝᛝ,
    };
    use std::convert::{
        AsMut as ᚰᚷᚭᚭᚽᚷᚰᛝ, AsRef as ᚰᚷᚭᚲᚹᚶᚸᛝ, From as ᚱᛀᚩᛌᚻᚰᛝᛝ, Into as ᚲᚶᛖᛑᚻᛍᛝᛝ
    };
    use std::default::Default as ᚱᚦᚵᛀᚸᚷᚵᛉᚽᚠᛝᛝ;
    use std::iter::{
        DoubleEndedIterator as ᚱᚦᛚᛒᚸᛀᛎᛀᚱᚶᛖᛄᚹᚶᚱᚩᚽᚦᚵᛏᚸᚷᚱᛌᚼᛀᛝᛝ,
        ExactSizeIterator as ᚱᚷᛁᛁᚸᛔᚱᚳᚺᚷᛆᛀᚲᚷᚱᛀᚼᛀᚥᛑᚻᛔᚨᛝ, Extend as ᚱᚷᛁᛑᚹᚶᛖᛄ,
        IntoIterator as ᚲᚶᛖᛑᚻᛑᛀᛑᚹᚷᚩᛁᚽᚦᛚᛏ, Iterator as ᚲᚷᚱᛀᚼᛀᚥᛑᚻᛔᚨᛝ,
    };
    use std::marker::{
        Copy as ᚰᛓᛚᛍᚾᚰᛝᛝ, Send as ᚴᛓᚵᛋᚹᚠᛝᛝ, Sized as ᚴᛓᛀᛗᚹᚶᚰᛝ, Sync as ᚴᛔᛀᛋᚸᛍᛝᛝ, Unpin as ᚵᚶᛖᛍᚺᚶᛕᛝ,
    };
    use std::mem::drop as ᚹᚧᚩᛌᚼᚠᛝᛝ;
    use std::ops::{
        Drop as ᚱᚧᚩᛌᚼᚠᛝᛝ, Fn as ᚱᛀᛕᛝ, FnMut as ᚱᛀᛖᚭᚽᚷᚰᛝ, FnOnce as ᚱᛀᛖᚯᚻᛀᚭᛀ
    };
    use std::option::Option as ᚳᛔᚡᛑᚺᚶᛚᛋ;
    use std::option::Option::{None as ᚳᛀᛚᛋᚹᚰᛝᛝ, Some as ᚴᛓᛚᛊᚹᚰᛝᛝ};
    use std::result::Result as ᚴᛀᚵᛐᚽᚶᛎᛑ;
    use std::result::Result::{Err as ᚱᚷᚩᛏ, Ok as ᚳᛓᛉᛝ};
    use std::string::{
        String as ᚴᛔᚱᛏᚺᚶᛖᛀ, ToString as ᚵᚦᛚᚳᚽᚧᚩᛆᚻᛀᚼᛝ
    };
    use std::vec::Vec as ᚵᛀᚵᛃ;
}
#[allow(unused_imports)]
use std::{
    assert as ᚸᚷᚭᛐᚹᚷᚩᛑ, assert_eq as ᚸᚷᚭᛐᚹᚷᚩᛑᚷᛓᚵᛎ, assert_ne as ᚸᚷᚭᛐᚹᚷᚩᛑᚷᛓᛖᛀ, cfg as ᚸᛓᚹᛀ,
    column as ᚸᛓᛚᛉᚽᚶᛒᛋ, compile_error as ᚸᛓᛚᛊᚼᚦᛀᛉᚹᚵᛚᛀᚼᛀᚩᛌᚼᛀᛝᛝ, concat as ᚸᛓᛚᛋᚸᛓᚥᛑ, dbg as ᚹᚦᚩᛀ,
    debug_assert as ᚹᚦᚵᛂᚽᚶᚽᚿᚸᚷᚭᛐᚹᚷᚩᛑ, debug_assert_eq as ᚹᚦᚵᛂᚽᚶᚽᚿᚸᚷᚭᛐᚹᚷᚩᛑᚷᛓᚵᛎ,
    debug_assert_ne as ᚹᚦᚵᛂᚽᚶᚽᚿᚸᚷᚭᛐᚹᚷᚩᛑᚷᛓᛖᛀ, env as ᚹᚶᛖᛓ, eprint as ᚹᚷᚡᛏᚺᚶᛖᛑ,
    eprintln as ᚹᚷᚡᛏᚺᚶᛖᛑᚻᚦᛕᛝ, file as ᚹᛀᛀᛉᚹᚰᛝᛝ, format as ᚹᛀᛚᛏᚻᚶᚥᛑ,
    format_args as ᚹᛀᛚᛏᚻᚶᚥᛑᚷᛓᚥᛏᚹᛔᚬᛝ, include as ᚺᚶᛖᛃᚻᚧᚵᛄᚹᚰᛝᛝ,
    include_bytes as ᚺᚶᛖᛃᚻᚧᚵᛄᚹᚵᛚᛂᚾᚷᚱᛀᚼᛍᛝᛝ, include_str as ᚺᚶᛖᛃᚻᚧᚵᛄᚹᚵᛚᛐᚽᚧᚨᛝ,
    is_x86_feature_detected as ᚺᚷᚭᚿᚾᚣᛀᛓᚷᛓᚹᛀᚸᚷᚱᛒᚼᛀᚵᚿᚹᚦᚵᛑᚹᚶᚭᛑᚹᚶᚰᛝ, line as ᚻᚦᛀᛋᚹᚰᛝᛝ,
    matches as ᚻᚶᚥᛑᚸᛓᛁᛀᚼᛍᛝᛝ, module_path as ᚻᚶᛚᛄᚽᚶᛎᛀᚷᛔᚡᛁᚽᚦᛀᛝ, option_env as ᚻᛔᚡᛑᚺᚶᛚᛋᚷᛓᚵᛋᚽᛀᛝᛝ,
    panic as ᚼᚦᚥᛋᚺᚶᚬᛝ, print as ᚼᚧᚩᛆᚻᛀᚰᛝ, println as ᚼᚧᚩᛆᚻᛀᚱᛉᚻᛀᛝᛝ, stringify as ᚼᛔᚱᛏᚺᚶᛖᛀᚺᚶᚹᛖ,
    thread_local as ᚽᚦᛁᛏᚹᚶᚥᛄᚷᛓᛎᛌᚸᛓᚥᛉ, todo as ᚽᚦᛚᛄᚻᛍᛝᛝ, unimplemented as ᚽᚶᛖᛆᚻᚷᚡᛉᚹᚶᛒᛀᚻᛀᚱᛀᚹᚠᛝᛝ,
    unreachable as ᚽᚶᛖᛏᚹᚶᚥᛃᚺᚦᚥᛂᚻᚦᚴᛝ, vec as ᚽᛀᚵᛃ, write as ᚽᛔᚩᛆᚽᚦᚴᛝ, writeln as ᚽᛔᚩᛆᚽᚦᚵᛉᚻᛀᛝᛝ,
};
#[allow(unused_imports)]
use ᚼᚧᚩᛀᚻᚧᚵᛄᚹᚰᛝᛝ::*;
fn main() {
    ᚼᚧᚩᛆᚻᛀᚱᛉᚻᛀᛝᛝ!("Hello, world!");
}
```

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
