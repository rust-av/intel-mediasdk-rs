# Intel Media SDK bindings

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

It is a simple [binding][1] and safe abstraction over the [Intel Media SDK][2].

## Building

The bindings are generated using the headers and libraries that ought to be present in the system.
The original library [repackaging][3] supports pkg-config thus alternate paths can be expressed
using the standard pkg-config variables.

## TODO
- [ ] Simple bindings
- [ ] Safe abstraction
- [ ] Examples

[1]: https://github.com/servo/rust-bindgen
[2]: https://software.intel.com/en-us/media-sdk
[3]: https://github.com/lu-zero/mfx_dispatch
