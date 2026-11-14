# ROADMAP — BTK long-term structural milestones

> Granular, immediately actionable work lives in [TODO.md](TODO.md).
> Vision and rationale live in [VISION.md](VISION.md).

## M1 — Trustworthy baseline (in progress)

- [x] CopperSpice → BTK rebrand (4,282+ files), `BTK::*` CMake aliases
- [x] Multi-language port skeletons: Go, Rust, C#, Java under `ports/`
- [ ] Rebrand validated by a clean full build + test run on `master`
      (last recorded state: compile green, `CsScript` link frontier staged)
- [ ] One-command build for every component (`scripts/build-all.ps1`)
- [ ] Catch2 suites (`test/core`, `test/gui`, `test/network`) run in CI
      and locally with a single command

## M2 — Script + declarative revival

- [x] `CsScript` (JavaScriptCore-derived engine) links and runs, not just
      compiles — done 2026-11-14 (v0.3.31); regression test `CsScriptSmoke`
- [ ] BML declarative runtime (`src/declarative`) builds against the revived engine
- [ ] Script API manifest + JSCore drift audits regenerated and checked in
      (`scripts/generate_bml_script_manifest.py`,
      `scripts/generate_csscript_jscore_drift_audit.py`)

## M3 — Multi-user GUI (the differentiator)

- [ ] `BcsInputArbitrator` parity verified across C++, Go, Rust, C#, Java
      (ownership, focus hotspot, per-user input routing)
- [ ] Multi-user focus foundation + developer overlay merged from design docs
      (`docs/ai/design/2026-04-04-multi-user-focus-foundation.md`,
      `docs/ai/design/2026-04-04-developer-overlay-groundwork.md`)
- [ ] Every widget exposes `Label` / `Description` / `Tooltip` in all ports

## M4 — Parity hardening

- [ ] Close the JUCE / Ultimate++ gap matrix
      (`docs/ai/design/2026-04-04-juce-ultimatepp-gap-matrix.md`)
- [ ] Feature-parity gap matrix burned down module by module
      (`docs/ai/design/2026-04-08-feature-parity-gap-matrix.md`)
- [ ] Public API reference generated from headers and published
      (`docs/ai/design/2026-04-08-btk-public-api-reference.md`)

## M5 — Ports graduate from skeletons to libraries

- [ ] Go port: real `Object`/`EventLoop`/`Widget` behavior + tests (`go test ./...`)
- [ ] Rust port: `cargo test` green, modules linked without tracking branches
- [ ] C# port: `dotnet build` green with a real project file
- [ ] Java port: buildable with a single documented command
- [ ] Cross-language conformance fixtures: same scenario, same result, 5 languages

## M6 — Release engineering

- [ ] Single version source: [VERSION](VERSION) drives docs/changelog;
      CMake ABI version (`2.1.0`) documented as a separate, deliberate number
- [ ] `DEPLOY.md` instructions verified from a clean checkout
- [ ] Tagged releases with built binaries kept (not cleaned) per handoff rules

## Branch & submodule policy

- `master` is the integration branch; `main` is kept caught up from it.
- AI-created feature branches are merged into `master`, then `main` is
  merged back into any surviving feature branch so it never goes stale.
- Submodules (`external/juce`, `external/ultimatepp`,
  `external/bqt-reference`) are pinned to fetchable upstream commits;
  never advance a pin without a build.
