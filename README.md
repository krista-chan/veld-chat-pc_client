# Veld.Chat PC Client

## Table of Contents

- ✨ [About](#about) ✨

## About

### The official PC client for [chat.veld.dev](https://chat.veld.dev)

This client was written in Rust with the [iced](https://crates.io/crates/iced) GUI lib and the [ws](https://crates.io/crates/ws) crate for connecting to the gateway (**_patent pending_**)  

## Drawbacks

As of now, the lib doesn't connect to the gateway because it can't interface via socket.io as there aren't the necessary Rust bindings
