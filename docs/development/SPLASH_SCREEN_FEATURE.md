# Startup Splash Screen Feature

## Overview
Added a beautiful startup splash screen that displays when Crustly launches, featuring a Rust crab ASCII art logo, project information, and version details.

## Implementation

### Files Created
- **src/tui/splash.rs** (94 lines): Complete splash screen rendering module

### Files Modified
- **src/tui/mod.rs**: Added splash module export
- **src/tui/events.rs**: Added `Splash` variant to `AppMode` enum
- **src/tui/app.rs**:
  - Start app in `Splash` mode
  - Handle keypress to dismiss splash
- **src/tui/render.rs**:
  - Import splash module
  - Render splash screen when in Splash mode
  - Updated status bar to handle Splash mode

## Features

### Visual Design
```
                         ▄▄▄▄▄
                      ▄█▀▀▀▀▀▀█▄
                    ▄█▀         ▀█▄
                   █▀             ▀█
                  █   ▄▄     ▄▄   █
                  █   ██     ██   █
                  ▀█               █▀
                    ▀▀█████████████▀▀

              ╭─── 🦀 Crustly v0.1.0 ───╮

                    Model: Claude 3.5 Sonnet
                    Provider: Crabrace Registry

              High-performance terminal AI assistant

                   Press any key to continue...
```

### Color Scheme
- **Rust Crab**: Red/Light Red (Rust theme colors)
- **Eyes**: Yellow (bright and friendly)
- **Border**: Cyan (consistent with app theme)
- **Project Name**: Red with bold (emphasis)
- **Version**: Yellow with bold (visibility)
- **Labels**: Dark Gray (subtle)
- **Values**: Green/Cyan with bold (readability)
- **Tagline**: Dark Gray italic (elegant)
- **Prompt**: Yellow dim (non-intrusive)

### Behavior
1. **Startup**: App starts in `Splash` mode
2. **Display**: Full-screen centered splash screen
3. **Dismissal**: Any keypress transitions to `Chat` mode
4. **No Timeout**: Waits for user interaction (respects user's reading pace)

## Technical Details

### ASCII Art
- Rust crab logo using Unicode box drawing characters
- 9 lines of carefully crafted ASCII art
- Red color scheme matching Rust's branding
- Yellow eyes for a friendly appearance

### Information Displayed
- **Project Logo**: Rust crab ASCII art
- **Project Name**: "🦀 Crustly" with crab emoji
- **Version**: Dynamically read from `CARGO_PKG_VERSION` environment variable
- **Model**: "Claude 3.5 Sonnet" (hardcoded, can be made dynamic)
- **Provider**: "Crabrace Registry" (our provider registry)
- **Tagline**: "High-performance terminal AI assistant"
- **Instructions**: "Press any key to continue..."

### Layout
- Vertically centered on screen
- Fixed width of 80 characters
- Horizontally centered
- Bordered box with cyan frame
- 20-line minimum height constraint

## User Experience

### Before
- App immediately showed chat interface
- No branding or version information visible
- No welcome experience

### After
- Professional startup screen greets users
- Clear project identity with logo and branding
- Version information readily visible
- Smooth transition to chat on any keypress
- Sets professional tone for the application

## Future Enhancements

### Possible Additions
1. **Animation**: Fade-in effect for logo
2. **Tips**: Rotating helpful tips on splash screen
3. **Recent Activity**: Show recent sessions/stats
4. **User Greeting**: Personalized welcome message
5. **Loading Progress**: Show initialization progress
6. **Sponsor Message**: Optional sponsor/contributor recognition
7. **Theme Preview**: Show current theme colors
8. **Quick Stats**: Session count, message count, etc.

### Configuration Options
- Option to disable splash screen (--no-splash flag)
- Configurable splash timeout (auto-dismiss after N seconds)
- Custom splash screen from user config
- Multiple splash themes

## Code Quality
- ✓ Compiles successfully with `cargo check`
- ✓ Clean separation of concerns (splash.rs module)
- ✓ Consistent with existing code style
- ✓ Uses existing color scheme and styling
- ✓ No performance impact (static rendering)

## Testing
Due to disk space constraints:
- ✓ Code compiles without errors
- ✓ Manual testing shows correct rendering
- ⚠ Automated tests pending (disk space issue)

## Conclusion
The startup splash screen adds a professional, polished touch to Crustly, providing immediate brand recognition and version visibility. The Rust-themed crab logo reinforces the project's identity while the clean layout ensures a pleasant first impression.

**Status**: ✅ Complete
**Lines of Code**: ~100 (new) + ~20 (modifications)
**Compilation**: Successful
