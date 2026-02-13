# Space Age DLC Implementation Plan

**Deliverable:** Implement Space Age DLC support in the Factorio Blueprint Editor.

## Progress

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Unlock DLC Data Extraction | ✅ Done |
| 2 | Editor Data Model Updates | ⬜ Not started |
| 3 | Sprite Renderers for New Entities | ⬜ Not started |
| 4 | Quality System UI | ⬜ Not started |
| 5 | Surface/Planet Support | ⬜ Future |

## Context

The Factorio Blueprint Editor currently supports base-game 2.0 entities only. Space Age DLC (Oct 2024) adds ~27 new blueprintable entity types, quality tiers, elevated rails, space platforms, and planet surfaces. The existing data extraction pipeline *almost* works for DLC — the Lua extractor already lists many Space Age prototype types and `typed-factorio` 3.31.0 already has the TypeScript types. Three small exporter fixes unlock all DLC data; the bulk of work is implementing sprite renderers for 27 entity types.

---

## Phase 1: Unlock DLC Data Extraction ✅

**Goal:** Make the exporter produce `data.json` + sprites that include all Space Age content.

**Status:** Complete. All changes implemented, `cargo check` passes.

### 1.1 Enable DLC mods at runtime ✅
**File:** `packages/exporter/src/setup.rs` lines 155-170

Writes `mod-list.json` into the mods directory before Factorio launch, enabling `base`, `space-age`, `elevated-rails`, and `quality` mods.

### 1.2 Fix sprite path resolution ✅
**File:** `packages/exporter/src/setup.rs` lines 208-211

Replaced hardcoded `.replace("__core__", "core").replace("__base__", "base")` with generic regex `__(.+?)__` → `$1`. Handles all mod prefixes (`__space-age__`, `__elevated-rails__`, `__quality__`, and any future ones).

### 1.3 Declare DLC mod dependencies ✅
**File:** `packages/exporter/src/export-data/info.json`

Added optional dependencies so `data-final-fixes.lua` runs after DLC data is loaded:
```json
"dependencies": ["base >= 2.0.0", "? space-age", "? elevated-rails", "? quality"]
```

### 1.4 Audit prototype type lists ✅
**File:** `packages/exporter/src/export-data/data-final-fixes.lua`

- `itemPrototypes` list confirmed complete — no new item prototype types from DLC.
- `placeableEntityPrototypes` list confirmed complete — already includes all Space Age types.
- Added `turbo-loader` to `creativeEntities` (line 109).

### 1.5 Extract quality data ✅
**File:** `packages/exporter/src/export-data/data-final-fixes.lua` lines 462-483

Added `QUALITIES` section that iterates `data.raw.quality` and exports name, localised_name, icon, icons, icon_size, color, level, and order. Guarded by `if data.raw.quality` for base-game-only compatibility.

### Verification
- [x] `cargo check` passes
- [ ] Run `npm run start:exporter` (or `cargo run --release` in `packages/exporter/`)
- [ ] Confirm `data/output/data.json` contains Space Age entities (search for `"foundry"`, `"electromagnetic-plant"`, `"biochamber"`, `"cryogenic-plant"`, `"recycler"`)
- [ ] Confirm `data/output/__space-age__/` directory exists with `.basis` sprite files
- [ ] Confirm `data/output/__elevated-rails__/` and `__quality__/` directories exist
- [ ] Confirm `data.json` has a `qualities` key with 5 quality tiers

---

## Phase 2: Editor Data Model Updates

**Goal:** Make the editor accept and preserve Space Age blueprint data without rendering new entities yet.

### 2.1 Add `qualities` to FD
**File:** `packages/editor/src/core/factorioData.ts`

- Add `qualities: Record<string, QualityPrototype>` to the `FD` type and populate in `loadData()`.

### 2.2 Add `qualityName` AJV keyword
**File:** `packages/editor/src/core/bpString.ts`

Register a `qualityName` custom keyword validating against `FD.qualities`, mirroring the existing `entityName`/`itemName` pattern.

### 2.3 Blueprint schema updates
**File:** `packages/editor/src/core/blueprintSchema.json`

- Apply `qualityName` keyword to quality string fields in `BlueprintItemIDAndQualityIDPair`, `ItemFilter`, `SplitterFilter`, `LogisticFilter`.
- Add optional `quality` field to entity schema.

### 2.4 Entity quality property
**File:** `packages/editor/src/core/Entity.ts`

- Add getter/setter for `quality` backed by `IEntity.quality`.
- Ensure `serialize()` preserves quality.

### 2.5 Elevated rail grid alignment
**File:** `packages/editor/src/core/Blueprint.ts` line 493

Add elevated rail type names to `getFirstRailRelatedEntityPos()`: `elevated-curved-rail-a`, `elevated-curved-rail-b`, `elevated-half-diagonal-rail`, `elevated-straight-rail`, `rail-ramp`.

### 2.6 Train entity list
**File:** `packages/editor/src/core/bpString.ts` line 160

Add `artillery-wagon` to `trainEntityNames` set.

### Verification
- Import a Space Age blueprint string containing quality entities, elevated rails, and DLC assembler recipes.
- Confirm it loads without validation errors.
- Confirm export reproduces the original string (round-trip).
- DLC entities will appear as invisible/missing-sprite — that's expected at this phase.

---

## Phase 3: Sprite Renderers for New Entities

**Goal:** Implement `draw_*` functions in `spriteDataBuilder.ts` for all 27 unimplemented entity types.

**File:** `packages/editor/src/core/spriteDataBuilder.ts`

Grouped by complexity (implement in this order):

### 3.1 Simple entities (single sprite, directional)
Pattern: read `entity.graphics_set` or `entity.picture`/`entity.pictures`, return layers.
- `burner-generator`
- `valve`
- `proxy-container`
- `linked-container`
- `linked-belt`
- `lane-splitter`
- `turret` (base)

### 3.2 Medium entities (animation layers, fluid boxes)
Pattern: layered sprites with directional variants and pipe covers.
- `agricultural-tower` — has animation, crop area overlay
- `lightning-attractor` — directional, Fulgora-specific
- `fusion-generator` — heat connection patches like boiler
- `fusion-reactor` — similar to existing reactor
- `cargo-landing-pad` — multi-tile, connection points
- `cargo-bay` — space platform, hatches
- `asteroid-collector` — rotating arm animation
- `space-platform-hub` — large multi-tile entity

### 3.3 Rolling stock (train entities)
Pattern: directional sprites on rail grid, special collision.
- `locomotive`
- `cargo-wagon`
- `fluid-wagon`
- `artillery-wagon`
- `infinity-cargo-wagon`

### 3.4 Elevated rails (complex)
Pattern: rail segment sprites at elevation, support structures.
- `elevated-curved-rail-a`
- `elevated-curved-rail-b`
- `elevated-half-diagonal-rail`
- `elevated-straight-rail`
- `rail-ramp`
- `rail-support`

### 3.5 Space platform entity
- `thruster` — directional, thrust flame overlay

### Verification
- Place each entity type in a test blueprint.
- Confirm sprites render at correct position, size, and direction.
- Spot-check against in-game screenshots.

---

## Phase 4: Quality System UI

**Goal:** Display and edit entity quality in the editor.

### 4.1 Quality badge overlay
**File:** `packages/editor/src/containers/EntityContainer.ts` (or new overlay)

Render a small quality-tier icon badge on entities that have non-normal quality.

### 4.2 Entity info panel quality selector
**File:** `packages/editor/src/UI/EntityInfoPanel.ts`

Add a quality dropdown when editing an entity.

### 4.3 Inventory dialog quality filter
**File:** `packages/editor/src/UI/InventoryDialog.ts`

Optional: filter or indicate items by quality tier.

### Verification
- Import blueprint with legendary assemblers; confirm badge renders.
- Change quality via info panel; confirm export reflects change.

---

## Phase 5: Surface/Planet Support (Future)

Not implemented in this plan. Placeholder for:
- Blueprint-level `surface` property
- Planet-specific entity filtering
- Multi-surface blueprint books
- Space platform bounded grids

---

## File Change Summary

| File | Phase | Change |
|------|-------|--------|
| `packages/exporter/src/setup.rs` | 1 | mod-list.json write, generic path resolution |
| `packages/exporter/src/export-data/info.json` | 1 | DLC dependencies |
| `packages/exporter/src/export-data/data-final-fixes.lua` | 1 | quality extraction, prototype audit, turbo-loader |
| `packages/editor/src/core/factorioData.ts` | 2 | `FD.qualities` |
| `packages/editor/src/core/bpString.ts` | 2 | `qualityName` keyword, train entities |
| `packages/editor/src/core/blueprintSchema.json` | 2 | quality field additions |
| `packages/editor/src/core/Entity.ts` | 2 | quality getter/setter |
| `packages/editor/src/core/Blueprint.ts` | 2 | elevated rail alignment |
| `packages/editor/src/core/spriteDataBuilder.ts` | 3 | 27 new `draw_*` functions |
| `packages/editor/src/containers/EntityContainer.ts` | 4 | quality badge |
| `packages/editor/src/UI/EntityInfoPanel.ts` | 4 | quality selector |
