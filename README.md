# stm32f0-rust-blinky

> This repository contains the source code to accompany ["Embedded Rust: From Zero to Blinky"](https://beta7.io/rust/embedded-rust-from-zero-to-blinky.html).  
> For a more in-depth explanation please refer to the post.

A minimal application to demonstrate embedded development using [Rust](https://github.com/stm32-rs/stm32f0xx-hal). Created using an [STM32 Nucleo F091RC](https://www.st.com/en/evaluation-tools/nucleo-f091rc.html) development board. Blinks an LED on pin `PA1`.

- - -

Made possible by the following amazing crates:

[cortex-m](https://github.com/rust-embedded/cortex-m) | [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt) | [panic-halt](https://github.com/korken89/panic-halt) | [stm32f0xx-hal](https://github.com/stm32-rs/stm32f0xx-hal)

- - -

## Quickstart

```bash
$ git clone https://github.com/jessebraham/stm32f0-rust-blinky.git
$ cd stm32f0-rust-blinky/
$ cargo build --release
$ ./flash_device.sh target/thumbv6m-none-eabi/release/stm32f0-rust-blinky
```
