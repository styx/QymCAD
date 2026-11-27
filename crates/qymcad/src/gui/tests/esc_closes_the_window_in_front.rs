//! ESC CLOSES THE WINDOW IN FRONT, whichever it is and whatever it shows, and goes no further: the tool in hand behind
//! it keeps its place. Reported behaviour: with a line in hand, Esc on Settings -> General put the line down and kept
//! the settings, while on Settings -> Keyboard it closed the settings and kept the line - one key, two rules, chosen by
//! the section shown.
use crate::gui::hand::Hand;
use crate::gui::tests::InEnglish;
use crate::gui::App;
use crate::i18n::tr;
use egui::Key;

fn line_in_hand(hand: &mut Hand) -> bool {
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let sketch = hand.press_word(&tr("plane-xy-table"), egui::Pos2::ZERO);
    hand.key(Key::L);
    sketch && hand.app.tools.armed.drawing() == Some(qymcad_ui_state::SketchShape::Line)
}

#[test]
fn esc_on_the_general_settings_closes_them_and_keeps_the_line() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let general = hand.press_word(&tr("menu-windows"), egui::Pos2::ZERO) && hand.press_word(&tr("menu-settings"), egui::Pos2::ZERO) && hand.press_word(&tr("settings-sec-general"), egui::Pos2::ZERO);
    let shown = hand.shows(&tr("settings-language"));
    hand.key(Key::Escape);
    let (open, held) = (hand.shows(&tr("settings-language")), hand.app.tools.armed.drawing());
    assert!(line && general && shown, "setup: (line: {line}, general opened: {general}, shown: {shown})");
    assert!(!open, "Esc on Settings -> General left the settings open");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc on Settings -> General reached the line in hand behind the settings");
}

#[test]
fn esc_on_the_parameters_closes_them_and_keeps_the_line() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let opened = hand.press_word(&tr("wb-params"), egui::Pos2::ZERO) && hand.shows(&tr("par-hint"));
    hand.key(Key::Escape);
    let (open, held) = (hand.shows(&tr("par-hint")), hand.app.tools.armed.drawing());
    assert!(line && opened, "setup: (line: {line}, parameters opened: {opened})");
    assert!(!open, "Esc on the parameters left them open");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc on the parameters reached the line in hand behind them");
}
