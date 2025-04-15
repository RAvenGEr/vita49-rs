# NATS Control
<!--
SPDX-FileCopyrightText: 2025 The vita49-rs Authors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

In VITA 49.2, packet types that allow command and control (C2) of RF devices were
added to the standard. These include packets typically sent by "controller"
devices (e.g., software commanding an SDR) and packets returned from
"controllee" devices (e.g., acknowledgment packets returned by an SDR).

The breakdown looks like this:

```
                             ┌──────────────────┐                                          
                             │                  │                                          
                   ┌─────────┤  Command Packet  ├─────────┐                                
                   │         │      Types       │         │                                
                   │         │                  │         │                                
          ┌────────▼───────┐ └──────────────────┘  ┌──────▼──────┐                         
          │                │                       │             │                         
        ┌─┤ Control Packet ├──┐                  ┌─┤  ACK Packet ├──┬──────────────┐       
        │ │     Types      │  │                  │ │    Types    │  │              │       
        │ │                │  │                  │ │             │  │              │       
        │ └────────────────┘  │                  │ └─────────────┘  │              │       
        │                     │                  │                  │              │       
┌───────▼────────┐   ┌────────▼───────┐    ┌─────▼───────────┐ ┌────▼─────┐  ┌─────▼──────┐
│    Control     │   │  Cancellation  │    │  Validation ACK │ │ Exec ACK │  │ Query ACK  │
│     Packet     │   │     Packet     │    │      Packet     │ │  Packet  │  │   Packet   │
└────────────────┘   └────────────────┘    └─────────────────┘ └──────────┘  └────────────┘
```

The programs in this directory show how to use a few of these packet types to
handle a basic C2 flow.

## Controller

The [controller program](src/controller.rs) has the following logic:

1. Start up and parse CLI args.
2. Connect to the NATS server.
3. Construct a command packet based on CLI args.
4. Publish the command packet to the NATS command subject.
5. Retrieve ACK and check values.

## Controllee

The [controllee program](src/controllee.rs) has the following logic:

1. Start up and connect to the NATS server.
2. Subscribe to the NATS command subject.
3. Wait for a command to come in.
4. When a command comes in, validate and execute it.
    - Modify internal state variables for freq and bandwidth.
    - If an ACK is requested, send one.
5. Go to 3.

## Running

Note: for either program, you can set `RUST_LOG=debug` in your environment
to see more detailed output.

To run these programs, you'll need a NATS server running on `localhost:4222`.
This can be done easily with Docker:

```
docker run --rm -d --name nats --network=host nats:latest
```

Now, start up the controllee program:

```
cargo run --bin controllee
```

You should see it come up and connect to NATS, then await C2 messages.

To send a C2 message, use the controller program. For example, set the tune
frequency to 100 MHz:

```
cargo run --bin controller -- -f 100000000
```

You should see the controllee receive the command, then send an ACK back to the
controller.

This fake SDR has a supported tune frequency of 0 Hz to 6 GHz, so let's try
sending a frequency of 6.1 GHz:

```
cargo run --bin controller -- -f 6100000000
```

You should see an error reported back in the ACK.

You can also query the current state of the fake radio:

```
cargo run --bin controller -- -q
```
