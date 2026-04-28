# streamframe

**streamframe** is a lightweight engine for computing real-time features over streaming data.

It’s built in Rust for performance, but designed for ease of use via Python.

---

## Why this exists

Most tools like pandas or Polars assume you already have all your data.

But in many real-world systems, data arrives continuously:
- API requests
- logs
- sensor readings
- market data

You don’t want to recompute everything every time a new value comes in.

**streamframe updates everything incrementally, in constant time.**

---

## Installation

```bash
pip install streamframe



![demo](assets/demo.gif)