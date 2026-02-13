# Data Extraction Architecture

How Factorio game data flows from the game installation into the blueprint editor at runtime.

**Read when:** working on game data updates, Space Age DLC support, adding new entity types, debugging missing sprites or data.

## Pipeline Overview

```
Factorio headless server (downloaded)
        |
        v
  Lua mod ("export-data") injected into data-loading phase
        |
        v
  data.json (serialized prototypes) + .png sprite references
        |
        v
  Rust post-processing: PNG -> Basis Universal textures
        |
        v
  packages/exporter/data/output/   (data.json + .basis sprites)
        |
        v
  Vite dev server (proxy) / production build (static copy)
        |
        v
  Browser: fetch('/data/data.json') -> FD global -> 20+ editor modules
```

## Packages

Monorepo with three packages (npm workspaces):

| Package | Path | Language | Role |
|---------|------|----------|------|
| `exporter` | `packages/exporter/` | Rust + Lua | Downloads Factorio, runs extraction mod, converts sprites |
| `@fbe/editor` | `packages/editor/` | TypeScript | Core editor library; consumes extracted data |
| `@fbe/website` | `packages/website/` | TypeScript | Web frontend; serves data via Vite |

Entry point: `npm run start:exporter` (runs `cargo run --release` inside `packages/exporter/`).

---

## Stage 1: Factorio Download

**File:** `packages/exporter/src/setup.rs` — `download_factorio()`

- Target version hardcoded in `packages/exporter/src/main.rs` (currently `2.0.68`)
- Downloads the headless server from `https://www.factorio.com/get-download/{version}/alpha/{os}`
- Auth via env vars `FACTORIO_USERNAME` and `FACTORIO_TOKEN`
- Extracts to `packages/exporter/data/factorio/`
- Version check: reads `data/base/info.json` to skip re-download if already correct

---

## Stage 2: Lua Data Extraction

The core trick: a **fake Factorio mod** that runs inside Factorio's data-loading pipeline to access `data.raw` (the full prototype table), then serializes everything out as JSON.

### The Mod: `export-data`

Three files in `packages/exporter/src/export-data/`:

#### `info.json`
Standard mod descriptor. Declares dependency on `base >= 2.0.0`, factorio_version `2.0`.

#### `data-final-fixes.lua`
Runs during Factorio's **data-final-fixes** stage (last stage where mods can read/modify prototype data — all base + mod prototypes are finalized). This is the heavy lifter.

**What it extracts from `data.raw`:**

| Key | Source | Description |
|-----|--------|-------------|
| `items` | 22 item prototype types (`item`, `ammo`, `capsule`, `gun`, `module`, `rail-planner`, `armor`, `space-platform-starter-pack`, etc.) | All items with localized names |
| `fluids` | `data.raw.fluid` | All fluid prototypes |
| `signals` | `data.raw['virtual-signal']` | All virtual signals |
| `recipes` | `data.raw.recipe` | All recipes (excludes "creative" entities) |
| `entities` | 77 entity prototype types (accumulators, beacons, combinators, assembling machines, rails, turrets, belts, etc.) | All blueprintable entities; filters out `not-blueprintable` and `breaths-air` flags |
| `tiles` | `data.raw.tile` | Minable tiles |
| `inventoryLayout` | Composite: items + recipes + fluids + signals organized by group/subgroup | Mirrors the in-game inventory tab structure |
| `utilitySprites` | `data.raw['utility-sprites'].default` | UI sprite definitions |
| `utilityConstants` | `data.raw['utility-constants'].default` | Game constants |
| `guiStyle` | `data.raw['gui-style'].default` | GUI styling data |
| `defines` | Factorio's `defines` table | Enum constants (inventory slots, directions, wire types, etc.) |

**Locale handling:** Before running Factorio, the Rust code (`setup.rs` lines 125-135) parses all `**/*/locale/en/*.cfg` files (INI-style) into a Lua table, written as `locale.lua`. The Lua script `require('locale')` loads this to resolve localized entity/item names.

**Serialization trick:** Factorio limits prototype string fields to 200 characters. The script:
1. Serializes the full output via `serpent.dump()`
2. Splits into 200-char chunks
3. Stores each chunk as a fake `simple-entity` prototype named `FBE-DATA-1` through `FBE-DATA-{N}`
4. Stores the count in `FBE-DATA-COUNT`

#### `control.lua`
Runs at **scenario runtime** (`on_init` event):
1. Reads chunk count from `prototypes.entity["FBE-DATA-COUNT"].localised_name`
2. Reassembles all chunks into the serialized Lua string
3. Deserializes via `load(serialized)()`
4. Writes as JSON: `helpers.write_file('data.json', helpers.table_to_json(data))`
5. Calls `error("!EXIT!")` to terminate Factorio

### Rust Orchestration

**File:** `packages/exporter/src/setup.rs` — `extract()`

1. Copies mod files into `data/factorio/mods/export-data/`
2. Generates `locale.lua` from `.cfg` files
3. Runs: `factorio --start-server-load-scenario export-data/export-data`
4. Reads `data/factorio/script-output/data.json`
5. Copies to `data/output/data.json`

---

## Stage 3: Sprite Processing

Still in `setup.rs` `extract()`:

1. Regex-scans `data.json` for all `.png` paths referenced by prototypes
2. For each image:
   - Resolves `__base__` / `__core__` prefixes to the Factorio data directory
   - Pads non-power-of-2 images to next power of 2
   - Converts PNG to **Basis Universal** (`.basis`) format via the `basisu` binary (committed at `packages/exporter/basisu`)
   - Uses `metadata.json` cache to skip unchanged files
3. Writes `.basis` files to `data/output/` preserving directory structure (e.g., `data/output/__base__/graphics/entity/...`)

Result: ~1,963 `.basis` sprite files alongside `data.json`.

---

## Stage 4: Serving to the Browser

**File:** `packages/website/vite.config.js`

| Mode | Mechanism |
|------|-----------|
| **Dev** | Rust exporter starts an HTTP server on `:8081` serving `data/output/`. Vite proxies `/data/*` to it. |
| **Prod** | `vite-plugin-static-copy` copies `../exporter/data/output/*` into the build's `data/` directory. |

---

## Stage 5: Editor Runtime Consumption

### Data Loading

**File:** `packages/editor/src/Editor.ts`

```typescript
await fetch('/data/data.json')
    .then(res => res.text())
    .then(modules => loadData(modules))
```

**File:** `packages/editor/src/core/factorioData.ts` — `loadData(str)`

1. Parses JSON
2. Populates the global `FD` object with all 11 data categories
3. Post-processes: separates `fast_replaceable_group` for splitters/underground belts
4. Exports `FD` as default export — the single source of truth for all game data

### Type Safety

Dev dependency `typed-factorio` (v3.31.0) provides TypeScript types for Factorio prototypes. `factorioData.ts` imports 100+ specific prototype types from `factorio:prototype` and `factorio:runtime`.

### Texture Loading

**File:** `packages/editor/src/common/globals.ts` — `getTexture(path, x, y, w, h)`

1. Rewrites `.png` paths to `.basis`: `/data/${path.replace('.png', '.basis')}`
2. Loads via pixi.js `Assets.load()` with a Basis Universal WebAssembly transcoder (`src/basis/transcoder.1.16.4.wasm`)
3. Supports sub-rectangle extraction from sprite sheets
4. Caches in a `Map<string, Texture>`

### Key Consumers of `FD`

~20 files import `FD`:

| Category | Files | What they use |
|----------|-------|---------------|
| **Blueprint parsing** | `Blueprint.ts`, `bpString.ts` | Entity/recipe/item/fluid name validation, schema keywords |
| **Entity logic** | `Entity.ts`, `PositionGrid.ts`, `Tile.ts`, `WireConnections.ts` | Entity properties, collision boxes, wire connection points |
| **Sprite rendering** | `spriteDataBuilder.ts` (2,117 LOC — largest file), `EntitySprite.ts` | Sprite layer selection per entity type/direction/state |
| **Containers** | `BlueprintContainer.ts`, `EntityContainer.ts`, `OverlayContainer.ts`, `PaintEntityContainer.ts`, `PaintTileContainer.ts`, `TileContainer.ts`, `UnderlayContainer.ts` | Rendering entity/tile graphics |
| **UI panels** | `InventoryDialog.ts`, `EntityInfoPanel.ts`, `Filters.ts`, `Preview.ts` | Inventory layout, entity details, filter options |
| **Auto-generators** | `pipe.ts`, `pole.ts`, `beacon.ts` | Entity placement rules |

### Blueprint Validation

`bpString.ts` registers custom AJV schema keywords (`entityName`, `itemName`, `fluidName`, `recipeName`, `tileName`, `itemFluidSignalRecipeEntityName`) that validate imported blueprint strings against `FD` data.

---

## Key Files Quick Reference

| File | Role |
|------|------|
| `packages/exporter/src/main.rs` | Entry point; version constant; dev HTTP server |
| `packages/exporter/src/setup.rs` | Download, mod injection, Factorio execution, sprite conversion |
| `packages/exporter/src/export-data/info.json` | Mod descriptor |
| `packages/exporter/src/export-data/data-final-fixes.lua` | Prototype extraction from `data.raw` |
| `packages/exporter/src/export-data/control.lua` | JSON serialization and file output |
| `packages/exporter/data/output/data.json` | Generated: all game data (2.3 MB) |
| `packages/exporter/data/output/__base__/` | Generated: base game sprites (.basis) |
| `packages/exporter/data/output/__core__/` | Generated: core UI sprites (.basis) |
| `packages/website/vite.config.js` | Dev proxy + prod static copy config |
| `packages/editor/src/Editor.ts` | Fetches data.json, initializes Basis transcoder |
| `packages/editor/src/core/factorioData.ts` | Parses JSON into `FD` global; type definitions |
| `packages/editor/src/common/globals.ts` | `getTexture()` — .basis loading via pixi.js |
| `packages/editor/src/core/spriteDataBuilder.ts` | Sprite rendering logic per entity type (2,117 LOC) |

---

## Space Age Implications

Areas that will need changes for DLC support (implementation details TBD in a separate doc):

- **Version target** — `main.rs` version constant must point to a Space Age–capable Factorio build
- **Lua extraction** — `data-final-fixes.lua` already iterates all prototype types generically; new DLC prototypes should appear automatically in `data.raw`. New prototype *types* (if any) would need explicit addition to the iteration lists.
- **Sprite processing** — New DLC sprites flow through the same PNG -> Basis pipeline; DLC mod paths (e.g., `__space-age__/graphics/...`) need to be resolved alongside `__base__` and `__core__`
- **`factorioData.ts`** — New entity categories, quality system fields, and planet data need TypeScript modeling
- **`spriteDataBuilder.ts`** — New entity types need rendering logic (largest effort)
- **`typed-factorio`** — Must be updated to include Space Age prototype types
- **Inventory layout** — New groups/subgroups from the DLC will appear automatically if `inventoryLayout` extraction is generic enough
- **Blueprint validation** — Schema keywords auto-validate against `FD` contents, so new names propagate; new *fields* (quality, surface) need schema updates
