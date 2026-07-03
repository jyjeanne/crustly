//! TUI Runner
//!
//! Main event loop and terminal setup for the TUI.

use super::app::App;
use super::events::EventHandler;
use super::render;
use anyhow::Result;
use crossterm::{
    event::{
        DisableBracketedPaste, EnableBracketedPaste, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

/// Run the TUI application
pub async fn run(mut app: App) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;

    // Query whether the terminal supports the Kitty keyboard protocol
    // (needed to disambiguate Shift+Enter from plain Enter). Must run after
    // raw mode is enabled since it reads a synchronous response from stdin.
    // Run on a blocking-pool thread with an explicit timeout: on a
    // terminal/multiplexer that never answers the query, this must not
    // hang startup or block the async runtime thread indefinitely.
    let kitty_keyboard_supported = match tokio::time::timeout(
        std::time::Duration::from_millis(1000),
        tokio::task::spawn_blocking(supports_keyboard_enhancement),
    )
    .await
    {
        Ok(Ok(Ok(supported))) => supported,
        _ => false,
    };
    if kitty_keyboard_supported {
        // Fall back to no keyboard enhancement on failure rather than
        // propagating the error - bailing out here would leave the
        // terminal in raw/alternate-screen mode with no cleanup.
        if let Err(e) = execute!(
            stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        ) {
            tracing::warn!("Failed to enable Kitty keyboard protocol: {e}");
        }
    }
    app.set_kitty_keyboard_protocol_active(kitty_keyboard_supported);

    // Run everything else and capture the result instead of using `?`, so
    // the terminal is always restored below regardless of whether setup
    // (Terminal::new, app.initialize()) or the main loop itself fails.
    let result = run_inner(stdout, &mut app).await;

    // Restore terminal unconditionally.
    if kitty_keyboard_supported {
        let _ = execute!(io::stdout(), PopKeyboardEnhancementFlags);
    }
    let _ = disable_raw_mode();
    let _ = execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableBracketedPaste,
        crossterm::cursor::Show
    );

    result
}

/// Create the terminal, initialize the app, and run the main loop. Split out
/// from `run()` so terminal state can always be restored by the caller
/// regardless of whether this succeeds or fails partway through.
async fn run_inner(stdout: io::Stdout, app: &mut App) -> Result<()> {
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    app.initialize().await?;

    let event_sender = app.event_sender();
    EventHandler::start_terminal_listener(event_sender);

    run_loop(&mut terminal, app).await
}

/// Main event loop
async fn run_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| render::render(f, app))?;

        // Check for quit
        if app.should_quit {
            break;
        }

        // Wait for next event with timeout
        let event =
            tokio::time::timeout(tokio::time::Duration::from_millis(100), app.next_event()).await;

        // Handle event if received
        if let Ok(Some(event)) = event {
            if let Err(e) = app.handle_event(event).await {
                // Show error in UI
                app.error_message = Some(e.to_string());
            }
        }
    }

    Ok(())
}
