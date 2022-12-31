# 01.07

In this package we will write the content of the videos:

## `1.7 Bringing paths into scope` inline modules

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