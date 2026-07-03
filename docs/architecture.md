# Furia Plugin Example — Architecture

## Overview

A complete, production-ready example of a Furia C2 plugin.
Demonstrates how to implement SDK traits and register them via FuriaBuilder.

## Architecture

```
┌──────────────────────────────────────────┐
│         Plugin Binary                      │
│  ┌────────────────────────────────────┐   │
│  │         src/main.rs                  │   │
│  │  ┌─────────────┐                    │   │
│  │  │ FuriaBuilder │──► providers      │   │
│  │  └─────────────┘     ┌───────────┐  │   │
│  │                      │ SimpleDrone│  │   │
│  │                      │ (Simulation│  │   │
│  │                      │  Provider) │  │   │
│  │                      └───────────┘  │   │
│  └────────────────────────────────────┘   │
│                                           │
│  Integrates with:                         │
│  * furia-control (via path/git dep)       │
│  * interop-gateway (via REST)             │
│  * @furia/ui (via NPM)                    │
└──────────────────────────────────────────┘
```

## Running

```bash
cargo build
cargo run
```