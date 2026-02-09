# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Rust-based curl wrapper using the `curl` crate for HTTP requests with GSS-Negotiate (Kerberos/SPNEGO) authentication.

## Build & Test Commands

- **Build:** `cargo build`
- **Check (fast compile check):** `cargo check`
- **Run:** `cargo run`
- **Run all tests:** `cargo test`
- **Run ignored tests too:** `cargo test -- --include-ignored`
- **Run single test:** `cargo test <test_name>`

## Architecture

- `src/main.rs` — binary entrypoint (thin CLI wrapper)
- `src/curl/mod.rs` — public API re-exports
- `src/curl/config.rs` — Method enum, RequestConfig struct, builder methods
- `src/curl/args.rs` — CLI argument parsing, usage text, credential parsing
- `src/curl/response.rs` — Response struct, Timing struct, Display impls
- `src/curl/request.rs` — perform_request, credential/proxy/noproxy resolution
- `src/curl/error.rs` — RequestError enum (Curl, Io, Config variants)

## Notes

- Uses Rust edition 2024
- Do not run `git commit`
