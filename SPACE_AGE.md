# Space Age DLC Support

**Goal:** Update Factorio Blueprint Editor for Space Age DLC compatibility

## Overview

The Factorio Space Age DLC (released October 2024) adds significant new content that needs to be reflected in the blueprint editor. This document outlines all new content and the implementation roadmap.

## New Content to Support

### New Planets & Surfaces
- **Nauvis** (vanilla)
- **Vulcanus** (lava planet)
- **Fulgora** (electrical storm planet)
- **Gleba** (organic/bio planet)
- **Aquilo** (ice planet)
- **Space platforms**

**Implementation Needs:**
- Multi-surface blueprint support
- Planet-specific entity availability
- Different tile sets per planet
- Space platform special mechanics

### New Entities

#### Production
- Big mining drill
- Foundry (advanced smelting)
- Electromagnetic plant (advanced crafting)
- Biochamber (Gleba organic processing)
- Cryogenic plant (Aquilo freezing)
- Recycler
- Asteroid collector
- Crusher

#### Quality System
- Quality modules (quality-1, quality-2, quality-3)
- Quality entities (all existing entities can have quality levels)
- Quality indicators in UI

#### Logistics
- Bulk inserter
- Elevated rails
- Advanced belts (turbo, upgraded variants)

#### Power
- Fusion reactor
- Fusion power cell
- Heating tower

#### Science
- Agricultural science pack (Gleba)
- Electromagnetic science pack (Fulgora)
- Cryogenic science pack (Aquilo)
- Metallurgic science pack (Vulcanus)
- Promethium science pack (space)

#### Space Platforms
- Space platform foundation
- Space platform hub
- Thruster
- Asteroid collector
- Cargo bay

### New Recipes
- Planet-specific recipes (hundreds of new recipes)
- Alternative recipes (scrap recycling, etc.)
- Quality-affected crafting

### New Mechanics

#### Quality System
- Every entity/item can have quality level (normal, uncommon, rare, epic, legendary)
- Quality affects entity stats (speed, modules slots, etc.)
- Quality modules increase quality chance
- Need to display quality in entity tooltips/info

#### Elevated Rails
- Rails can be at different heights
- Support pillars
- Ramps between levels

#### Space Platforms
- Moving platforms (thrust direction, velocity)
- Platform tiles with collision
- Asteroid fields

## Implementation Roadmap

### Phase 1: Core Entity Updates
**Priority: Critical**

1. Update entity definitions from latest Factorio data
2. Add all new entity types with correct graphics
3. Update recipe database
4. Add quality system data structures

**Tasks:**
- [ ] Extract Factorio Space Age game data
- [ ] Update entity definitions file
- [ ] Add new entity graphics/sprites
- [ ] Update recipe database
- [ ] Update item lists

**Data Extraction:**
```bash
# Extract from Factorio installation
/path/to/factorio --dump-data
```

### Phase 2: Quality System
**Priority: High**

1. Add quality attribute to all entities
2. UI for selecting/displaying entity quality
3. Quality module placement and effects
4. Quality filtering in entity picker

**Tasks:**
- [ ] Add quality enum (normal, uncommon, rare, epic, legendary)
- [ ] Extend entity data model with quality field
- [ ] UI dropdown for quality selection
- [ ] Quality badge/indicator in entity rendering
- [ ] Quality modules in module slots
- [ ] Entity picker filter by quality
- [ ] Blueprint string quality encoding

**UI Mockup:**
```
[Assembler-3] [▼ Legendary]
Modules: [Prod-3] [Prod-3] [Speed-3] [Quality-3]
```

### Phase 3: Multi-Surface Support
**Priority: High**

1. Blueprint book support for multi-surface blueprints
2. Surface/planet selector in UI
3. Planet-specific entity filtering
4. Different tile rendering per surface

**Tasks:**
- [ ] Surface/planet data model
- [ ] UI selector for active surface
- [ ] Filter entities by planet availability
- [ ] Load planet-specific tile graphics
- [ ] Multi-surface blueprint book import/export
- [ ] Validation for planet-specific recipes

**Planet Availability Matrix:**
```
Entity          | Nauvis | Vulcanus | Fulgora | Gleba | Aquilo | Space
----------------|--------|----------|---------|-------|--------|-------
Foundry         | ✓      | ✓        | ✓       | ✗     | ✗      | ✗
Biochamber      | ✗      | ✗        | ✗       | ✓     | ✗      | ✗
Cryogenic Plant | ✗      | ✗        | ✗       | ✗     | ✓      | ✗
```

### Phase 4: Space Platforms
**Priority: Medium**

1. Space platform foundation tiles
2. Thruster orientation and logic
3. Cargo bay configurations
4. Platform blueprint validation (connected foundations, etc.)

**Tasks:**
- [ ] Space platform tile rendering
- [ ] Foundation connection validation
- [ ] Thruster placement and direction
- [ ] Cargo bay configuration
- [ ] Platform boundary detection
- [ ] Thrust vector visualization (optional)

**Validation Rules:**
- All platform entities must be on foundation tiles
- Foundations must be connected
- At least one platform hub required
- Thrusters must be correctly oriented

### Phase 5: Elevated Rails
**Priority: Low**

1. Rail height attribute
2. Support pillar placement
3. Ramp generation
4. Height visualization

**Tasks:**
- [ ] Add height/elevation to rail entities
- [ ] Support pillar rendering
- [ ] Ramp tile generation
- [ ] Height overlay visualization
- [ ] Validation for proper height transitions

**Challenges:**
- Complex 3D-ish visualization in 2D editor
- May require significant rendering changes
- Consider deferring until other phases complete

## Data Sources

### Official Resources
- [Factorio Wiki](https://wiki.factorio.com/)
- [Factorio Lua API](https://lua-api.factorio.com/latest/)
- [Blueprint String Format](https://wiki.factorio.com/Blueprint_string_format)
- [Space Age Release Notes](https://factorio.com/blog/post/fff-373)

### Community Resources
- Factorio Data Raw dumps
- Community-maintained entity databases
- Space Age mod compatibility layers

## Testing & Quality Assurance

### Test Cases
- [ ] Import vanilla blueprints (backward compatibility)
- [ ] Import Space Age blueprints with quality entities
- [ ] Export blueprints and import to Space Age game
- [ ] Multi-surface blueprint books
- [ ] Quality module configurations
- [ ] Planet-specific recipe validation
- [ ] Space platform foundation connectivity
- [ ] Elevated rail height transitions

### Validation
- Blueprint string format compliance
- Entity availability per planet
- Recipe compatibility per surface
- Quality tier constraints
- Space platform structural integrity

### Compatibility
- Maintain backward compatibility with vanilla Factorio blueprints
- Graceful degradation when Space Age entities unavailable
- Version detection and warnings for incompatible features
- Clear error messages for planet-specific constraints

## Performance Considerations

- Large blueprint rendering with quality indicators
- Multi-surface blueprint switching
- Entity picker search with expanded entity list
- Graphics loading for new entities
- Memory usage for multiple surface tile sets

## Future Enhancements

- Planet surface preview backgrounds
- Recipe chain visualization per planet
- Quality probability calculator
- Space platform thrust calculator
- Elevated rail auto-routing

## Contributing

Contributions needed:
- Space Age entity/recipe data extraction
- Graphics/sprites for new entities
- Quality system testing
- Multi-surface blueprint examples
- Documentation improvements

## References

- [Original Project](https://github.com/teoxoy/factorio-blueprint-editor)
- [Factorio Space Age DLC](https://www.factorio.com/space-age)
- [Quality Mechanics Guide](https://wiki.factorio.com/Quality)

---

**Last Updated:** 2026-02-10  
**Fork Maintainer:** CraightonH  
**Original Author:** teoxoy
