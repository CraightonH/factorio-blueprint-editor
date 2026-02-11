# AI Interaction & Space Age Support

**Fork Purpose:** Enable AI-assisted blueprint design and update for Factorio Space Age DLC

## AI Interaction Goals

This fork aims to make the Factorio Blueprint Editor accessible to AI agents for automated blueprint creation. The goal is to enable AI assistants to design complex production lines, including belt weaving, direct insertion chains, and circuit logic, without requiring manual drag-and-drop interaction.

### Use Cases

- AI-assisted blueprint design via natural language
- Automated production ratio calculations and layout
- Complex belt routing and inserter placement
- Circuit network design and configuration
- Blueprint optimization and refactoring

## Proposed AI-Friendly Features

### 1. JavaScript API (Primary Approach)

Expose programmatic methods to the browser console for AI interaction via browser automation tools.

**Core Methods:**
```javascript
// Entity Placement
editor.placeEntity(entityName, options)
// options: { x, y, direction, recipe, modules, filters, etc. }

// Connection Management
editor.connectEntities(sourceId, targetId, connectionType)
// connectionType: 'belt', 'inserter', 'circuit-red', 'circuit-green', 'pipe'

// Entity Manipulation
editor.removeEntity(entityId)
editor.rotateEntity(entityId, direction)
editor.copySelection(entityIds)
editor.pasteSelection(x, y)

// Blueprint Operations
editor.exportBlueprintString()
editor.importBlueprintString(bpString)
editor.clear()

// State Queries
editor.getEntities() // Returns all placed entities with positions
editor.getEntity(entityId) // Get specific entity details
editor.getBounds() // Get blueprint dimensions
editor.validateBlueprint() // Check for errors/warnings

// Utility
editor.setGridSize(width, height)
editor.centerView()
editor.zoomTo(level)
```

**Benefits:**
- Most flexible approach
- Minimal UI changes required
- Can build entire blueprints programmatically
- Easy to test and debug

**Implementation Plan:**
1. Create `src/core/editorAPI.ts` to expose methods
2. Attach API to window object: `window.factorioEditor = editorAPI`
3. Document all methods and parameters
4. Add TypeScript definitions for type safety

### 2. URL-Based Commands (Alternative/Complement)

Allow blueprint construction via URL parameters.

**Examples:**
```
https://fbe.teoxoy.com/?cmd=place&entity=assembler-3&x=0&y=0&direction=north
https://fbe.teoxoy.com/?blueprint=base64string&edit=add-modules&entity=5&module=prod3&count=4
```

**Benefits:**
- No JavaScript execution required
- Stateless interaction
- Can chain commands via redirects
- Easy to bookmark/share specific operations

**Challenges:**
- URL length limits for complex operations
- Requires page reload for each operation (slower)
- State management between operations

### 3. Command Palette Enhancement

Extend existing keyboard shortcuts with text-based command input.

**Proposed Commands:**
```
place assembler-3 at 5,5 facing north
connect entity-12 to entity-15 via red-wire
set recipe entity-8 to advanced-circuit
add modules prod-3 x4 to entity-8
export blueprint
```

**Benefits:**
- Human-readable
- Discoverable via autocomplete
- Can be scripted via keyboard automation
- Familiar to users of VS Code, etc.

**Challenges:**
- Requires natural language parsing
- More complex to implement
- May have ambiguous commands

### 4. Grid Coordinate System

Add visual grid coordinates to help AI agents and users specify exact positions.

**Features:**
- Optional overlay showing X/Y coordinates
- Hoverable tiles showing current position
- Click-to-copy coordinates
- Configurable grid origin (center vs corner)

**Display Options:**
- Toggle via UI checkbox or keyboard shortcut
- Coordinate labels every N tiles (configurable)
- Highlight current cursor position

### 5. Batch Operation API

For complex blueprints, support multi-step operations in a single call.

```javascript
editor.executeBatch([
  { action: 'place', entity: 'assembler-3', x: 0, y: 0 },
  { action: 'place', entity: 'assembler-3', x: 3, y: 0 },
  { action: 'connect', from: 'entity-1', to: 'entity-2', type: 'inserter' },
  { action: 'set-recipe', entity: 'entity-1', recipe: 'electronic-circuit' }
]);
```

**Benefits:**
- Atomic operations (all or nothing)
- Better performance (single render pass)
- Transaction-like behavior with rollback

## Space Age DLC Support

The Factorio Space Age DLC (released October 2024) adds significant new content that needs to be reflected in the blueprint editor.

### New Content to Support

#### New Planets & Surfaces
- Nauvis (vanilla)
- Vulcanus (lava planet)
- Fulgora (electrical storm planet)
- Gleba (organic/bio planet)
- Aquilo (ice planet)
- Space platforms

**Implementation Needs:**
- Multi-surface blueprint support
- Planet-specific entity availability
- Different tile sets per planet
- Space platform special mechanics

#### New Entities

**Production:**
- Big mining drill
- Foundry (advanced smelting)
- Electromagnetic plant (advanced crafting)
- Biochamber (Gleba organic processing)
- Cryogenic plant (Aquilo freezing)
- Recycler
- Asteroid collector
- Crusher

**Quality System:**
- Quality modules (quality-1, quality-2, quality-3)
- Quality entities (all existing entities can have quality levels)
- Quality indicators in UI

**Logistics:**
- Bulk inserter
- Elevated rails
- Advanced belts (turbo, upgraded variants)

**Power:**
- Fusion reactor
- Fusion power cell
- Heating tower

**Science:**
- Agricultural science pack (Gleba)
- Electromagnetic science pack (Fulgora)
- Cryogenic science pack (Aquilo)
- Metallurgic science pack (Vulcanus)
- Promethium science pack (space)

**Space Platforms:**
- Space platform foundation
- Space platform hub
- Thruster
- Asteroid collector
- Cargo bay

#### New Recipes
- Planet-specific recipes (hundreds of new recipes)
- Alternative recipes (scrap recycling, etc.)
- Quality-affected crafting

#### New Mechanics

**Quality System:**
- Every entity/item can have quality level (normal, uncommon, rare, epic, legendary)
- Quality affects entity stats (speed, modules slots, etc.)
- Quality modules increase quality chance
- Need to display quality in entity tooltips/info

**Elevated Rails:**
- Rails can be at different heights
- Support pillars
- Ramps between levels

**Space Platforms:**
- Moving platforms (thrust direction, velocity)
- Platform tiles with collision
- Asteroid fields

### Implementation Priorities

#### Phase 1: Core Entity Updates
1. Update entity definitions from latest Factorio data
2. Add all new entity types with correct graphics
3. Update recipe database
4. Add quality system data structures

#### Phase 2: Quality System
1. Add quality attribute to all entities
2. UI for selecting/displaying entity quality
3. Quality module placement and effects
4. Quality filtering in entity picker

#### Phase 3: Multi-Surface Support
1. Blueprint book support for multi-surface blueprints
2. Surface/planet selector in UI
3. Planet-specific entity filtering
4. Different tile rendering per surface

#### Phase 4: Space Platforms
1. Space platform foundation tiles
2. Thruster orientation and logic
3. Cargo bay configurations
4. Platform blueprint validation (connected foundations, etc.)

#### Phase 5: Elevated Rails
1. Rail height attribute
2. Support pillar placement
3. Ramp generation
4. Height visualization

### Data Source

Factorio data extraction:
```bash
# Extract from Factorio installation
/path/to/factorio --dump-data
# Or use community-maintained data dumps
```

**Community Resources:**
- Factorio Wiki: https://wiki.factorio.com/
- Factorio Data Raw: https://lua-api.factorio.com/latest/Data-Lifecycle.html
- Factorio Blueprint String Format: https://wiki.factorio.com/Blueprint_string_format

### Testing

**Quality Assurance:**
- Test blueprints created in-editor import correctly to Space Age game
- Verify all new entities render correctly
- Check recipe compatibility
- Quality system edge cases (legendary quality, quality modules)
- Multi-surface blueprint book import/export

**Compatibility:**
- Maintain backward compatibility with vanilla Factorio blueprints
- Graceful degradation when Space Age entities are unavailable
- Version detection and warnings

## Development Roadmap

### Milestone 1: JavaScript API Foundation
- [ ] Design API interface
- [ ] Implement core entity placement methods
- [ ] Add connection/wiring methods
- [ ] Add export/import methods
- [ ] Write API documentation
- [ ] Create example scripts

### Milestone 2: Space Age Data Update
- [ ] Extract latest Factorio Space Age data
- [ ] Update entity definitions
- [ ] Update recipe database
- [ ] Add new entity graphics
- [ ] Update item/entity lists

### Milestone 3: Quality System
- [ ] Add quality data model
- [ ] UI for quality selection
- [ ] Quality rendering/indicators
- [ ] Quality module support
- [ ] Quality filtering

### Milestone 4: Advanced Features
- [ ] Multi-surface blueprint support
- [ ] Space platform mechanics
- [ ] Elevated rails (if feasible)
- [ ] Grid coordinate overlay
- [ ] Batch operation API

### Milestone 5: AI Integration Testing
- [ ] Test JavaScript API with AI agents
- [ ] Document AI workflow patterns
- [ ] Create example AI-generated blueprints
- [ ] Performance optimization for large blueprints
- [ ] Error handling and validation

## Contributing

This fork welcomes contributions, especially:
- Space Age entity/recipe data
- Graphics/sprites for new entities
- JavaScript API improvements
- AI interaction examples
- Bug reports and testing

## References

- [Original Project](https://github.com/teoxoy/factorio-blueprint-editor)
- [Factorio Wiki](https://wiki.factorio.com/)
- [Blueprint String Format](https://wiki.factorio.com/Blueprint_string_format)
- [Factorio Lua API](https://lua-api.factorio.com/latest/)
- [Space Age Release Notes](https://factorio.com/blog/post/fff-373)

## License

Same as original: MIT License

---

**Last Updated:** 2026-02-10  
**Fork Maintainer:** CraightonH  
**Original Author:** teoxoy
