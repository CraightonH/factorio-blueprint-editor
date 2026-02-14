# Factorio Data Exporter

Extracts entity prototypes, item data, and sprites from Factorio (including Space Age DLC) into JSON + `.basis` image files for use by the blueprint editor.

## Automated Updates

The GitHub workflow `.github/workflows/update-factorio-data.yml` automatically:
- Runs monthly to check for Factorio updates
- Can be triggered manually via Actions tab
- Builds a container with Factorio + Space Age DLC (via `factoriotools/factorio`)
- Extracts all game data to `data/output/`
- Creates a PR with updated data files

**No local Factorio installation required** - the workflow handles everything.

## Manual Local Execution

If you need to run the exporter locally:

### Prerequisites
- Factorio installed with Space Age DLC
- Rust toolchain (`cargo`)

### Using Docker (Recommended)

```bash
cd packages/exporter

# Build the container
docker build -t factorio-exporter .

# Run extraction (output goes to data/output/)
docker run --rm -v "$(pwd)/data/output:/exporter/data/output" factorio-exporter
```

### Without Docker

```bash
cd packages/exporter

# Set Factorio installation path
export FACTORIO_BASE_DIR=/path/to/factorio  # e.g., /opt/factorio

# Run exporter
cargo run --release
```

The exporter expects Factorio at `$FACTORIO_BASE_DIR` with this structure:
```
$FACTORIO_BASE_DIR/
├── bin/x64/factorio
├── data/
│   ├── base/
│   ├── space-age/
│   ├── elevated-rails/
│   └── quality/
└── mods/
```

## Output

Generated files in `data/output/`:
- `data.json` - All entity/item prototypes + metadata (~2MB)
- `metadata.json` - File hashes for incremental updates
- `__base__/` - Base game sprite assets (.basis format)
- `__space-age__/` - Space Age DLC sprites
- `__elevated-rails__/` - Elevated Rails DLC sprites
- `__quality__/` - Quality DLC sprites

## How It Works

1. **Inject custom Lua scripts** into Factorio's mod directory
2. **Launch Factorio headless** with `--start-server-load-scenario`
3. **Extract data** via Lua during the data loading phase:
   - All entity/item/recipe prototypes → JSON
   - Locale strings (English) → embedded in output
   - Quality tiers (if DLC present)
4. **Convert sprites** from PNG to `.basis` (GPU-compressed format)
   - Only processes changed files (via metadata.json cache)
   - Parallel processing for speed

## Space Age Support

Phase 1 implementation (✅ Complete):
- DLC mods enabled via `mod-list.json`
- Generic sprite path resolution (`__mod-name__` → `mod-name`)
- Quality data extraction
- All 27 new Space Age entity types included in output

See `docs/space-age-implementation-plan.md` for full roadmap.
