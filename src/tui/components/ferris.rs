//! Ferris, Rust's crab mascot, as animated ASCII art.
//!
//! Ferris scuttles across the empty chat on the welcome screen and while the
//! model works on a reply. Frames are a pure function of the app's animation
//! tick (one every 100ms), so the art needs no state of its own.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Rust's brand orange.
pub const FERRIS_ORANGE: Color = Color::Rgb(247, 76, 0);

/// Columns every frame takes; rows are padded to it so Ferris never jitters.
pub const WIDTH: usize = 15;

/// Rows every frame takes.
pub const HEIGHT: usize = 4;

/// Ferris's expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    /// Waiting for a request: a calm face.
    Idle,
    /// Working on a reply: muttering to itself.
    Thinking,
}

/// One frame of Ferris at animation tick `tick`: [`HEIGHT`] rows of
/// [`WIDTH`] columns. The claws wave, the legs step, and Ferris blinks every
/// three seconds.
pub fn frame(tick: usize, mood: Mood) -> [String; HEIGHT] {
    let (left_claw, right_claw) = if (tick / 3) % 2 == 0 {
        ("\\)", "(/")
    } else {
        ("/)", "(\\")
    };
    let eyes = if tick % 30 == 0 { "- -" } else { "o o" };
    let mouth = match mood {
        Mood::Idle => '-',
        Mood::Thinking if (tick / 4) % 2 == 0 => 'o',
        Mood::Thinking => '-',
    };
    let legs = if (tick / 2) % 2 == 0 {
        "/ '-----' \\"
    } else {
        "| '-----' |"
    };

    [
        "    _~^~^~_    ".to_string(),
        format!("{left_claw} /  {eyes}  \\ {right_claw}"),
        format!("  '_   {mouth}   _'  "),
        format!("  {legs}  "),
    ]
}

/// How far from the left of a `lane` columns wide Ferris stands at `tick`:
/// it walks to the right edge and back, one column every two ticks.
pub fn lane_offset(tick: usize, lane: usize) -> usize {
    let travel = lane.saturating_sub(WIDTH);
    if travel == 0 {
        return 0;
    }
    let position = (tick / 2) % (2 * travel);
    if position > travel {
        2 * travel - position
    } else {
        position
    }
}

/// Ferris walking in a lane `lane` columns wide, as styled lines that are all
/// exactly `lane` columns, so centering them keeps the lane in place.
pub fn walking_lines(tick: usize, lane: usize, mood: Mood) -> Vec<Line<'static>> {
    let lane = lane.max(WIDTH);
    let offset = lane_offset(tick, lane);
    let style = Style::default()
        .fg(FERRIS_ORANGE)
        .add_modifier(Modifier::BOLD);
    frame(tick, mood)
        .into_iter()
        .map(|row| {
            let padded = format!(
                "{}{row}{}",
                " ".repeat(offset),
                " ".repeat(lane - offset - WIDTH)
            );
            Line::from(Span::styled(padded, style))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(line: &Line<'_>) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn every_frame_row_is_the_same_width() {
        for tick in 0..120 {
            for mood in [Mood::Idle, Mood::Thinking] {
                for row in frame(tick, mood) {
                    assert_eq!(row.chars().count(), WIDTH, "tick {tick}: {row:?}");
                }
            }
        }
    }

    #[test]
    fn ferris_blinks_and_waves() {
        assert!(frame(0, Mood::Idle)[1].contains("- -"));
        assert!(frame(1, Mood::Idle)[1].contains("o o"));
        assert_ne!(frame(0, Mood::Idle)[1], frame(3, Mood::Idle)[1]);
    }

    #[test]
    fn ferris_walks_to_the_edge_and_back() {
        let lane = WIDTH + 5;
        let offsets: Vec<usize> = (0..24).step_by(2).map(|t| lane_offset(t, lane)).collect();
        assert_eq!(offsets, [0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0, 1]);
        assert_eq!(lane_offset(7, WIDTH), 0, "no room to walk");
    }

    #[test]
    fn walking_lines_fill_the_lane_exactly() {
        for tick in 0..60 {
            let lines = walking_lines(tick, 30, Mood::Thinking);
            assert_eq!(lines.len(), HEIGHT);
            for line in &lines {
                assert_eq!(text(line).chars().count(), 30);
            }
        }
        // A lane narrower than Ferris is widened to fit it.
        assert_eq!(
            text(&walking_lines(0, 4, Mood::Idle)[0]).chars().count(),
            WIDTH
        );
    }
}
