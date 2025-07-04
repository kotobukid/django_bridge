# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# Django Bridge Project - AI Assistant Guide

## Project Overview
This is a WIXOSS Trading Card Game management system that bridges Django (for database management) with Rust (for performance-critical operations) and a Leptos frontend.

## Important Directories to Avoid
- **DO NOT EXPLORE**: `text_cache/`, `custom_cache` - Contains thousands of cached HTML files
- **DO NOT EXPLORE**: `datapack/src/gen/` - Contains large generated files (especially cards.rs)
- **AVOID**: Hidden directories like `.git`, `node_modules`, etc. unless specifically needed

## Key Commands

### Database Operations
```bash
# Make Django migrations
python manage.py makemigrations  # Run from table_definition/ using .venv

# Apply migrations
python manage.py migrate  # Run from table_definition/ using .venv

# Sync Django models to Rust
cargo make syncdb
```

### Running Servers
```bash
# Rust API server (port 8002)
cargo make server

# Frontend dev server (port 3001)
cargo make front_server

# Django admin (port 8003)
python manage.py runserver 0.0.0.0:8003  # Run from table_definition/ using .venv

# Fixed Data Server (port 8004) - Manual feature override management
cargo make fixed_data_server
```

### Scraping and Analysis
```bash
# Scrape cards from a specific product
cargo run --release -p wxdb-scraper -- booster WX24-P1
# Or use the convenience command:
cargo make scraper booster WX24-P1

# Fast scraping with reduced delays
cargo make scraper-fast booster WX24-P1

# Analyze scraped RawCard data
cargo run -p analyzer -- --limit 100 --verbose

# Analyze cards from a specific product (by name)
cargo run -p analyzer -- --product "GLOWING DIVA" --limit 1000

# Analyze cards from a specific product (by code)
cargo run -p analyzer -- --product "WXDi-P01" --limit 1000

# Force re-analysis of specific product cards
cargo run -p analyzer -- --product "WXDi-P01" --force

# Force re-analysis of all cards
cargo make analyze_all
```

### Build Commands
```bash
# Generate static data from database
cargo make static

# Build Leptos frontend
cargo make trunk_build

# Build WASM datapack module
cargo make wasm_linux

# Complete rebuild sequence (after data changes)
cargo make static && cargo make wasm_linux && cargo make trunk_build
```

## Project Structure

### Backend
- `server/` - Main Axum web server (port 8002)
- `shared/webapp/` - Core business logic and API routes
- `shared/models/` - Rust data models (auto-generated from Django)
- `table_definition/wix/models.py` - Django model definitions (source of truth)
- `scraper/` - Card data scraping from web sources
- `analyzer/` - RawCard analysis and feature extraction

### Frontend
- `wasm_front/` - Leptos WASM application (primary frontend)
- `front/` - Nuxt.js application (legacy, being phased out)

### Tools
- `syncdb/` - Converts Django models to Rust structs
- `datapack/` - WebAssembly module for frontend filtering
- `shared/feature/` - Card feature detection logic
- `static_generator/` - Generates static Rust code from database (with manual override support)
- `icon_encoder/` - Icon encoding/decoding system for skill text rendering
- `fixed_data_server/` - Manual CardFeature override management server (port 8004)

## Development Workflow

1. **Model Changes**: Always start with Django models in `table_definition/wix/models.py`
2. **Generate Migrations**: Run `python manage.py makemigrations` from `table_definition/` using `.venv`
3. **Apply Migrations**: Run `python manage.py migrate` from `table_definition/` using `.venv`
4. **Sync to Rust**: Run `cargo make syncdb` to update Rust structs
5. **Test**: Run servers and verify changes

## Environment Variables
- `BACKEND_PORT` - Main server port (default: 8002)
- `DJANGO_ADMIN_PORT` - Django admin port (default: 8003)

## Database
- PostgreSQL is required
- Connection configured in Django settings
- Rust uses the same database via SQLx
- **Python Environment**: Use `table_definition/.venv` for Django operations

## Python Environment
**IMPORTANT**: Always use the Python environment located at `table_definition/.venv` for all Python operations.

Before running any Python commands:
```bash
# Activate the virtual environment
source table_definition/.venv/bin/activate
```

Or run Python commands directly using the virtual environment:
```bash
# Example: Django commands
table_definition/.venv/bin/python manage.py makemigrations
table_definition/.venv/bin/python manage.py migrate
table_definition/.venv/bin/python manage.py runserver 0.0.0.0:8003

# Example: Python scripts
table_definition/.venv/bin/python script_name.py
```

## Ad-hoc Analysis Scripts
**Location and Execution Rules**: All ad-hoc scripts for analyzing code or database should be placed in the `scripts/` directory.

**Execution Format**: Always run scripts from the project root using relative paths:
```bash
# Correct execution from project root
python scripts/hoge.py
table_definition/.venv/bin/python scripts/analyze_data.py

# Script should handle relative imports and paths properly
```

**Script Writing Guidelines**:
- Place all analysis scripts in `scripts/` directory
- Write scripts to work when executed from project root
- Use relative paths for imports and file access
- Include proper Django setup if accessing database models
- Add meaningful comments and docstrings for future reference

## Static Data Generation Critical Process
**When to Regenerate**: Always run `cargo make static` after any database schema changes or card data updates.

**Process Flow**:
1. Database changes (via analyzer or direct updates)
2. Run `cargo make static` to regenerate `datapack/src/gen/cards.rs`
3. WASM rebuild (if needed): `cargo make wasm_linux`
4. Frontend rebuild to pick up new data

**Field Mapping in Static Data**: The static data generation process converts database records to Rust tuples. Critical fields include:
- Position 6: level (String)
- Position 7: limit (String) 
- Position 8: limit_ex (String)
- Position 9: power (String)

**Important**: The `Card::to_rust_code()` method in `shared/models/src/card.rs` handles the conversion from database `Option<i32>` fields to string representation for static data.

**Manual Override System**: The static generator now checks for manual feature overrides in the `wix_card_feature_override` table. If an override exists for a card's pronunciation, the fixed feature bits are used instead of the auto-detected ones.

## Data Flow Architecture

### Production Data Pipeline ✅ **COMPLETED**
```
[Web Scraping] → [RawCard DB] → [Text Analysis] → [Card DB] → [Frontend]
```

1. **Scraper** (`scraper/` crate):
   - Downloads HTML from card detail pages
   - Extracts card names, skill text, and life burst text
   - Saves to `RawCard` table with proper `product_id` mapping
   - Handles upsert logic to prevent duplicates

2. **Analyzer** (`analyzer/` crate):
   - Reads `RawCard` records from database
   - Analyzes skill text and life burst text for features
   - Creates `Card` records with proper `product_id` propagation
   - Marks `RawCard` as analyzed when complete

3. **Product Relationship**:
   - Products are pre-loaded in database with unique codes
   - Scraper maps product codes to IDs via ProductRepository
   - Both `RawCard` and `Card` tables maintain product foreign keys

## Manual Feature Override System
The project now includes a manual feature override system for fine-tuning CardFeature detection beyond what rule-based automation can achieve.

### Architecture Overview
1. **CardFeatureOverride Model**: Django model storing manual corrections by pronunciation
2. **Fixed Data Server**: Axum server (port 8004) providing CRUD API for overrides
3. **Static Generator Integration**: Automatically applies overrides during code generation
4. **Frontend Integration**: Maintenance mode for editing features (coming soon)

### Key Commands
```bash
# Start Fixed Data Server
cargo make fixed_data_server

# Apply overrides and regenerate static data
cargo make static && cargo make wasm_linux && cargo make trunk_build
```

### API Endpoints (port 8004)
- `GET /api/overrides` - List all overrides
- `POST /api/overrides` - Create/update override
- `GET /api/overrides/:pronunciation` - Get specific override
- `DELETE /api/overrides/:pronunciation` - Delete override
- `POST /api/analyze/:pronunciation` - Re-analyze specific cards
- `GET /api/export` - Export all overrides (for backup/sync)
- `POST /api/import` - Import overrides (for backup/sync)
- `GET /api/check-consistency` - Check which overrides match rule-based detection

### Data Flow
1. **Rule-based detection** generates initial CardFeature bits
2. **Manual overrides** stored in `wix_card_feature_override` table (by pronunciation)
3. **Static generator** checks for overrides and applies them during code generation
4. **Frontend** receives corrected feature data through static generation

### Override Data Structure
- **pronunciation**: Primary key (same pronunciation = same features)
- **fixed_bits1/fixed_bits2**: Manual CardFeature bit flags
- **fixed_burst_bits**: Manual BurstFeature bit flags
- **note**: Optional explanation for the override

## Rule Editor Tool
`rule_editor/` is a specialized tool for creating and managing regex patterns to detect card features from WIXOSS card text.

### Key Commands
```bash
# Start web interface (recommended)
cargo run -p rule_editor web
# Access at http://localhost:3030

# Export patterns to Rust code
cargo run -p rule_editor export

# Run without arguments defaults to web mode
cargo run -p rule_editor
```

### Workflow
1. **Search**: Use keywords like "手札に加える" (add to hand) to search wix_rawcard table
2. **Feature Selection**: Choose target CardFeature (e.g., "トラッシュ回収" for Salvage)
3. **Classification**: Use continuous mark mode to classify sentences as positive/negative/ignore
4. **AI Generation**: Generate regex patterns using OpenAI API with Tool Calling
5. **Save**: Store patterns in wix_rule_pattern table for code generation

### Architecture
- **Backend**: Axum web server with OpenAI API integration
- **Frontend**: Nuxt 3 SPA with Tailwind CSS
- **Database**: Uses existing PostgreSQL (wix_rawcard, wix_rule_pattern tables)
- **Output**: Generates `shared/feature/src/generated_patterns.rs`

### AI Pattern Generation
- Uses OpenAI GPT-4o-mini with Tool Calling and JSON mode
- Designed to create simple patterns like those in `shared/feature/src/lib.rs`
- Focuses on core functionality keywords rather than complex matching
- Example: "手札に加える" for Salvage detection instead of complex conditional patterns

### UI Features
- **Continuous Mark Mode**: Keyboard navigation (↑↓←→ Space Esc) for efficient classification
- **Visual Feedback**: Active item highlighting and auto-scroll
- **Pattern Preview**: Shows generated regex with explanation before saving

## Linting and Code Quality
```bash
# Run clippy (Rust linter)
cargo clippy --all-targets --all-features

# Run rustfmt
cargo fmt --all

# Type check TypeScript/JavaScript in Nuxt frontend
cd front && npm run typecheck

# Type check rule_editor frontend
cd rule_editor/ui && npm run typecheck
```

## Common Tasks

### Adding a New Product for Scraping
1. Ensure product exists in database: Check `wix_product` table for the product code
2. Run scraper: `cargo run --release -p wxdb-scraper -- booster <PRODUCT_CODE>`
3. Run analyzer: `cargo run -p analyzer -- --limit 1000`

### Adding a New API Endpoint
1. Add route handler in `shared/webapp/src/routers/`
2. Register in router configuration
3. Add corresponding frontend API call

### Modifying Card Data Structure
1. Update Django model in `table_definition/wix/models.py`
2. Run migration commands from `table_definition/` using `.venv`
3. Run `cargo make syncdb`
4. Update any affected Rust code
5. Update frontend if needed

### Debugging Scraper Issues
1. Check if product exists: `python manage.py shell -c "from wix.models import Product; print(Product.objects.filter(product_code='WX24-P1'))"` from `table_definition/` using `.venv`
2. Test with existing product codes from database
3. Check ProductRepository cache initialization in scraper

## Architecture Notes
- The project uses a unique Django-Rust hybrid architecture
- Django handles all database migrations and schema
- Rust provides the API and business logic
- Frontend is a separate Leptos application
- **Scraping and analysis are completely decoupled** for better testing and reliability

## Critical Design Decisions & Lessons Learned

### ⚠️ Symbol Preservation in Text Processing
**CRITICAL**: The analyzer crate depends heavily on specific symbols in card text for feature detection.

**Key Symbols That Must Be Preserved**:
- `【】` brackets: Used for ability detection (`【アサシン】`, `【チャーム】`, `【ライフバースト】`, etc.)
- `《》` brackets: Used for game mechanic detection (`《ガードアイコン》`, `《クラフト》`, etc.)
- `:` colons: Used for timing detection (`出：`, `自：`, `起：`, etc.)

**Location**: `scraper/src/raw_card.rs` - `replace_img_tags_with_alt()` function
- **DO NOT remove** `【】` or `《》` symbols during HTML processing
- These symbols are essential for pattern matching in `analyzer/src/lib.rs` and `shared/feature/src/lib.rs`
- Removing them breaks dozens of detection rules and causes silent feature detection failures

**Historical Issue**: Previously symbols were stripped for "clean text", causing:
- Card colors detected incorrectly (e.g., green cards showing as 0 results)
- Abilities like Guard, Assassin, Charm not being detected
- Skill text missing timing indicators (`出：`, `自：`)

**Current Implementation** (2025-01-10): Symbols are preserved to maintain detection accuracy.

## Implementation Status

### ✅ Completed - RawCard System & Field Extraction

**Icon Encoding/Decoding System**: Complete pipeline for replacing HTML icon images with Leptos components in WASM frontend:
- `icon_encoder/` crate provides encoding (static_generator) and decoding (wasm_front) functions  
- Pattern definitions in `icon_encoder/src/definitions.rs` map symbols like `【出】` to codes like `[c]`
- Macro system in `wasm_front/src/components/skill_text_renderer.rs` auto-generates component mapping
- 22+ icon types supported including timing, abilities, mechanics, and collaboration icons
1. **RawCard Django Model**: Complete with product foreign key relationship
2. **Database Migration**: Applied with proper constraints and indexing
3. **Rust Model Generation**: Auto-synced via `syncdb`
4. **Scraper Integration**: 
   - HTML text extraction with proper CSS selectors
   - Product ID mapping via ProductRepository
   - Upsert functionality prevents duplicates
   - Error handling for missing products
5. **Analyzer System**:
   - Standalone binary crate (`analyzer/src/main.rs`)
   - Product ID propagation from RawCard to Card
   - Batch processing with progress tracking
   - CLI options for flexible operation
   - **Card Field Extraction**: Complete extraction of power, level, limit, timing, story fields from HTML
   - **Card Type Conditional Logic**: Proper field extraction based on card type (Lrig vs Signi vs Arts etc.)
6. **Frontend Integration**: 
   - CardItem component displays level, power, limit with colored badges
   - CardExport properly exposes all new fields via getter methods
7. **Static Data Pipeline**: End-to-end data flow from database to frontend via static generation
8. **Testing**: Complete end-to-end validation from scraping to final card data

### 🎯 Current Focus
- Code quality improvements (Clippy warnings)
- Feature detection enhancement in analyzer
- Performance optimization for large datasets
- WASM build optimization (current issues with wasm-opt)

## Testing
```bash
# Run Rust tests
cargo test

# Test specific analyzer features  
cargo test -p analyzer field_extraction_tests

# Test icon encoder system
cargo test -p icon_encoder

# Test individual crate
cargo test -p <crate_name>

# Test scraper with small dataset
cargo run --release -p wxdb-scraper -- booster WX24-P1

# Test analyzer on recent data
cargo run -p analyzer -- --limit 10 --verbose

# Test datapack field extraction
cargo run --example test_new_fields  # Run from datapack directory

# Check data integrity
python manage.py shell -c "from wix.models import RawCard, Card; print(f'RawCard: {RawCard.objects.count()}, Card: {Card.objects.count()}')"  # From table_definition/ using .venv

# Verify field extraction statistics
python scripts/test_field_extraction.py

# Run icon encoder integration tests
table_definition/.venv/bin/python scripts/test_icon_encoder_integration.py
```

## Frontend Development (Leptos/WASM)

### Running Frontend
```bash
# Development server with hot reload (port 8080)
cargo make trunk_serve

# Build for development (unoptimized, faster builds)
cargo make trunk_build

# Build for production (optimized and minified)
cargo make trunk_release

# Build for GitHub Pages deployment (if configured)
cargo make trunk_pages
```

### Frontend Architecture
- **Leptos framework** with reactive signals for state management
- **Component structure**: 
  - `wasm_front/src/components/` - Reusable UI components
  - `wasm_front/src/pages/` - Page-level components
  - `wasm_front/src/main.rs` - Router and app entry point
- **Styling**: Tailwind CSS with custom components in `input.css`
- **Data filtering**: Uses `datapack` WASM module for client-side card filtering

### Icon Rendering System
The frontend uses an advanced icon encoding/decoding system to replace HTML icon images with SVG Leptos components:

**Pipeline Flow**:
1. **Static Generation**: `static_generator` uses `icon_encoder::encode_skill_text()` to convert `【出】` → `[c]` in card data
2. **Frontend Decoding**: `SkillTextRenderer` uses `icon_encoder::decode_skill_text()` to parse encoded text
3. **Component Rendering**: Macro-generated `render_icon_component()` maps codes to SVG components

**Adding New Icons**:
1. Add rule to `icon_encoder/src/definitions.rs`: `["【新機能】", "[nf]", "IconNewFeature"]`
2. Add component to `wasm_front/src/components/skill_text_renderer.rs`: `"IconNewFeature" => IconNewFeature()`
3. Implement component function with SVG or styled span
4. Regenerate static data: `cargo make static && cargo make wasm_linux && cargo make trunk_build`

### Frontend UI Patterns & Architecture

#### Filter System Design
The frontend implements a two-tier filter system:
- **Always Visible Filters**: Color, CardType, Level, Text search, and Feature shortcuts
- **Overlay Filters**: Complex feature selection and product selection (accessed via overlay buttons)

#### Component State Management
- Use `RwSignal<T>` for shared mutable state across components
- Use `Signal::derive()` for computed/reactive values
- Use `WriteSignal<T>` for callback-based state updates
- Minimize reactive updates by batching state changes in single `update()` calls

#### Feature Integration Patterns
**Color vs CardFeature Distinction**: The system separates basic card colors (white, red, blue, green, black, colorless) from CardFeature-based filtering:
- **ColorSelector**: Handles basic 6-color filtering via `ColorFilter` struct
- **FeatureShortcuts**: Provides quick access to collaboration features (プリパラ, にじさんじ, ディソナ, 電音部, ブルーアーカイブ) with mutual exclusion behavior
- **FeatureOverlay**: Full feature selection organized by categories

**IMPORTANT - CardFeature Naming**: When working with CardFeatures, always use the Japanese display names as defined in `shared/feature/src/feature.rs`:
- Enum variants: `CardFeature::Pripara`, `CardFeature::Nijisanji`, `CardFeature::Dissona`, etc.
- Display labels (used in UI): "プリパラ", "にじさんじ", "ディソナ", "電音部", "ブルーアーカイブ"
- The filtering system uses the Japanese display labels, not the enum names

#### Responsive Design Patterns
Use Tailwind CSS responsive prefixes for adaptive layouts:
```rust
// Example: Side-by-side on PC, stacked on mobile
<div class="grid grid-cols-1 md:grid-cols-2 gap-3">
```

#### Accessibility Considerations
Always include proper ARIA labels and titles for interactive elements:
```rust
aria-label=feature_name
title=feature_name
```

**Additional Frontend Documentation**: See `wasm_front/CLAUDE.md` for detailed UI design patterns, overlay system implementation, and specific Leptos component architecture decisions.

## Nuxt.js Frontend (Legacy)
```bash
# Run development server (port 3001)
cargo make front_server

# Generate static build
cargo make generate
```

## Troubleshooting

### WASM Build Issues
If `cargo make wasm_linux` fails with wasm-opt errors:

1. **Check for bulk memory operations error**: This is a known issue with wasm-opt
2. **Workaround**: Disable wasm-opt by adding to `datapack/Cargo.toml`:
   ```toml
   [package.metadata.wasm-pack.profile.release]
   wasm-opt = false
   ```
3. **Alternative**: Use development builds which skip optimization

### Static Data Not Updating
If frontend doesn't show new field values:
1. Verify database has the data: Check with Django shell or scripts
2. Regenerate static data: `cargo make static`
3. Rebuild WASM: `cargo make wasm_linux`
4. Clear browser cache or use hard refresh

### Field Extraction Issues
If new fields aren't being extracted properly:
1. Test extraction methods: `cargo test -p analyzer field_extraction_tests`
2. Check HTML structure: Use `scripts/analyze_html_structure.py`
3. Verify card type detection: Ensure proper dd element extraction
4. Run field extraction verification: `python scripts/test_field_extraction.py`

### Icon System Issues
If icons show as fallback text (e.g., `[c]` instead of icon):
1. Check icon definition exists in `icon_encoder/src/definitions.rs`
2. Verify component mapping in `wasm_front/src/components/skill_text_renderer.rs`
3. Ensure static data regeneration: `cargo make static`
4. Test encoding/decoding: `cargo test -p icon_encoder`
5. Check browser console for component rendering errors