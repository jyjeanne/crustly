# Windows Build Fix Guide

**Issue:** `dlltool.exe not found` error when building on Windows
**Status:** 🔧 In Progress
**Date:** October 28, 2025

---

## Current Situation

You've successfully switched to the GNU toolchain:
```powershell
rustup default stable-x86_64-pc-windows-gnu
```

**Result:** ✅ Toolchain switched
**But:** ❌ Still need MinGW-w64 tools

---

## The Problem

The Rust GNU toolchain requires MinGW-w64 tools (gcc, dlltool, etc.) to be installed separately. Switching the toolchain alone is not enough.

---

## Solution: Install MinGW-w64

### Option 1: Using MSYS2 (Recommended)

**Step 1: Install MSYS2**
1. Download from: https://www.msys2.org/
2. Run the installer (msys2-x86_64-*.exe)
3. Install to default location: `C:\msys64`
4. Complete installation

**Step 2: Open MSYS2 Terminal**
1. Start Menu → "MSYS2 MSYS"
2. Run update command:
```bash
pacman -Syu
```
3. Close terminal when prompted, reopen

**Step 3: Install MinGW-w64 Toolchain**
```bash
pacman -S mingw-w64-x86_64-toolchain
```
Press Enter to install all packages (about 400MB)

**Step 4: Add to Windows PATH**
1. Press `Win + X` → System
2. Click "Advanced system settings"
3. Click "Environment Variables"
4. Under "User variables", find "Path"
5. Click "Edit"
6. Click "New"
7. Add: `C:\msys64\mingw64\bin`
8. Click OK on all dialogs

**Step 5: Verify Installation**
```powershell
# Open NEW PowerShell (important!)
where gcc
where dlltool
gcc --version
```

Expected output:
```
C:\msys64\mingw64\bin\gcc.exe
C:\msys64\mingw64\bin\dlltool.exe
gcc.exe (Rev10, Built by MSYS2 project) 13.x.x
```

**Step 6: Build Crustly**
```powershell
cd C:\Users\jjeanne\Documents\Perso\Projects\Crusty-cli\crustly
cargo clean
cargo check
```

### Option 2: Standalone MinGW-w64

**Download:**
- https://github.com/niXman/mingw-builds-binaries/releases
- Choose: `x86_64-*-release-posix-seh-ucrt-*.7z`
- Extract to: `C:\mingw64`

**Add to PATH:**
- Add `C:\mingw64\bin` to PATH (same steps as above)

**Verify:**
```powershell
where gcc
where dlltool
```

### Option 3: Switch Back to MSVC with WSL2

If MinGW installation is problematic:

**Install WSL2:**
```powershell
# Run as Administrator
wsl --install
```

**Install Rust in WSL2:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Build in WSL2:**
```bash
cd /mnt/c/Users/jjeanne/Documents/Perso/Projects/Crusty-cli/crustly
cargo check
```

---

## Quick Verification Commands

After installing MinGW-w64:

```powershell
# Check PATH
$env:PATH -split ';' | Select-String mingw

# Check tools
where gcc
where dlltool
where g++

# Check versions
gcc --version
dlltool --version

# Test Rust build
cd crustly
cargo clean
cargo check
```

---

## Troubleshooting

### "gcc not found" after installation
**Solution:** Open a NEW PowerShell/Terminal window
- PATH changes require new terminal session

### "wrong ELF class" error
**Solution:** Make sure you installed 64-bit tools
- Look for `x86_64` in the package name
- Check: `gcc -dumpmachine` should output `x86_64-w64-mingw32`

### Multiple MinGW installations conflict
**Solution:** Check PATH order
```powershell
$env:PATH -split ';' | Select-String mingw
```
- MSYS2 path should be first
- Remove other MinGW paths if conflicts occur

### Still getting dlltool error
**Solution:** Clean cargo cache
```powershell
cargo clean
Remove-Item -Recurse -Force ~/.cargo/registry/cache
cargo check
```

---

## Why This Happens

### Rust on Windows: Two Toolchains

**MSVC (default):**
- Uses Microsoft's Visual Studio tools
- Native Windows toolchain
- **Issue:** Some crates need MinGW tools anyway

**GNU (MinGW):**
- Uses GCC-based MinGW-w64
- POSIX-compatible on Windows
- **Requires:** Separate MinGW-w64 installation

### The Catch

Rust GNU toolchain does NOT include MinGW tools!
- Only provides Rust compiler
- Expects MinGW tools (gcc, dlltool) to be installed
- Must install separately via MSYS2 or standalone

---

## Current Status Checklist

- [x] Switched to GNU toolchain (`rustup default stable-x86_64-pc-windows-gnu`)
- [ ] Installed MinGW-w64 tools
- [ ] Added MinGW to PATH
- [ ] Verified gcc/dlltool work
- [ ] Tested `cargo check`
- [ ] Tested `cargo test`
- [ ] Tested `cargo clippy`

---

## Next Steps

### 1. Install MSYS2 (5-10 minutes)
```powershell
# Download and install MSYS2
# URL: https://www.msys2.org/
```

### 2. Install MinGW toolchain (5-10 minutes)
```bash
# In MSYS2 terminal
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain
```

### 3. Update PATH (1 minute)
- Add `C:\msys64\mingw64\bin` to PATH

### 4. Test build (2-5 minutes)
```powershell
# New PowerShell window
cd crustly
cargo check
```

---

## Alternative: Stay with MSVC

If you prefer to avoid MinGW setup:

```powershell
# Switch back to MSVC
rustup default stable-x86_64-pc-windows-msvc
```

**Then use WSL2 for development:**
- Windows: Use for editing files
- WSL2: Use for building/testing

**Pros:**
- No MinGW installation needed
- True Linux environment
- Better compatibility

**Cons:**
- Requires WSL2 setup
- Cross-filesystem access (slightly slower)

---

## Expected Timeline

### Fast Path (MSYS2)
- Download MSYS2: 2 minutes
- Install MSYS2: 3 minutes
- Install MinGW: 5 minutes
- Update PATH: 1 minute
- Test build: 5 minutes
- **Total: ~15-20 minutes**

### Alternative Path (WSL2)
- Enable WSL2: 5 minutes
- Install Ubuntu: 10 minutes
- Install Rust in WSL2: 5 minutes
- Test build: 5 minutes
- **Total: ~25 minutes**

---

## Success Criteria

You'll know it works when:

```powershell
PS> where dlltool
C:\msys64\mingw64\bin\dlltool.exe

PS> cargo check
   Compiling crustly v0.1.0
   ...
    Finished dev [unoptimized + debuginfo] target(s) in 2m 15s
```

---

## Help Resources

- **MSYS2 Documentation:** https://www.msys2.org/docs/what-is-msys2/
- **Rust Windows FAQ:** https://github.com/rust-lang/rustup/blob/master/README.md#working-with-rust-on-windows
- **MinGW-w64:** https://www.mingw-w64.org/

---

## Summary

**Current State:**
- ✅ GNU toolchain active
- ❌ MinGW tools missing

**Required Action:**
- Install MSYS2 + MinGW-w64 toolchain
- Add `C:\msys64\mingw64\bin` to PATH

**Expected Result:**
- ✅ `cargo check` succeeds
- ✅ `cargo test` runs
- ✅ `cargo clippy` works

---

**Status:** Ready to install MinGW-w64 🔧

**ETA:** 15-20 minutes to full working build

---

**Need Help?** See BUILD_NOTES.md for additional troubleshooting options.
