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
cargo make manage makemigrations

# Apply migrations
cargo make manage migrate

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
cargo make manage runserver 0.0.0.0:8003
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

### Frontend
- `front/` - Nuxt.js 3 application
- `front/pages/` - Page components
- `front/components/` - Reusable Vue components
- `front/stores/` - Pinia stores

### Tools
- `syncdb/` - Converts Django models to Rust structs
- `datapack/` - WebAssembly module
- `shared/injector/` - Data injection/scraping tools

## Development Workflow

1. **Model Changes**: Always start with Django models in `table_definition/wix/models.py`
2. **Generate Migrations**: Run `cargo make manage makemigrations`
3. **Apply Migrations**: Run `cargo make manage migrate`
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

## Testing
```bash
# Run Rust tests
cargo test

# Run specific examples
cargo run --example parse_signi
```

## Common Tasks

### Adding a New API Endpoint
1. Add route handler in `shared/webapp/src/routers/`
2. Register in router configuration
3. Add corresponding frontend API call

### Modifying Card Data Structure
1. Update Django model in `table_definition/wix/models.py`
2. Run migration commands
3. Run `cargo make syncdb`
4. Update any affected Rust code
5. Update frontend if needed

## Notes
- The project uses a unique Django-Rust hybrid architecture
- Django handles all database migrations and schema
- Rust provides the API and business logic
- Frontend is a separate Nuxt.js application
- All card-related static data is scraped and cached