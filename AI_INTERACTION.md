# AI Interaction Support

**Goal:** Enable AI-assisted blueprint design via programmatic control

## Overview

This document outlines features to make the Factorio Blueprint Editor accessible to AI agents for automated blueprint creation. The goal is to enable AI assistants to design complex production lines, including belt weaving, direct insertion chains, and circuit logic, without requiring manual drag-and-drop interaction.

## Use Cases

- AI-assisted blueprint design via natural language
- Automated production ratio calculations and layout
- Complex belt routing and inserter placement
- Circuit network design and configuration
- Blueprint optimization and refactoring

## Proposed AI-Friendly Features

### 1. JavaScript API (Primary Approach)

**Priority: Critical**

Expose programmatic methods to the browser console for AI interaction via browser automation tools.

#### Core Methods

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

#### Benefits
- Most flexible approach
- Minimal UI changes required
- Can build entire blueprints programmatically
- Easy to test and debug

#### Implementation Plan

**Milestone 1: API Foundation**
- [ ] Create `src/core/editorAPI.ts` to expose methods
- [ ] Attach API to window object: `window.factorioEditor = editorAPI`
- [ ] Implement core entity placement methods
- [ ] Add connection/wiring methods
- [ ] Add export/import methods
- [ ] Write API documentation with examples
- [ ] Add TypeScript definitions for type safety

**Example Usage:**
```javascript
// Place a basic green circuit production line
const assembler1 = editor.placeEntity('assembler-2', {
  x: 0,
  y: 0,
  direction: 'north',
  recipe: 'electronic-circuit'
});

const assembler2 = editor.placeEntity('assembler-2', {
  x: 3,
  y: 0,
  direction: 'north',
  recipe: 'electronic-circuit'
});

// Add belt between them
editor.placeEntity('transport-belt', { x: 1, y: 0, direction: 'east' });
editor.placeEntity('transport-belt', { x: 2, y: 0, direction: 'east' });

// Export the blueprint
const bpString = editor.exportBlueprintString();
console.log(bpString);
```

#### Error Handling
```javascript
// Return structured errors for AI to parse
{
  success: false,
  error: {
    code: 'ENTITY_COLLISION',
    message: 'Cannot place entity: collision with existing entity at (5, 3)',
    entity: 'assembler-3',
    position: { x: 5, y: 3 },
    conflictingEntityId: 'entity-12'
  }
}
```

### 2. URL-Based Commands (Alternative/Complement)

**Priority: Low**

Allow blueprint construction via URL parameters.

#### Examples
```
https://fbe.teoxoy.com/?cmd=place&entity=assembler-3&x=0&y=0&direction=north
https://fbe.teoxoy.com/?blueprint=base64string&edit=add-modules&entity=5&module=prod3&count=4
```

#### Benefits
- No JavaScript execution required
- Stateless interaction
- Can chain commands via redirects
- Easy to bookmark/share specific operations

#### Challenges
- URL length limits for complex operations
- Requires page reload for each operation (slower)
- State management between operations

#### Implementation Plan
- [ ] Design URL command schema
- [ ] Add URL parser to handle commands on page load
- [ ] Implement command handlers
- [ ] Add state persistence between reloads
- [ ] Document URL API

### 3. Command Palette Enhancement

**Priority: Medium**

Extend existing keyboard shortcuts with text-based command input.

#### Proposed Commands
```
place assembler-3 at 5,5 facing north
connect entity-12 to entity-15 via red-wire
set recipe entity-8 to advanced-circuit
add modules prod-3 x4 to entity-8
export blueprint
```

#### Benefits
- Human-readable
- Discoverable via autocomplete
- Can be scripted via keyboard automation
- Familiar to users of VS Code, etc.

#### Challenges
- Requires natural language parsing
- More complex to implement
- May have ambiguous commands

#### Implementation Plan
- [ ] Add command input UI (keyboard shortcut to open)
- [ ] Implement command parser with fuzzy matching
- [ ] Add command autocomplete
- [ ] Create command help/documentation
- [ ] Support command history

### 4. Grid Coordinate System

**Priority: High**

Add visual grid coordinates to help AI agents and users specify exact positions.

#### Features
- Optional overlay showing X/Y coordinates
- Hoverable tiles showing current position
- Click-to-copy coordinates
- Configurable grid origin (center vs corner)

#### Display Options
- Toggle via UI checkbox or keyboard shortcut (e.g., `Ctrl+G`)
- Coordinate labels every N tiles (configurable, default: 5)
- Highlight current cursor position
- Semi-transparent overlay to not obscure entities

#### Implementation Plan
- [ ] Add coordinate overlay rendering layer
- [ ] UI toggle button
- [ ] Keyboard shortcut binding
- [ ] Configuration for label frequency
- [ ] Cursor position indicator
- [ ] Click-to-copy coordinates to clipboard

#### Mockup
```
     0    5   10   15   20
  ┌────┬────┬────┬────┬────
0 │    │    │    │    │
  ├────┼────┼────┼────┼────
5 │    │ A  │    │    │
  ├────┼────┼────┼────┼────
10│    │    │ B  │    │
  ├────┼────┼────┼────┼────
```

### 5. Batch Operation API

**Priority: Medium**

For complex blueprints, support multi-step operations in a single call.

#### API Design
```javascript
editor.executeBatch([
  { action: 'place', entity: 'assembler-3', x: 0, y: 0 },
  { action: 'place', entity: 'assembler-3', x: 3, y: 0 },
  { action: 'connect', from: 'entity-1', to: 'entity-2', type: 'inserter' },
  { action: 'set-recipe', entity: 'entity-1', recipe: 'electronic-circuit' }
]);
```

#### Benefits
- Atomic operations (all or nothing)
- Better performance (single render pass)
- Transaction-like behavior with rollback
- Reduced API calls for complex blueprints

#### Implementation Plan
- [ ] Design batch operation schema
- [ ] Implement transaction system with rollback
- [ ] Add validation before execution
- [ ] Optimize rendering for batch updates
- [ ] Document batch operation format

## Development Roadmap

### Phase 1: JavaScript API Foundation
**Status:** Not Started

**Goals:**
- Basic entity placement and manipulation
- Blueprint export/import
- Entity querying

**Deliverables:**
- [ ] Core API implementation
- [ ] TypeScript definitions
- [ ] API documentation
- [ ] Example scripts
- [ ] Unit tests

### Phase 2: Grid Coordinates & Visual Aids
**Status:** Not Started

**Goals:**
- Visual coordinate system
- AI-friendly positioning

**Deliverables:**
- [ ] Coordinate overlay rendering
- [ ] UI toggle
- [ ] Click-to-copy functionality
- [ ] Configuration options

### Phase 3: Advanced API Features
**Status:** Not Started

**Goals:**
- Batch operations
- Complex entity manipulation
- Validation and error handling

**Deliverables:**
- [ ] Batch operation API
- [ ] Enhanced error reporting
- [ ] Blueprint validation
- [ ] Performance optimization

### Phase 4: Command Palette (Optional)
**Status:** Not Started

**Goals:**
- Text-based command input
- Alternative to pure API

**Deliverables:**
- [ ] Command parser
- [ ] UI implementation
- [ ] Command documentation
- [ ] Autocomplete system

## AI Integration Patterns

### Pattern 1: Direct API Usage
AI agent uses browser automation to execute JavaScript directly:
```javascript
// AI generates this code
const api = window.factorioEditor;
api.placeEntity('assembler-3', { x: 0, y: 0, recipe: 'iron-gear-wheel' });
// ... more commands
const blueprint = api.exportBlueprintString();
```

### Pattern 2: Iterative Design
AI queries current state, designs next step, executes:
```javascript
// Step 1: Query current state
const entities = api.getEntities();

// Step 2: AI analyzes and plans next placement
// ...

// Step 3: Execute next step
api.placeEntity(/*...*/);
```

### Pattern 3: Batch Generation
AI designs entire blueprint offline, executes in single batch:
```javascript
const blueprint = generateBlueprintPlan(requirements);
api.executeBatch(blueprint);
```

## Testing

### Unit Tests
- Entity placement validation
- Connection logic
- Blueprint export/import
- Batch operation rollback

### Integration Tests
- Complete blueprint generation via API
- Error handling and recovery
- Performance with large blueprints

### AI Agent Testing
- Natural language → blueprint conversion
- Complex production line design
- Circuit network configuration
- Blueprint optimization

## Performance Considerations

- Defer rendering until batch complete
- Optimize entity lookup/query operations
- Lazy validation (validate on export, not every operation)
- Throttle UI updates during rapid API calls

## Security Considerations

- No sensitive data exposure via API
- Validate all inputs to prevent injection
- Rate limiting for API calls (prevent abuse)
- CORS configuration for API access

## Documentation

### For Developers
- API reference with all methods
- TypeScript definitions
- Architecture overview
- Extension points

### For AI Agents
- Common patterns and workflows
- Error handling guide
- Production ratio calculations
- Example blueprints

### For Users
- How to enable/use AI features
- Privacy and security info
- Troubleshooting guide

## Contributing

Contributions welcome, especially:
- API design feedback
- AI agent integration examples
- Performance improvements
- Bug reports and testing

## References

- [Original Project](https://github.com/teoxoy/factorio-blueprint-editor)
- [Factorio Blueprint String Format](https://wiki.factorio.com/Blueprint_string_format)
- [Browser Automation Best Practices](https://developer.chrome.com/docs/devtools/)

---

**Last Updated:** 2026-02-10  
**Fork Maintainer:** CraightonH  
**Original Author:** teoxoy
