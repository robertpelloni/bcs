# IDEAS — aggressive options, unfiltered

> Nothing here is committed work. Committed work is [ROADMAP.md](ROADMAP.md)
> and [TODO.md](TODO.md). Steal from this list deliberately.

## Pivots

- **Multi-user OS shell on BTK.** The input arbitrator is the one feature
  no rival framework has. Push it from "widget focus rule" to a session
  layer: multiple pointers, keyboards, and remote users on one desktop,
  each with identity, permissions, and audit trail.
- **BTK as the bqt dashboard's native shell.** Replace the web dashboard
  in the `bqt` submodule (OmniDash) with a native BTK app rendering the
  same consolidated page — dogfooding as the parity test.
- **BML as the single UI language.** Make every demo, tool (`linguist`,
  `uic` previews), and the dashboard itself a BML document, so the
  declarative runtime is exercised by everything the project ships.

## Re-architecture

- **Kernel extraction.** Split `CsCore`'s object/event kernel into a
  small, dependency-free `btk-kernel` library that Go/Rust/C#/Java ports
  target as their semantic spec, with conformance fixtures generated
  from the C++ headers.
- **Header-driven ports.** Generate the ports' type skeletons
  (`BcsString`, `BcsVariant`, geometry, containers) from the C++ headers
  with the existing `scripts/generate_btk_api_inventory.py` pipeline,
  so ports stop drifting from the C++ API by construction.
- **Plugin-first tools.** Turn `uic`/`rcc`/`lupdate` into loadable
  pipeline plugins so IDEs can embed them instead of shelling out.

## Language ports

- **Rust as the second reference implementation**, not a port: unsafe-free
  core, `cargo test` conformance suite shared with C++ via fixtures.
- **Go port as the server side**: BTK semantics for a headless
  multi-user session server (the arbitrator over WebSocket/WebRTC),
  with the C++ GUI as one client among many.
- **WASM target** for BML apps: compile the declarative runtime to
  WebAssembly so BML demos run in the browser dashboard unchanged.

## Feature expansions

- **System-tray operations app.** A small BTK tray app (framework support
  already exists: `src/gui/util/qsystemtrayicon.*`) showing build/test
  status, with Start/Stop for the port servers and Quit — the missing
  operational surface this repo currently lacks.
- **Live API explorer.** Generate an interactive widget gallery from the
  API inventory: every widget, its `Label`/`Description`/`Tooltip`,
  live-editable BML, on one page.
- **Parity radar.** Auto-diff BTK vs JUCE vs Ultimate++ public APIs on
  every submodule bump and post the delta to the dashboard.

## Wild

- Port the kernel to a microcontroller profile (no-heap object model)
  for embedded multi-user panels — industrial HMIs are multi-user
  already and nobody serves them well.
- Record/replay input-arbitration sessions as tests: capture real
  multi-device input, replay deterministically in CI for all 5 languages.
