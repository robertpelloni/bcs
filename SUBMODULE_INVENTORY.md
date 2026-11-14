# Submodule inventory

Source of truth for URLs: `.gitmodules`. Live state: `git submodule status`.
(`+` = checkout differs from the commit recorded in this repo.)

| Path | URL | Checked out | Recorded pin state |
|---|---|---|---|
| `external/bqt-reference` | https://github.com/robertpelloni/bqt | `94501ec8` (heads/main) | differs (`+`) |
| `external/juce` | https://github.com/juce-framework/JUCE | `abd9515a` (6.0.8-4396) | differs (`+`) |
| `external/ultimatepp` | https://github.com/ultimatepp/ultimatepp | `b337b159` (v2026.1-93) | differs (`+`) |

All three are reference code for parity work (see `VISION.md`,
`docs/DASHBOARD_UI_AUDIT.md`); BTK itself builds without them.
Do not advance or reset a pin without a build — see `TODO.md`.
Last inventoried: 2026-11-13.
