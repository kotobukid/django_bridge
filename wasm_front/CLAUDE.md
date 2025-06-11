# WASM Frontend (Leptos) - UI Design Notes

## Overlay System Implementation

### Background
The card search interface was becoming cluttered with multiple filter components taking up significant screen space. To improve UX, we implemented an overlay system for complex filters while keeping frequently-used filters always visible.

### Design Decisions

#### Filter Hierarchy
- **Always Visible**: Color and Card Type selectors (most frequently used)
- **Overlay System**: Feature and Product selectors (more complex, less frequent)

#### Overlay Components Architecture
Instead of creating a generic `Overlay` component (which caused Leptos closure trait issues), we implemented overlay functionality directly in the main page component with specialized overlay content components:

- `FeatureOverlay` - Feature selection with categorized checkboxes
- `ProductOverlay` - Product selection grouped by type (booster, deck, other)
- `OverlayButton` - Reusable button with active state indicator

### Visual Design Features

#### Active State Indicators
Overlay buttons show visual feedback when filters are active:
- **Active state**: Blue background, blue border, blue text
- **Visual dot**: Small blue circle appears next to label when filters are applied
- **Inactive state**: White background with gray border

#### Overlay Background Transparency
**Problem**: Using Tailwind's `bg-opacity-*` classes affected child elements, making content semi-transparent.

**Solution**: Use inline RGBA styles for precise control:
```rust
style="background-color: rgba(0, 0, 0, 0.75);"
```

This ensures only the overlay background is semi-transparent while content remains fully opaque.

#### Responsive Design
- Overlays are responsive with `max-w-4xl` and mobile margins
- Product grid adapts from 1 column on mobile to 2 columns on medium screens
- Overlay content is scrollable with `max-h-[60vh]` to prevent overflow

### Technical Implementation Notes

#### State Management
- Uses Leptos `RwSignal` for mutable filter state
- `Signal::derive()` for reactive active state detection
- Proper event propagation handling with `stop_propagation()`

#### Performance Considerations
- Static product list filtering to only show products with actual cards
- Efficient grouping logic for product categorization
- Memoized signals for expensive computations

#### Code Organization
```
components/
├── overlay_button.rs     # Reusable overlay trigger button
├── feature_overlay.rs    # Feature selection content
├── product_overlay.rs    # Product selection content
└── mod.rs               # Component exports
```

### User Experience Improvements
1. **Reduced visual clutter** - Main interface now shows only essential filters
2. **Clear hierarchy** - Important filters always visible, complex ones on-demand
3. **Visual feedback** - Users can immediately see which overlay filters are active
4. **Mobile-friendly** - Overlays work well on smaller screens
5. **Intuitive interaction** - Click outside overlay to close, clear close button

### Future Considerations
- Consider implementing keyboard shortcuts for overlay toggling
- Could add animation transitions for smoother overlay appearance
- Product grouping logic could be made configurable
- Active filter count could be displayed in overlay buttons