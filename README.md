# TaskTape

A lightweight recorder for Tokio runtime state.

TaskTape connects to `console-subscriber` and captures runtime updates about tasks, resources, async operations, and polling activity.

Recorded sessions can later be inspected and replayed, making it easier to understand how an asynchronous Rust application behaved over time.

## Goals

* Capture Tokio Console runtime updates
* Store recorded execution sessions
* Reconstruct runtime state over time
* Inspect relationships between tasks, resources, and async operations
* Provide a foundation for debugging async Rust applications

## Status

TaskTape is an early work in progress and a learning project toward building **Gordian** — a time-traveling debugger for async Rust.

> The current focus is runtime data collection and state reconstruction.
