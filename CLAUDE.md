# Django Bridge Project - AI Assistant Guide

## Project Overview
This is a WIXOSS Trading Card Game management system that bridges Django (for database management) with Rust (for performance-critical operations) and a Nuxt.js frontend.

## Important Directories to Avoid
- **DO NOT EXPLORE**: `shared/webapp/text_cache/` - Contains thousands of cached HTML files
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
```

### Scraping and Analysis
```bash
# Scrape cards from a specific product
cargo run --release -p wxdb-scraper -- booster WX24-P1

# Analyze scraped RawCard data
cargo run -p analyzer -- --limit 100 --verbose

# Force re-analysis of all cards
cargo make analyze_all
```

### Build Commands
```bash
# Generate static data
cargo make static

# Build WASM module
cargo make wasm_linux  # or cargo make wasm on other platforms

# Generate frontend
cargo make generate

# Format Rust code
cargo make fmt
```

## Project Structure

### Backend
- `server/` - Main Axum web server
- `shared/webapp/` - Core business logic and API routes
- `shared/models/` - Rust data models (auto-generated from Django)
- `table_definition/wix/models.py` - Django model definitions (source of truth)
- `scraper/` - Card data scraping from web sources
- `analyzer/` - RawCard analysis and feature extraction

### Frontend
- `front/` - Nuxt.js 3 application
- `front/pages/` - Page components
- `front/components/` - Reusable Vue components
- `front/stores/` - Pinia stores

### Tools
- `syncdb/` - Converts Django models to Rust structs
- `datapack/` - WebAssembly module for frontend filtering
- `shared/feature/` - Card feature detection logic

## Development Workflow

1. **Model Changes**: Always start with Django models in `table_definition/wix/models.py`
2. **Generate Migrations**: Run `python manage.py makemigrations` from `table_definition/` using `.venv`
3. **Apply Migrations**: Run `python manage.py migrate` from `table_definition/` using `.venv`
4. **Sync to Rust**: Run `cargo make syncdb` to update Rust structs
5. **Test**: Run servers and verify changes

## Environment Variables
- `DJANGO_BRIDGE_PORT` - Main server port (default: 8002)
- `NUXT_DEV_PORT` - Frontend dev port (default: 3001)
- `DJANGO_ADMIN_PORT` - Django admin port (default: 8003)

## Database
- PostgreSQL is required
- Connection configured in Django settings
- Rust uses the same database via SQLx
- **Python Environment**: Use `table_definition/.venv` for Django operations

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
- Frontend is a separate Nuxt.js application
- **Scraping and analysis are completely decoupled** for better testing and reliability

## Implementation Status

### ✅ Completed - RawCard System
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
6. **Testing**: Complete end-to-end validation from scraping to final card data

### 🎯 Current Focus
- Code quality improvements (Clippy warnings)
- Feature detection enhancement in analyzer
- Performance optimization for large datasets

## Testing
```bash
# Run Rust tests
cargo test

# Test scraper with small dataset
cargo run --release -p wxdb-scraper -- booster WX24-P1

# Test analyzer on recent data
cargo run -p analyzer -- --limit 10 --verbose

# Check data integrity
python manage.py shell -c "from wix.models import RawCard, Card; print(f'RawCard: {RawCard.objects.count()}, Card: {Card.objects.count()}')"  # From table_definition/ using .venv
```