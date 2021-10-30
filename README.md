# `cortex-m0-quickstart`


This project has been generated using the [cortex-mo-quickstart](https://github.com/rust-embedded/cortex-m-quickstart).


<span style="color:green">**What follows is explicit for the Adafruit Feather M0 and this project.**</span> 


## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`  

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation). 

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

0. Before we begin you need to identify some characteristics of the target
  device as these will be used to configure the project:

- The ARM core. e.g. Cortex-M3.

- Does the ARM core include an FPU? Cortex-M4**F** and Cortex-M7**F** cores do.

- How much Flash memory and RAM does the target device has? e.g. 256 KiB of
  Flash and 32 KiB of RAM.

- Where are Flash memory and RAM mapped in the address space? e.g. RAM is
  commonly located at address `0x2000_0000`.

You can find this information in the data sheet or the reference manual of your
device.

**In this project we are using the Adafruit Feather M0 (and older version).** 

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: blink-cortex-m0
 
$ cd app
```

2. Set a default compilation target. There are four options as mentioned at the
   bottom of `.cargo/config`. For the Adafruit Feather M0, which has a Cortex-M0
   core, we'll pick the `thumbv6m-none-eabi` target.

``` console
$ Edit .cargo/config
```


``` toml
[build]
# Pick ONE of these compilation targets
target = "thumbv6m-none-eabi"    # CortexM0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```
3. Edit the dependencies in `.\Cargo.toml`

``` toml
[dependencies]
cortex-m = "0.6.0"
cortex-m-rt = "0.6.10"
cortex-m-semihosting = "0.3.3"
panic-halt = "0.2.0"
feather_m0 = "0.10.1"
```

> Orginally tried:  
>``` toml
>[dependencies]
>panic-halt = "0.2.0"
>feather_m0 = { git = "https://github.com/atsamd-rs/atsamd" }
>```
>
> This worked, but it seems some changes in the quickstart repo have caused a build error. Maybe this will work later.


3. Enter the memory region information into the `memory.x` file. This information comes from [another project](https://github.com/atsamd-rs/atsamd/blob/master/boards/feather_m0/memory.x).

``` console
MEMORY
{
  /* Leave 8k for the default bootloader on the Feather M0 */
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
```
4. Place the code in ```main.rs``` under ```./src```

5. Build the appplication.

``` console
$ cargo build
``` 


6. Copy the generated code over to a binary suitable for flashing.
``` console
$ objcopy -O binary .\target\thumbv6m-none-eabi\debug\blink-cortex-m0 .\target\thumbv6m-none-eabi\debug\blink-cortext-m0.bin
```

> 
> Also experimented with: 
> ``` console
>$  cargo objcopy -- -O binary  .\target\thumbv6m-none-eabi\debug\blink-context-m0.bin
>```
>    Need to first install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils).
>
> But had little success!
>  
>


7. Flash the microcontroller. Note that the old version of the Feather M0 being used does not have the HF2 bootloader so  have to use this the `bossac` command.
**Windows**:
``` console
$ bossac -p COM7 -e -w -v -R  .\target\thumbv6m-none-eabi\debug\blink-cortex-m0.bin
```
**Unix**: 

**<span style="color:red">For version 1.9!  Note the offset! Getting this wrong can brick the microcontroller.</span>**
``` console
$ bossac -p COM7 -e -w -v -R  --offset=0x2000 .\target\thumbv6m-none-eabi\debug\blink-cortex-m0.bin  
```
8. The red led and any led connected to pin D7 should blink.

# Debugging 
    
Do not think that I can use OCD for debugging as the feather board does not have the corrresponding debugger chip. Problem as most of the examples in the cortex-m-quickstart use this to output something.

Note that openOSD is supplied with the Ardunino IDE. See [this video](https://youtu.be/aC7VN_tFGfg).
    
# Useful Links

* [Arduino example with serial output](https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-usart.rs)

* [cargo-utils, for example cargo objcopy, documentation](https://docs.rs/crate/cargo-binutils/0.1.0)
