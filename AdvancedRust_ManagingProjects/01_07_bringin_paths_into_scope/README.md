# 01.07 to 01.08

In this package we will write the content of the videos:

## `1.7 Bringing paths into scope`

Sometimes paths are vey long and verbose. To simplify the paths in our code we can use the `use` statement to bring a path into the local scope. This is very useful when you need to call a function from a module multiple times. 

```rust 
// Instead of calling the fuction "english" like this
crate::greeting::formal::english();

// -------------------------------------------

// You could just bring it into scope with
use crate::greeting::formal;

// and call it with
formal::english();
```

When bringing multiple items from a module you can use the next syntax:
```rust
use crate::greeting::formal;
use crate::greeting::casual;

// Even better 
use crate::greeting::{formal, casual};
```

## `1.8 Using external crates`

To bring multiple items with the same name into scope you can use `as` to change th name of it.
```rust
use crate::greeting::formal::spanish as f_spanish;
use crate::greeting::casual::spanish as c_spanish;
```

to bring multiple items from a module you can use the glob operator `*`
```rust
use rand::prelude::*;
```