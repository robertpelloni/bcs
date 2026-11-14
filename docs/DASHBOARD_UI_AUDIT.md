# Dashboard & UI audit — BTK (this repo)

Date: 2026-11-13 · Project version: see [../VERSION](../VERSION)

## The honest headline

**This repo has no web dashboard and no server to quit.** BTK is a C++
application framework (plus Go/Rust/C#/Java ports). The multi-page web
dashboard the project brief describes — OmniDash, with subpages per
feature — lives in a different repo, kept here only as the reference
submodule `external/bqt-reference` (`OmniUI/apps/OmniDash`). Nothing in
this repo serves HTTP, so there is nothing to consolidate server-side
and nothing for a tray icon to stop.

What this repo *does* have as its user-facing surface:

1. The framework widgets themselves (`src/gui`), used by downstream apps.
2. Six GUI/CLI tools (`uic`, `rcc`, `lupdate`, `lrelease`, `lconvert`,
   `linguist`) — only `linguist` has a real GUI.
3. Four language ports whose widgets carry `Label` / `Description` /
   `Tooltip` by design (see `MEMORY.md`, "UI Foundation Compliance").

So the consolidation deliverable for *this* repo is a **single static
status page** — [dashboard.html](dashboard.html) — that puts every
module, tool, port, test suite, and workflow on one page, grouped by
value, with tooltips. No subpages, no server, opens from disk.

## Page-by-page findings (old fragmented state → consolidated)

| Fragmented source | What it held | Now on the one page |
|---|---|---|
| `README.md` module table | 12 C++ modules | § Framework modules, in value order |
| `HANDOFF.md` (58 KB, stacked session logs) | Build state, frontiers | § Status & how to run (distilled; HANDOFF stays the log) |
| `MEMORY.md` port phases 11–21 | Port coverage per language | § Language ports matrix |
| `docs/ai/**` (100+ dated notes) | Design/plan/test evidence | Linked, not copied; gaps in § Known gaps |
| `scripts/*.py` (5 generators) | API/BML/Script audits | § Workflows, one row each |
| `build-vs2019*/` (9 trees) | Prebuilt tools + stale caches | § Status: which tree is usable and why |

## Reorganization (value order)

1. **Status & how to run** — the three commands anyone needs first.
2. **Framework modules** — Core/Gui first (highest value), then the rest.
3. **Tools** — the six shipped binaries.
4. **Language ports** — Go/Rust/C#/Java with build state.
5. **Tests** — Catch2 suites and how to enable them.
6. **Workflows & docs** — scripts and the doc set.

## System tray: where is it?

- Framework support exists: `src/gui/util/qsystemtrayicon.*` (+ platform
  backend `src/gui/platform/qplatform_systemtrayicon.*`).
- **No application in this repo creates a tray icon**, because no
  long-running app or server ships here. There is therefore no
  "Quit servers" menu — there are no servers.
- The tray operations app (status + quit) is a genuinely good idea and
  is recorded as a feature candidate in [../IDEAS.md](../IDEAS.md);
  in the `bqt` product it belongs to OmniDash, not to this framework repo.

## UI quality rules applied to the dashboard

- Every card has a label, a one-line description, and a `ⓘ` tooltip
  (`title` text) for anything needing explanation.
- Related concepts grouped (modules / tools / ports / tests), parallel
  concepts shown as matrices instead of separate pages.
- No page left out: every `src/*` module, every `src/tools/*` tool,
  every `ports/*` language, every test suite, and every script in
  `scripts/` appears exactly once.
