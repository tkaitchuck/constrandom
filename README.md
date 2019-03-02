# Random constants
This crate provides compile time random number generation.
This allows you to insert random constants into your code that will be auto-generated at compile time.

A new value will be generated every time the file is rebuilt.
This obviously makes the resulting binary or lib non-deterministic.

# Example 

```
#![feature(proc_macro_hygiene)]
use const_random::const_random  ;
const MY_RANDOM_NUMBER: u32 = const_random!(u32);
```
Note that the `#![feature(proc_macro_hygiene)]` must be enabled on your crate for the macro to work.

The following types are supported u8, u16, u32, u64, and u128