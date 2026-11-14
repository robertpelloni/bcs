# scripts/

One entry point per job. Prefer `build-all.ps1` over calling the
generators by hand.

| Script | Job | Run |
|---|---|---|
| `build-all.ps1` | Build/check every component (ports, then C++ tools) at BelowNormal priority. `-CheckOnly` = fast layout/entry-point regression check, no compilation. | `powershell -File scripts\build-all.ps1 [-CheckOnly]` |
| `check_btk_naming.py` | Audit BTK naming conventions across the tree | `python scripts\check_btk_naming.py` |
| `generate_btk_api_inventory.py` | Generate the BTK public-API inventory | `python scripts\generate_btk_api_inventory.py` |
| `generate_bml_script_manifest.py` | Generate the BML script-API manifest | `python scripts\generate_bml_script_manifest.py` |
| `generate_bml_qtscript_dependency_audit.py` | Audit BML ↔ QtScript dependencies | `python scripts\generate_bml_qtscript_dependency_audit.py` |
| `generate_csscript_jscore_drift_audit.py` | Audit CsScript ↔ JavaScriptCore drift | `python scripts\generate_csscript_jscore_drift_audit.py` |
| `stage_probe_core_artifacts.ps1` | Copy validated `CsCore` artifacts from `build-vs2019` into the script probe tree | `powershell -File scripts\stage_probe_core_artifacts.ps1` |

Generated audit outputs live under `docs/ai/` (dated design/implementation
notes), not here. Do not add new top-level build scripts — extend
`build-all.ps1` instead, so there stays one build entry point.
