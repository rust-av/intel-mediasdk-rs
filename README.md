# Intel Media SDK bindings

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Actions Status](https://github.com/rust-av/intel-mediasdk-rs/workflows/intel-mediasdk/badge.svg)](https://github.com/rust-av/intel-mediasdk-rs/actions)

It is a simple [binding][1] and safe abstraction over the [Intel Media SDK][2].

## Building

The bindings are generated using the headers and libraries that ought to be present in the system.
On Linux the [main repository](https://github.com/Intel-Media-SDK/MediaSDK) provides full instructions,
on Windows you may use the [repackaged dispatcher](https://github.com/lu-zero/mfx_dispatch).

## TODO
- [x] Simple bindings
- [ ] Safe abstraction
- [ ] Examples

[1]: https://github.com/rust-lang-nursery/rust-bindgen
[2]: http://mediasdk.intel.com
[3]: https://github.com/lu-zero/mfx_dispatch
[4]: https://github.com/Intel-Media-SDK/MediaSDK
