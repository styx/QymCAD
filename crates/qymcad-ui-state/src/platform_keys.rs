//! THE KEYS EACH SYSTEM KEEPS FOR ITSELF - one table per system, so a new rule for one of them is a row in its
//! table rather than a condition threaded through the rebinding code.
//!
//! What differs is what happens to a binding's Ctrl (Ctrl on Linux and Windows, Cmd on a Mac) BEFORE the
//! program hears it:
//!
//! * on Linux and Windows the Ctrl key IS the binding's Ctrl, and a text field edits with some of its letters
//!   (egui erases with Ctrl+H, Ctrl+U and Ctrl+W, Shift or not): a tool there would run AND eat a word of the
//!   expression it was pressed in;
//! * on a Mac the binding's Ctrl is Cmd, which a text field does not edit with. The window does take some: the
//!   application menu winit installs hides the program on Cmd+H and quits it on Cmd+Q. (Its Option+Cmd+H needs
//!   Alt, which no binding has.)
//!
//! What does NOT differ stays beside the bindings: the General letters (`GENERAL_CTRL_KEYS`) are the program's
//! own handlers on every system, and the Mac's Ctrl key - reported by egui as Ctrl without Cmd, which no other
//! system produces - is answered in `pressed_chord`.

use crate::Chord;

/// The system the program runs on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Os {
    Linux,
    Windows,
    Mac,
}

impl Os {
    /// Every system the program is built for - a table must exist for each.
    pub const ALL: [Os; 3] = [Os::Linux, Os::Windows, Os::Mac];

    pub const fn current() -> Os {
        if cfg!(target_os = "macos") {
            Os::Mac
        } else if cfg!(target_os = "windows") {
            Os::Windows
        } else {
            Os::Linux
        }
    }
}

/// One letter a system keeps under the binding's Ctrl.
pub struct Kept {
    pub key: egui::Key,
    /// Kept with Shift as well: a field's erasing ignores Shift, a Mac menu item answers to its exact chord.
    pub with_shift: bool,
    /// The catalogue key of the reason shown in the window.
    pub why: &'static str,
}

/// THE RULES OF ONE SYSTEM.
pub struct PlatformKeys {
    pub os: Os,
    /// The letters under the binding's Ctrl that the system, the window or a text field takes first.
    pub ctrl_kept: &'static [Kept],
}

const FIELD_EDITS: &str = "hotkeys-field-edits";

/// egui's text field erases under the Ctrl key: H a character, U the line before the caret, W the word before it
/// (K, the line after it, is the search's already - a General letter).
const FIELD_ERASERS: [Kept; 3] = [
    Kept { key: egui::Key::H, with_shift: true, why: FIELD_EDITS },
    Kept { key: egui::Key::U, with_shift: true, why: FIELD_EDITS },
    Kept { key: egui::Key::W, with_shift: true, why: FIELD_EDITS },
];

pub const LINUX: PlatformKeys = PlatformKeys { os: Os::Linux, ctrl_kept: &FIELD_ERASERS };

/// The same as Linux today - and a table of its own, so a Windows rule never has to be a Linux one too.
pub const WINDOWS: PlatformKeys = PlatformKeys { os: Os::Windows, ctrl_kept: &FIELD_ERASERS };

pub const MAC: PlatformKeys = PlatformKeys {
    os: Os::Mac,
    ctrl_kept: &[
        Kept { key: egui::Key::H, with_shift: false, why: "hotkeys-os-hide" },
        Kept { key: egui::Key::Q, with_shift: false, why: "hotkeys-os-quit" },
    ],
};

pub fn platform_keys(os: Os) -> &'static PlatformKeys {
    match os {
        Os::Linux => &LINUX,
        Os::Windows => &WINDOWS,
        Os::Mac => &MAC,
    }
}

impl PlatformKeys {
    /// Why this system will not let a chord reach a tool - a catalogue key - or `None`.
    pub fn refusal(&self, chord: &Chord) -> Option<&'static str> {
        if !chord.ctrl {
            return None;
        }
        self.ctrl_kept.iter().find(|k| k.key == chord.key && (k.with_shift || !chord.shift)).map(|k| k.why)
    }
}
