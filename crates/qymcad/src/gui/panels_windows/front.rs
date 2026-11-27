//! THE WINDOW IN FRONT and the Esc it is owed.
use super::*;

/// THE WINDOW IN FRONT (`qymcad_shell::front_window`), when no menu, context menu or list stands open over the windows
/// and the window is not folded to its title: what is open over them reads its own keys, and a folded window was put
/// out of the way of the work behind it. Read from the layers of the frame before, the one the person saw when
/// pressing.
pub(crate) fn in_front(ctx: &egui::Context) -> Option<egui::Id> {
    let over = ctx.memory(|m| m.areas().visible_layer_ids().iter().any(|l| l.order == egui::Order::Foreground));
    // egui 0.35 keeps the folding of a window under its id `.with("collapsing")` (containers/window.rs:640)
    let folded = |id: &egui::Id| egui::collapsing_header::CollapsingState::load(ctx, id.with("collapsing")).is_some_and(|s| !s.is_open());
    qymcad_shell::front_window(ctx).filter(|id| !over && !folded(id))
}

/// ESC CLOSES THE WINDOW IN FRONT, whichever it is, and is taken off the frame: the tool in hand behind the window and
/// the selection never see it. A field typing keeps it (it gives the keyboard up first), and so does a menu open over
/// the windows. Over the table of keys an open clash is answered "keep as it was" first, and the settings stay.
///
/// Run at the top of the frame, before anything reads the keys; Esc pressed is asked first, so the layers are walked
/// only on the frame Esc came. A window that is no record of `Windows` (a question asked once) keeps its own Esc.
/// Answers whether Esc was a window's.
pub(crate) fn esc_closes_the_front_window(win: &mut qymcad_ui_state::Windows, hk: &mut qymcad_ui_state::HotkeyCapture, ctx: &egui::Context) -> bool {
    if !ctx.input(|i| i.key_pressed(egui::Key::Escape)) || ctx.egui_wants_keyboard_input() {
        return false;
    }
    let Some(front) = in_front(ctx) else { return false };
    let settings = front == WinKind::Settings.id();
    if settings && hk.clash.take().is_some() {
        return ctx.input_mut(crate::gui::hotkeys::consume_esc);
    }
    if !win.close_window(front) {
        return false;
    }
    if settings {
        hk.settle();
    }
    ctx.input_mut(crate::gui::hotkeys::consume_esc)
}
