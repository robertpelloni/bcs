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
- [ ] Run Catch2 `test/gui` + `test/network` (build in progress in `build/`)
- [ ] `QDateTime std_chrono` fails on non-Pacific Windows zones
      (fromStdLocalTime DST handling) — investigate or run on a Pacific machine
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
- [ ] Confirm `Label`/`Description`/`Tooltip` exist on every ported widget
      (Go, Rust, C#, Java), not just the `BcsWidget` baseline

## Cleanup (nondestructive)

- [ ] Scripts inventory + entry point: `scripts/README.md`, `scripts/build-all.ps1`
- [ ] `.hypernexus/` is untracked — decide: commit the non-sensitive config
      or gitignore it explicitly (currently neither)
- [ ] `SUBMODULE_INVENTORY.md` is a 79-byte stub — fill it from
      `git submodule status` or delete it and point at `.gitmodules`
