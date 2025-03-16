# Using the `vita49` Crate from a C++ Application
<!--
SPDX-FileCopyrightText: 2025 The vita49-rs Authors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

Rust is cool and all, but there are a ton of signal processing applications
written in C++ that would be difficult to write ports or bindings for.

Instead, let's show how we can use the [cxx](https://cxx.rs/index.html)
crate to use the `vita49` crate from a C++ application.

## Defining the Foreign Function Interface (FFI)

Using this Rust crate from C++ isn't as simple as just importing a header.
You do need to define an FFI boundary. But, with CXX, that process isn't
too painful.

In our example, we have our FFI binding defined in [`src/main.rs`](src/main.rs):

```rust
#[cxx::bridge]
mod ffi {
    pub struct MySignalData {
        stream_id: u32,
        signal_data: Vec<u8>,
    }

    extern "Rust" {
        fn parse_vita49(packet_data: &[u8]) -> MySignalData;
    }
}
```

The struct `MySignalData` is going to be shared between C++ and Rust. The
language features used by the `Vrt` structure make it impossible to share
the whole struct, but you can write Rust code to extract the things you
need from the crate and pass it to C++ for downstream processing.

So, for the sake of this example, let's assume we just want to grab incoming
data samples and the stream ID out of incoming signal data packets.

Then, we present an external function, `parse_vita49()` for our C++ app to
hook into. The app will pass in a raw data slice and get a `MySignalData`
struct in return.

Let's take a look at the implementation for `parse_vita49()` (also in
[`src/main.rs`](src/main.rs)):

```rust
pub fn parse_vita49(packet_data: &[u8]) -> MySignalData {
    match Vrt::try_from(packet_data) {
        Ok(packet) => match packet.header.packet_type() {
            PacketType::SignalData => {
                println!(
                    "[RUST] Parsed signal data packet with a {} byte payload",
                    &packet.payload.signal_data().unwrap().payload_size_bytes()
                );
                MySignalData {
                    stream_id: packet.stream_id.unwrap(),
                    signal_data: packet.payload.signal_data().unwrap().payload().clone(),
                }
            }
            // Other packet types are not covered in this example
            _ => unimplemented!(),
        },
        Err(e) => panic!("Failed to parse: {}", e),
    }
}
```

The logic to parse the packet and load up the values in `MySignalData`
is defined here. This lets you abstract all the complex parsing logic
from your C++ and just give it what it needs.

## The C++ Side

In your C++ app, `src/main.cc`, you can see how it's used:

```c++
#include "cxx_demo/include/my_vrt.h"
#include "cxx_demo/src/main.rs.h"
#include <functional>
#include <string>
#include <vector>
#include <iostream>
#include <fstream>

int main(int argc, char** argv) {
    if (argc != 2) {
        std::cerr << "error - please pass a raw VRT file" << std::endl;
        exit(1);
    }
    std::ifstream file(argv[1], std::ios::binary);
    std::vector<uint8_t> input(std::istreambuf_iterator<char>(file), {});
    rust::Slice<const uint8_t> slice{input.data(), input.size()};
    MySignalData ret = parse_vita49(slice);
    printf("[C++] Got data packet with stream ID: 0x%X\n", ret.stream_id);
}
```

This example reads a VRT packet from a file (but it could be from anywhere),
then constructs a `rust::Slice` out of it. This is the CXX container type for
passing slices of data to Rust.

From there, we can run our `parse_vita49()` function and get our structure
back.

## Try it out

This demo requires a raw VRT file to run (passed as an argument). If you
don't have one handy, you can generate one from the JSON test files in this
repo. From the top level:

```bash
cargo run --features=serde --example json2vrt vita49/tests/spectral_data_packet.json5
```

With your file, in this subdirectory, try running `cargo run <your file>`.
You should get something like this:

```text
% cargo run ../vita49/tests/spectral_data_packet.vrt
   Compiling cxx_demo v0.0.2 (vita49/cxx_demo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `vita49/target/debug/cxx_demo ../vita49/tests/spectral_data_packet.vrt`
[RUST] Parsed signal data packet with a 1280 byte payload
[C++] Got data packet with stream ID: 0x1
```
