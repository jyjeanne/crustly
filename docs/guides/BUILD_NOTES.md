# Build Notes for Crustly

## Current Status (Sprint 0)

✅ **Sprint 0 Complete** - Project initialization and foundational structure

## Known Build Issues

### Windows: dlltool.exe Not Found

**Error:**
```
error: Error calling dlltool 'dlltool.exe': program not found
```

**Cause:**
This is a known issue with the Rust MSVC toolchain on Windows when certain crates need MinGW tools.

**Solutions:**

#### Option 1: Install MinGW-w64 (Recommended)
```bash
# Using MSYS2
pacman -S mingw-w64-x86_64-toolchain

# Or download from: https://www.mingw-w64.org/
```

#### Option 2: Use WSL2 (Linux subsystem)
```bash
# In WSL2 Ubuntu/Debian:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd /mnt/c/Users/yourname/Documents/Perso/Projects/Crusty-cli/crustly
cargo build
```

#### Option 3: Use Linux/macOS
The project compiles without issues on Linux and macOS.

#### Option 4: Use Rust GNU toolchain instead of MSVC
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
cargo build
```

## Dependency Notes

### SQLite Conflict Resolution

We removed `rusqlite` from dependencies to avoid `libsqlite3-sys` version conflicts with `sqlx`.

**Original Issue:**
- `rusqlite 0.31` uses `libsqlite3-sys 0.28`
- `sqlx 0.7` uses `libsqlite3-sys 0.26`
- Cargo doesn't allow multiple versions linking to the same native library

**Solution:**
- Using only `sqlx` for database operations
- Migrations will use `sqlx-cli` instead of `refinery`

## Sprint 0 Achievements

✅ **Project Structure:**
- 30+ Rust source files created
- 17 module directories scaffolded
- Complete module hierarchy

✅ **Core Implementation:**
- Error types with error codes
- CLI with Clap v4 (5 commands)
- Configuration system with Crabrace integration
- Logging setup with tracing
- Basic application structure

✅ **Development Setup:**
- Cargo.toml configured (40+ dependencies)
- .gitignore created
- .rustfmt.toml formatting rules
- Module stubs for all planned features

✅ **Documentation:**
- This BUILD_NOTES.md
- Inline code documentation
- Module-level docs for all stubs

## Next Steps (Sprint 1)

Once build issues are resolved:

1. **Sprint 1: Database Layer**
   - Implement SQLx connection pool
   - Create database schema
   - Add migrations
   - Implement repository pattern

2. **Sprint 2: Configuration**
   - Config file loading (TOML)
   - Environment variable support
   - Config validation
   - Crabrace client integration

3. **Verify Build:**
   ```bash
   cargo check    # Type checking
   cargo test     # Run tests
   cargo build    # Full build
   cargo run      # Test CLI
   ```

## Testing the CLI (When Built)

```bash
# Show help
crustly --help

# Show version
crustly version

# Interactive mode (default)
crustly

# Run a prompt
crustly run "hello world"

# List sessions
crustly sessions
```

## Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Startup Time | < 50ms | 🔜 To measure |
| Memory (idle) | < 25MB | 🔜 To measure |
| Binary Size | < 15MB | 🔜 To measure |

---

**Sprint 0 Status:** ✅ COMPLETE (pending Windows build environment setup)
**Next Sprint:** Sprint 1 - Database Layer
**Timeline:** 18 weeks total (currently Week 1)
