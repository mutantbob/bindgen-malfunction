This project illustrates a malfunction in bindgen 0.59.2

When generating bindings for a header file,
and there is a derived class 
(`Alpha` in `avr-borken/src-cpp/alpha.h`)
whose base class is defined in a `blocklist_file` 
(`Base` in `avr-borken/src-cpp/base.h`), 
sometimes bindgen fails to generate the proper `bindgen_padding_`.

To illustrate the problem `cd avr-borken; cargo build --release` .

The correct output appears in `avr-borken/target/avr-atmega328p/release/build/avr-borken-*/out/alpha-minus.rs`
```
#[repr(C)]
pub struct Alpha__bindgen_vtable(cty::c_void);
#[repr(C)]
pub struct Alpha {
    pub vtable_: *const Alpha__bindgen_vtable,
    pub __bindgen_padding_0: u16,
    pub bar: cty::c_short,
    pub bacon: cty::c_short,
}
```

The incorrect output appears in `avr-borken/target/avr-atmega328p/release/build/avr-borken-*/out/alpha-wrong.rs`

```
#[repr(C)]
pub struct Alpha__bindgen_vtable(cty::c_void);
#[repr(C, packed)]
pub struct Alpha {
    pub vtable_: *const Alpha__bindgen_vtable,
    pub bar: cty::c_int,
}
```