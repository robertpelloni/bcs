# TODO — granular short-term tasks

> Long-term structure: [ROADMAP.md](ROADMAP.md). Vision: [VISION.md](VISION.md).
> Check items off here; move finished structural work into the roadmap.

## Broken right now (fix first)

- [x] `MEMORY.md` contained committed merge-conflict markers
      (`<<<<<<< HEAD` / `>>>>>>> origin/master`) — resolved by union, 2026-11-13
- [x] Root `start.bat` ran `go run .` at the repo root, where no Go main
      package exists — repointed at `ports/go`, 2026-11-13
- [ ] Submodule checkouts differ from the recorded pins
      (`git submodule status` shows `+` on all three). Decide per submodule:
      commit the new pin or `git submodule update --checkout`. Do not guess.
- [ ] `build-vs2019-script-probe*` (8 probe trees) clutter the repo root.
      They are gitignored (`/build-*/`) but huge on disk. Archive or delete
      only with the owner's word — CMake caches inside hold absolute paths,
      so moving them breaks them anyway.

## Build & test

- [ ] Verify the full C++ build on `master` after the rebrand
      (configure `build-vs2019`, build `Release`, below-normal priority)
- [x] Build with `-DBUILD_TESTS=ON` and run Catch2 core: **1081/1082 pass**
      (ctest needs `build\bin\Release` on PATH — see CHANGELOG 0.3.28)
- [x] Run Catch2 `test/gui` + `test/network` — Gui 15/15, Network 41/42
      (Debug config; CsGui needs the amd64 MSBuild, 32-bit node OOMs)
- [ ] `QHostInfo basic_methods` pins copperspice.com to IP 69.89.11.103;
      the domain is now behind Cloudflare (2 anycast IPs) — upstream test
      should assert lookup success/shape, not pinned DNS answers
- [ ] `QDateTime std_chrono` (test/core/datetime/qdatetime.cpp:467): two
      stacked defects, analyzed 2026-11-14 — do not "fix" by tweaking one side.
      (a) Product: `QDateTime::fromStdLocalTime` (src/core/datetime/qdatetime.h:362)
      builds `QDateTime(1970-01-01, 00:00)` (local spec) and `addMSecs(count)`,
      so the instant is resolved with the system zone's *1970* offset, not the
      offset at the target wall time. Qt/std semantics: interpret the value as
      wall time in the system local zone, DST-aware at that date (construct
      `QDateTime(wallDate, wallTime, localZone)`). Header-inline, so the fix
      recompiles most of CsCore — schedule it on its own, not mid-other-builds.
      (b) Test: the constant `1698763505` is wall 2023-10-31 14:45:05, i.e. it
      converts the America/Los_Angeles dt (15:45:05 PDT, UTC-7) using the
      standard offset UTC-8 — one hour off. Even a correct (a) cannot make
      this pass as written on any machine; the test needs its constant
      recomputed (1698767105) and a Pacific system zone, or restructuring
      to avoid depending on the OS zone at all.
- [ ] Poll/finish the `CsScript` linked build in `build-vs2019-script-probe5`
      (artifacts already staged by `scripts/stage_probe_core_artifacts.ps1`)
- [ ] `ports/go`: `go build ./...` + `go vet ./...`
- [ ] `ports/rust`: `cargo build` + `cargo test`
- [x] `ports/csharp`: add a `.csproj` so `dotnet build` works — done 2026-11-13
      (`Bcs.csproj`; builds 0 warnings/0 errors and runs). Required fixing
      committed conflict markers, `BCS.*`/`Bcs.*` namespace split, duplicate
      stub classes, and a missing `EventType`/`BcsTimerEvent`/`BcsChildEvent`.
- [x] `ports/java`: document/add the one build command — done 2026-11-13
      (`javac -d out` over `Main.java` + `src/**`, wired into
      `scripts/build-all.ps1`; compiles and runs). Required unifying
      `core.*`/`bcs.core.*` packages and adding `EventType`.

## Docs & dashboard

- [x] Create root `VISION.md`, `ROADMAP.md`, `TODO.md`, `DEPLOY.md`, `IDEAS.md`
- [ ] Keep `CHANGELOG.md` in step with [VERSION](VERSION) on every bump
- [ ] Regenerate the single-page dashboard after each build
      (`docs/dashboard.html`, see `docs/DASHBOARD_UI_AUDIT.md`)
- [ ] User manual: mirror the dashboard's module map into help/manual text

## UI representation (per VISION rule 1)

- [ ] Audit C++ widgets for missing `setToolTip` / `setWhatsThis`
- [x] Confirm `Label`/`Description`/`Tooltip` exist on ported widgets —
      verified 2026-11-14: all four `BcsWidget` baselines carry them
      (go `bcswidget.go`, rust `bcswidget.rs`, csharp `BcsWidget.cs`,
      java `BcsWidget.java`) and every widget inherits the baseline

## Cleanup (nondestructive)

- [ ] Scripts inventory + entry point: `scripts/README.md`, `scripts/build-all.ps1`
- [ ] `.hypernexus/` is untracked — decide: commit the non-sensitive config
      or gitignore it explicitly (currently neither)
- [ ] `SUBMODULE_INVENTORY.md` is a 79-byte stub — fill it from
      `git submodule status` or delete it and point at `.gitmodules`
