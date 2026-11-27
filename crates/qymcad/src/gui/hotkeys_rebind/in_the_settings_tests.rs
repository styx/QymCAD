use egui::Key;
use crate::gui::hand::Hand;
use crate::gui::App;
use crate::i18n::tr;
use crate::gui::tests::InEnglish;

fn words(hand: &mut Hand) -> Vec<String> {
    hand.frame(Vec::new());
    hand.words_drawn().iter().map(|(t, _)| t.clone()).collect()
}

fn extrude_row() -> String {
    crate::gui::hotkeys::hotkey_what(qymcad_ui_state::HOTKEYS.iter().find(|r| r.action == "part.extrude").expect("the extrude row"))
}

#[test]
fn the_start_screen_opens_the_keys_in_the_settings() {
    let _en = InEnglish::new();
    let mut plain = App::default();
    let mut hand = Hand::new(&mut plain);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let keys = tr("settings-hotkeys");
    let before = words(&mut hand).iter().any(|w| w.contains(&keys));
    let help = hand.press_word(&tr("menu-help"), egui::Pos2::ZERO);
    let in_help: Vec<String> = words(&mut hand).into_iter().filter(|w| w.contains(&keys)).collect();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let pressed = hand.press_word(&tr("start-hotkeys"), egui::Pos2::ZERO);
    let (table, row) = (hand.shows(&tr("settings-sec-keyboard")) && hand.shows(&keys), hand.shows(&extrude_row()));
    assert!(!before && help && in_help.is_empty(), "the Help menu still offers the keys: {in_help:?} (opened: {help}, shown before: {before})");
    assert!(pressed, "the start screen has no keyboard shortcuts button");
    assert!(table && row, "the start screen's button did not show the table in the settings (caption: {table}, extrude row: {row})");
}

#[test]
fn esc_answers_a_clash_and_keeps_the_settings() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = press_keyboard_settings(&mut hand);
    let row = hand.written_at(&extrude_row());
    let waits = row.is_some_and(|r| hand.press_word("E", r.center()));
    hand.key(Key::F);
    let asked = hand.shows(&tr("hotkeys-swap"));
    hand.key(Key::Escape);
    let (still_asked, open) = (hand.shows(&tr("hotkeys-swap")), hand.shows(&tr("settings-hotkeys")));
    let key = qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude");
    assert!(opened && waits, "the extrude key was not reached in Settings -> Keyboard (opened: {opened}, row: {row:?})");
    assert!(asked, "F is the fillet's and the table asked nothing");
    assert!(!still_asked && key == "E", "Esc did not answer the clash with the old key (question left: {still_asked}, key now {key:?})");
    assert!(open, "Esc answered the clash and closed the settings too");
}

use super::super::hotkeys::press_keyboard_settings;

fn wait_on_the_extrusion(hand: &mut Hand) -> bool {
    let row = hand.written_at(&extrude_row());
    row.is_some_and(|r| hand.press_word("E", r.center())) && hand.shows(&tr("hotkeys-press"))
}

#[test]
fn the_settings_closed_from_the_menu_let_the_keyboard_go() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let closed = hand.press_word(&tr("menu-windows"), egui::Pos2::ZERO) && hand.press_word(&tr("menu-settings"), egui::Pos2::ZERO) && !hand.shows(&extrude_row());
    hand.frame(Vec::new());
    let held = hand.app.hotkeys.action.clone();
    let reopened = press_keyboard_settings(&mut hand);
    let still_waits = hand.shows(&tr("hotkeys-press"));
    assert!(waits && closed && reopened, "setup: the table did not wait, or the menu did not close and open the settings (waits: {waits}, closed: {closed}, reopened: {reopened})");
    assert!(held.is_none() && !still_waits, "the settings closed by the menu left the table waiting for a key for {held:?} (waits when opened again: {still_waits})");
}

#[test]
fn a_search_that_hides_the_table_drops_its_question() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    hand.key(Key::F);
    let asked = hand.shows(&tr("hotkeys-swap"));
    let typed = hand.press_word(&tr("settings-search"), egui::Pos2::ZERO);
    hand.type_text("grid").frame(Vec::new());
    let hidden = !hand.shows(&extrude_row());
    hand.frame(Vec::new());
    let left = hand.app.hotkeys.clash.clone();
    assert!(waits && asked && typed && hidden, "setup: no clash asked, or the search did not hide the table (waits: {waits}, asked: {asked}, typed: {typed}, hidden: {hidden})");
    assert!(left.is_none(), "the search hid the table and its question was left: {left:?}");
}

/// Reported behaviour: every letter typed into the filter drew the table's grids under new ids; egui threw the
/// pass away to measure them ("new Grid") and drew the frame twice, and kept the state of every grid ever drawn.
#[test]
fn a_letter_typed_into_the_filter_draws_the_table_once() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = press_keyboard_settings(&mut hand) && hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    let passes: Vec<u64> = ["c", "i", "r"].iter().map(|letter| hand.passes_to_type(letter)).collect();
    let filter = hand.app.hotkeys.filter.clone();
    assert!(opened && filter == "cir", "setup: the letters did not reach the filter (opened: {opened}, filter: {filter:?})");
    assert_eq!(passes, [1, 1, 1], "a letter typed into the filter drew the frame more than once");
}

#[test]
fn the_hint_of_the_key_pressed_again_names_the_key_bound_now() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = press_keyboard_settings(&mut hand) && hand.press_word(&tr("settings-again-cycles"), egui::Pos2::ZERO) && hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    hand.type_text("circle").frame(Vec::new());
    let circle = crate::gui::hotkeys::hotkey_what(qymcad_ui_state::HOTKEYS.iter().find(|r| r.action == "sketch.circle").expect("the circle row"));
    let row = hand.written_at(&circle);
    let waits = row.is_some_and(|r| hand.press_word("C", r.center())) && hand.shows(&tr("hotkeys-press"));
    hand.key(Key::J).frame(Vec::new());
    let bound = qymcad_ui_state::hotkey_key(&hand.app.set, "sketch.circle");
    let (named, factory) = (hand.words_drawn().iter().any(|(w, _)| w.contains("J pressed again")), hand.words_drawn().iter().any(|(w, _)| w.contains("C pressed again")));
    assert!(opened && waits && bound == "J", "setup: the circle was not rebound to J in the table (opened: {opened}, waits: {waits}, bound: {bound:?})");
    assert!(named && !factory, "the hint does not name the key of the circle bound now (J named: {named}, C named: {factory})");
}

#[test]
fn the_settings_folded_let_the_keyboard_go() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let title = hand.fold_titled(&tr("win-settings"));
    let folded = hand.window_titled(&tr("win-settings")).is_some_and(|w| w.height() < 60.0);
    hand.frame(Vec::new());
    let held = hand.app.hotkeys.action.clone();
    assert!(waits && folded, "setup: the table did not wait, or the arrow of the title did not fold the settings (waits: {waits}, folded: {folded}, arrow found: {title})");
    assert!(held.is_none(), "the settings folded left the table waiting for a key for {held:?}");
}

#[test]
fn the_section_shown_clicked_again_keeps_the_table_waiting() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let clicked = hand.press_word(&tr("settings-sec-keyboard"), egui::Pos2::ZERO);
    hand.frame(Vec::new());
    let held = hand.app.hotkeys.action.clone();
    assert!(waits && clicked, "setup: the table did not wait, or the Keyboard section was not clicked (waits: {waits}, clicked: {clicked})");
    assert_eq!(held.as_deref(), Some("part.extrude"), "the Keyboard section clicked while shown dropped the waiting of the table");
}

#[test]
fn another_section_clicked_lets_the_keyboard_go() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let clicked = hand.press_word(&tr("settings-sec-general"), egui::Pos2::ZERO);
    // the frame of the click still draws the table; the press after the one that drew General binds nothing
    hand.frame(Vec::new()).key(Key::J);
    let (held, key) = (hand.app.hotkeys.action.clone(), qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude"));
    let back = hand.press_word(&tr("settings-sec-keyboard"), egui::Pos2::ZERO);
    let still_waits = hand.shows(&tr("hotkeys-press"));
    assert!(waits && clicked && back, "setup: the table did not wait, or the sections were not clicked (waits: {waits}, General: {clicked}, Keyboard: {back})");
    assert!(held.is_none() && !still_waits && key == "E", "the General section clicked left the table waiting for a key for {held:?} (waits when back: {still_waits}, extrude key now {key:?})");
}

#[test]
fn the_filter_typed_letter_by_letter_marks_no_widget_red() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = press_keyboard_settings(&mut hand) && hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    for letter in ["c", "i", "r"] {
        hand.type_text(letter).frame(Vec::new());
    }
    let filter = hand.app.hotkeys.filter.clone();
    assert!(opened, "setup: the filter of Settings -> Keyboard was not reached");
    assert_eq!(filter, "cir", "setup: the letters did not reach the filter");
}

#[test]
fn esc_with_the_table_on_screen_closes_the_settings_and_nothing_behind() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let sketch = hand.press_word(&tr("plane-xy-table"), egui::Pos2::ZERO);
    hand.key(Key::L);
    let line = hand.app.tools.armed.drawing() == Some(qymcad_ui_state::SketchShape::Line);
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    hand.key(Key::Escape);
    let (open, held) = (hand.shows(&extrude_row()), hand.app.tools.armed.drawing());
    assert!(sketch && line && opened, "setup: no sketch, no line in hand or no table (sketch: {sketch}, line: {line}, table: {opened})");
    assert!(!open, "Esc with the table on screen left the settings open");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc with the table on screen reached the line in hand behind the settings");
}

/// Reported behaviour: a tool taken AFTER the settings were opened had Esc - its box of fields appeared at the
/// geometry, egui raised it to the top of the windows' layer, and the settings were "not in front". Taken before,
/// the same tool was left alone. The first Esc leaves the field the box focuses; the second is the one asked.
#[test]
fn esc_closes_the_settings_under_a_tool_taken_after_they_were_opened() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let circle = a_finished_circle(&mut hand);
    let opened = circle && press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    hand.key(Key::E);
    let extrude = hand.app.tools.armed.part_tool() == Some(qymcad_ui_state::PartTool::Extrude);
    let boxed = hand.settle().tool_box_id().and_then(|id| hand.area_rect(id)).is_some();
    hand.key(Key::Escape);
    // the row of the extrusion reads "Extrude" as the bar of the tool in hand does: the settings are asked by title
    let settings = tr("win-settings");
    let field_left = hand.window_titled(&settings).is_some() && hand.app.tools.armed.part_tool() == Some(qymcad_ui_state::PartTool::Extrude);
    hand.key(Key::Escape);
    let (open, held) = (hand.window_titled(&settings).is_some(), hand.app.tools.armed.part_tool());
    assert!(opened && extrude && boxed && field_left, "setup: (circle and table: {opened}, extrude: {extrude}, its box at the geometry: {boxed}, the first Esc left only the field: {field_left})");
    assert!(!open, "Esc left the settings open under a tool taken after them");
    assert_eq!(held, Some(qymcad_ui_state::PartTool::Extrude), "Esc reached the extrude taken after the settings were opened");
}

/// A circle R10 drawn on XY from the start screen and the sketch finished and rebuilt, left selected as a finished
/// sketch is.
fn a_finished_circle(hand: &mut Hand) -> bool {
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let sketch = hand.press_word(&tr("plane-xy-table"), egui::Pos2::ZERO);
    hand.sk_tool(qymcad_ui_state::SketchShape::Circle).click2d(0.0, 0.0).click2d(10.0, 0.0).key(Key::Enter);
    let finished = sketch && hand.press_word(&tr("wb-finish"), egui::Pos2::ZERO);
    // the rebuild the finish starts is waited for: its card holds the clicks meant for the menus
    hand.settle();
    finished
}

#[test]
fn the_start_screen_shows_the_table_over_a_search_left_behind() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = hand.press_word(&tr("menu-windows"), egui::Pos2::ZERO) && hand.press_word(&tr("menu-settings"), egui::Pos2::ZERO);
    let typed = hand.press_word(&tr("settings-search"), egui::Pos2::ZERO);
    hand.type_text("scale");
    let closed = hand.press_word(&tr("menu-windows"), egui::Pos2::ZERO) && hand.press_word(&tr("menu-settings"), egui::Pos2::ZERO);
    let pressed = hand.press_word(&tr("start-hotkeys"), egui::Pos2::ZERO);
    let row = hand.shows(&extrude_row());
    assert!(opened && typed && closed && pressed, "setup: (opened: {opened}, typed: {typed}, closed: {closed}, start button: {pressed})");
    assert!(row, "the start screen's button opened the settings over the search left behind, without the table");
}

fn line_in_hand(hand: &mut Hand) -> bool {
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let sketch = hand.press_word(&tr("plane-xy-table"), egui::Pos2::ZERO);
    hand.key(Key::L);
    sketch && hand.app.tools.armed.drawing() == Some(qymcad_ui_state::SketchShape::Line)
}

/// Reported behaviour: with Settings -> Keyboard open and the "save the changes?" question up, Esc closed
/// the settings behind the question and the question stayed. The question stands in front; Esc answers it.
#[test]
fn esc_answers_the_question_in_front_of_the_table() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    hand.click2d(0.0, 0.0).click2d(30.0, 0.0).key(Key::Escape);
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    let asked = hand.press_word(&tr("menu-file"), egui::Pos2::ZERO) && hand.press_word(&tr("file-new"), egui::Pos2::ZERO) && hand.shows(&tr("nav-unsaved-text"));
    hand.key(Key::Escape);
    let (question, table) = (hand.shows(&tr("nav-unsaved-text")), hand.shows(&extrude_row()));
    assert!(line && opened && asked, "setup: (line: {line}, table: {opened}, question: {asked})");
    assert!(!question, "Esc left the question in front of the settings unanswered");
    assert!(table, "Esc with a question in front closed the settings behind it");
}

#[test]
fn esc_over_the_settings_folded_reaches_the_tool_in_hand() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let opened = press_keyboard_settings(&mut hand);
    hand.fold_titled(&tr("win-settings"));
    let folded = hand.window_titled(&tr("win-settings")).is_some_and(|w| w.height() < 60.0);
    hand.key(Key::Escape);
    let held = hand.app.tools.armed.drawing();
    assert!(line && opened && folded, "setup: (line: {line}, opened: {opened}, folded: {folded})");
    assert_ne!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc over the settings folded did not reach the line in hand");
}

/// Reported behaviour: the hint under "pressed again" told of cycling through the modes with "Puts the tool
/// down" chosen.
#[test]
fn the_hint_under_the_choice_tells_of_the_choice_made() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let chosen = press_keyboard_settings(&mut hand) && hand.press_word(&tr("settings-again-puts-down"), egui::Pos2::ZERO);
    let puts_down = hand.app.set.again == qymcad_ui_state::Again::PutsDown;
    let (cycling, said) = (hand.shows(&crate::i18n::tr1("settings-again-hint", "key", "C")), hand.shows(&tr("settings-again-hint-puts-down")));
    assert!(chosen && puts_down, "setup: \"Puts the tool down\" was not chosen (pressed: {chosen}, chosen: {puts_down})");
    assert!(!cycling, "with \"Puts the tool down\" chosen the hint still tells of cycling through the modes");
    assert!(said, "with \"Puts the tool down\" chosen the hint does not say the tool is put down");
}

/// Reported behaviour: with the key of the circle cleared, the hint read "...the polygon:  pressed again..." -
/// a gap where the key stood.
#[test]
fn the_hint_names_no_key_the_circle_has_lost() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let cycles = press_keyboard_settings(&mut hand) && hand.press_word(&tr("settings-again-cycles"), egui::Pos2::ZERO);
    let filtered = hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    hand.type_text("circle");
    let circle = crate::gui::hotkeys::hotkey_what(qymcad_ui_state::HOTKEYS.iter().find(|r| r.action == "sketch.circle").expect("the circle row"));
    let row = hand.written_at(&circle);
    let waits = row.is_some_and(|r| hand.press_word("C", r.center()));
    hand.key(Key::Backspace);
    let cleared = qymcad_ui_state::hotkey_key(&hand.app.set, "sketch.circle").is_empty();
    let gap = hand.words_drawn().iter().any(|(w, _)| w.contains(&crate::i18n::tr1("settings-again-hint", "key", "")));
    let said = hand.shows(&tr("settings-again-hint-unbound"));
    assert!(cycles && filtered && waits && cleared, "setup: (cycles: {cycles}, filter: {filtered}, circle waits: {waits}, cleared: {cleared})");
    assert!(!gap, "the hint shows a gap where the cleared key of the circle stood");
    assert!(said, "with the circle's key cleared the hint does not tell of cycling without a key");
}

/// Reported behaviour: with Settings -> Keyboard open, Esc on the File menu left the menu open and closed the
/// settings behind it.
#[test]
fn esc_on_a_menu_over_the_table_closes_the_menu_and_keeps_the_settings() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    let menu = hand.press_word(&tr("menu-file"), egui::Pos2::ZERO) && hand.shows(&tr("file-new"));
    hand.key(Key::Escape);
    let (still_menu, table) = (hand.shows(&tr("file-new")), hand.shows(&extrude_row()));
    assert!(opened && menu, "setup: (table: {opened}, File menu: {menu})");
    assert!(!still_menu, "Esc left the File menu over the settings open");
    assert!(table, "Esc on the File menu closed the settings behind it");
}

/// Reported behaviour: with Settings -> Keyboard open, Esc on the font window in front of them left the font
/// window open and closed the settings.
#[test]
fn esc_on_a_window_in_front_of_the_table_closes_that_window_and_keeps_the_settings() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let sketch = hand.press_word(&tr("plane-xy-table"), egui::Pos2::ZERO);
    hand.sk_tool(qymcad_ui_state::SketchShape::Text).frame(Vec::new());
    let named = qymcad_ui_state::font_label(&hand.app.tool_prefs.font, "opt-font");
    let fonts = hand.press_word(&named, egui::Pos2::ZERO) && hand.app.font_cache.picker.open;
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    // the settings open over the font window; their title is dragged down until the font window's shows
    let title = hand.window_titled(&tr("win-settings")).map(|w| egui::pos2(w.center().x, w.min.y + 10.0));
    if let Some(grab) = title {
        hand.drag_screen(grab, grab + egui::vec2(0.0, 150.0));
    }
    // the font window is raised by its title bar: "Font" is also written in the box of the text tool, and a press on
    // that word raised the box instead
    let font_title = hand.window_titled(&tr("font-window")).map(|w| egui::pos2(w.center().x, w.min.y + 10.0));
    if let Some(grab) = font_title {
        hand.drag_screen(grab, grab + egui::vec2(10.0, 0.0));
    }
    let raised = font_title.is_some() && hand.window_titled(&tr("win-settings")).is_some_and(|w| w.min.y > 150.0);
    hand.key(Key::Escape);
    let (still_fonts, table) = (hand.app.font_cache.picker.open, hand.shows(&extrude_row()));
    assert!(sketch && fonts && opened && raised, "setup: (sketch: {sketch}, font window: {fonts}, table: {opened}, font window raised: {raised})");
    assert!(!still_fonts, "Esc left the font window in front of the settings open");
    assert!(table, "Esc on the font window closed the settings behind it");
}

/// Reported behaviour: with Settings -> Keyboard open and a line in hand, Ctrl+Esc put the line down and left the
/// settings open. The readers of Esc behind the settings take it with any modifier held; so does the table.
#[test]
fn ctrl_esc_over_the_table_closes_the_settings_and_keeps_the_line() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    hand.chord(egui::Modifiers::CTRL, Key::Escape);
    let (table, held) = (hand.shows(&extrude_row()), hand.app.tools.armed.drawing());
    assert!(line && opened, "setup: (line: {line}, table: {opened})");
    assert!(!table, "Ctrl+Esc left the settings open");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Ctrl+Esc reached the line in hand behind the settings");
}

/// Reported behaviour: with Settings -> Keyboard open and a line in hand, Ctrl+K then Esc closed the command
/// search, and the next Esc put the line down instead of closing the settings. A window closed leaves its
/// layer at the top of the order; only the windows still drawn stand in front.
#[test]
fn esc_after_a_window_closed_over_the_table_goes_to_the_settings() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let opened = press_keyboard_settings(&mut hand) && hand.shows(&extrude_row());
    hand.chord(egui::Modifiers::COMMAND, Key::K);
    let search = hand.window_titled(&tr("cs-title")).is_some();
    hand.key(Key::Escape);
    let closed = hand.window_titled(&tr("cs-title")).is_none();
    hand.key(Key::Escape);
    let (table, held) = (hand.shows(&extrude_row()), hand.app.tools.armed.drawing());
    assert!(line && opened && search && closed, "setup: (line: {line}, table: {opened}, search opened: {search}, search closed: {closed})");
    assert!(!table, "Esc after the command search closed left the settings open");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc after the command search closed reached the line in hand behind the settings");
}

/// WHAT A FIELD TYPED INTO WHILE A KEY IS WAITED FOR CAME TO: the key of the extrusion, the question of a clash,
/// what the field holds, and whether the table still waits.
struct TypedOverTheWait {
    key: String,
    clash: Option<qymcad_ui_state::HotkeyClash>,
    field: String,
    waits: bool,
}

/// WHAT IS TYPED: a letter comes as its key and its text in one frame, as a keyboard sends it; Backspace as its key.
enum Typing {
    Letter { key: Key, text: &'static str },
    Backspace,
}

/// THE FIELD TYPED INTO: the search of the whole settings, or the table's own filter.
#[derive(Clone, Copy)]
enum Field {
    SettingsSearch,
    TableFilter,
}

impl Field {
    fn hint(self) -> String {
        tr(match self {
            Field::SettingsSearch => "settings-search",
            Field::TableFilter => "hotkeys-filter-hint",
        })
    }

    fn holds(self, app: &App) -> String {
        match self {
            Field::SettingsSearch => app.scheme.search.clone(),
            Field::TableFilter => app.hotkeys.filter.clone(),
        }
    }
}

/// The extrusion's key is clicked, then `field` is clicked and typed into.
fn type_over_the_wait(hand: &mut Hand, field: Field, typing: Typing) -> Option<TypedOverTheWait> {
    let ready = press_keyboard_settings(hand) && wait_on_the_extrusion(hand) && hand.press_word(&field.hint(), egui::Pos2::ZERO);
    if !ready {
        return None;
    }
    match typing {
        Typing::Letter { key, text } => {
            let event = |pressed| egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers: Default::default() };
            hand.frame(Vec::new());
            hand.frame(vec![event(true), egui::Event::Text(text.to_string())]);
            hand.frame(vec![event(false)]);
        }
        Typing::Backspace => {
            hand.key(Key::Backspace);
        }
    }
    let key = qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude");
    Some(TypedOverTheWait { key, clash: hand.app.hotkeys.clash.clone(), field: field.holds(hand.app), waits: hand.app.hotkeys.action.is_some() })
}

/// The field holds what was typed into it, and the extrusion keeps E, with no question and no waiting.
fn assert_the_field_took_the_keys(typed: Option<TypedOverTheWait>, holds: &str) {
    let Some(t) = typed else { panic!("setup: the extrusion's key did not wait, or the field was not reached") };
    assert_eq!(t.field, holds, "the keys did not reach the field");
    assert_eq!(t.key, "E", "a key typed into the field rebound the extrusion");
    assert!(t.clash.is_none(), "a key typed into the field asked about a clash: {:?}", t.clash);
    assert!(!t.waits, "the table still waits for a key while the field holds the keyboard");
}

/// Reported behaviour: with the extrusion's key waiting, a letter typed into "Search the settings" rebound the
/// extrusion to it, silently. A is free in the part area, and the Keyboard section stays shown under it.
#[test]
fn a_letter_typed_into_the_settings_search_leaves_the_waited_key_alone() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let typed = type_over_the_wait(&mut hand, Field::SettingsSearch, Typing::Letter { key: Key::A, text: "a" });
    assert_the_field_took_the_keys(typed, "a");
}

/// Reported behaviour: with the extrusion's key waiting, Backspace in "Search the settings" left the extrusion
/// without a key.
#[test]
fn backspace_in_the_settings_search_leaves_the_waited_key_alone() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let typed = type_over_the_wait(&mut hand, Field::SettingsSearch, Typing::Backspace);
    assert_the_field_took_the_keys(typed, "");
}

/// Reported behaviour: with the extrusion's key waiting, a letter typed into the table's own filter rebound the
/// extrusion to it. T is free in the part area and keeps the extrusion's row in the table.
#[test]
fn a_letter_typed_into_the_filter_of_the_table_leaves_the_waited_key_alone() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let typed = type_over_the_wait(&mut hand, Field::TableFilter, Typing::Letter { key: Key::T, text: "t" });
    assert_the_field_took_the_keys(typed, "t");
}

/// Reported behaviour: with a line in hand and the table waiting for a key, Esc ended the waiting and put the line
/// down behind the settings too. The press the table waits for is the table's alone.
#[test]
fn esc_that_ends_the_waiting_reaches_nothing_behind() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let line = line_in_hand(&mut hand);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    hand.key(Key::Escape);
    let (still_waits, table, held) = (hand.shows(&tr("hotkeys-press")), hand.shows(&extrude_row()), hand.app.tools.armed.drawing());
    assert!(line && waits, "setup: (line: {line}, waits: {waits})");
    assert!(!still_waits && table, "Esc did not end the waiting alone (waits: {still_waits}, settings open: {table})");
    assert_eq!(held, Some(qymcad_ui_state::SketchShape::Line), "Esc that ended the waiting put the line in hand down behind the settings");
}

/// Reported behaviour: with the table waiting for a key and About opened in front of the settings, J pressed over
/// About became the key of the extrusion.
#[test]
fn a_key_pressed_over_a_window_in_front_does_not_bind() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let about = hand.press_word(&tr("menu-help"), egui::Pos2::ZERO) && hand.press_word(&tr("help-about"), egui::Pos2::ZERO) && hand.window_titled(&tr("win-about")).is_some();
    hand.key(Key::J);
    let key = qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude");
    assert!(waits && about, "setup: (waits: {waits}, About in front: {about})");
    assert_eq!(key, "E", "J pressed over About in front of the settings rebound the extrusion");
}

/// Reported behaviour: "Reset this section" on Keyboard with a key waited for left the table waiting, and the next
/// press bound the key the reset had just put back.
#[test]
fn the_section_reset_ends_the_waiting() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    // the table filtered down to the extrusion, so the reset under it is on screen
    let filtered = press_keyboard_settings(&mut hand) && hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    hand.type_text("extrude");
    let waits = filtered && wait_on_the_extrusion(&mut hand);
    let reset = hand.press_word(&tr("settings-reset-section"), egui::Pos2::ZERO);
    hand.key(Key::J);
    let (key, still_waits) = (qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude"), hand.shows(&tr("hotkeys-press")));
    assert!(waits && reset, "setup: (waits: {waits}, reset pressed: {reset})");
    assert!(key == "E" && !still_waits, "the section reset left the table waiting (extrude key now {key:?}, waits: {still_waits})");
}

/// A KEY PRESSED UNDER AN OPEN FILE CHOOSER binds nothing: the system window is modal, and the table behind it waits
/// on. The chooser is the system's and cannot be pressed open by the hand; it is armed as `deaf_while_the_system_asks`
/// arms it.
#[test]
fn a_key_pressed_under_the_file_chooser_does_not_bind() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let waits = press_keyboard_settings(&mut hand) && wait_on_the_extrusion(&mut hand);
    let (_answer, asked) = std::sync::mpsc::channel();
    hand.app.arm_file_ask(asked, |_app, _path| {});
    hand.key(Key::J);
    let key = qymcad_ui_state::hotkey_key(&hand.app.set, "part.extrude");
    assert!(waits, "setup: the table did not wait for the key of the extrusion");
    assert_eq!(key, "E", "J pressed under an open file chooser rebound the extrusion");
}

/// Reported behaviour: Settings -> Keyboard showed two buttons "Reset this section" - one put back the keys of the
/// sketch alone, the other every reassigned key of all the workbenches. The reset of an area names the area.
#[test]
fn the_reset_of_an_area_names_its_area() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
    let filtered = press_keyboard_settings(&mut hand) && hand.press_word(&tr("hotkeys-filter-hint"), egui::Pos2::ZERO);
    hand.type_text("extrude");
    let rebound = filtered && wait_on_the_extrusion(&mut hand) && hand.key(Key::J).shows("J");
    let named = crate::i18n::tr1("hotkeys-reset-area", "area", &tr("hotkeys-area-part"));
    let section = tr("settings-reset-section");
    let (area_reset, twice) = (hand.shows(&named), hand.words_drawn().iter().filter(|(w, _)| *w == section).count() > 1);
    assert!(rebound, "setup: the extrusion was not given J");
    assert!(area_reset && named != section && !twice, "the reset of the Part keys is not named by its area (shown: {area_reset}, caption {named:?}, the section's caption twice: {twice})");
}

/// THE FILTER AND "RESET EVERY KEY" STAY ON TOP while the rows are scrolled: they are for finding a row among sixty
/// and more, and a filter scrolled away with them is looked for at the top of a list that no longer starts there.
#[test]
fn the_filter_of_the_keys_stays_when_the_rows_are_scrolled() {
    let _en = InEnglish::new();
    let mut app = App::default();
    let mut hand = Hand::new(&mut app);
    let opened = press_keyboard_settings(&mut hand);
    let over = hand.written_at(&extrude_row()).map(|r| r.center());
    let bottom = tr("hotkeys-rebind-note");
    let at_the_bottom_before = hand.shows(&bottom);
    if let Some(over) = over {
        for _ in 0..40 {
            let wheel = egui::Event::MouseWheel { unit: egui::MouseWheelUnit::Point, delta: egui::vec2(0.0, -400.0), phase: egui::TouchPhase::Move, modifiers: egui::Modifiers::NONE };
            hand.frame(vec![egui::Event::PointerMoved(over), wheel]);
        }
    }
    let (scrolled, filter, search) = (hand.shows(&bottom), hand.written_at(&tr("hotkeys-filter-hint")), hand.written_at(&tr("settings-search")));
    assert!(
        opened && over.is_some() && !at_the_bottom_before && scrolled,
        "setup: the rows were not scrolled to their end (opened: {opened}, row: {over:?}, end shown before: {at_the_bottom_before}, after: {scrolled})"
    );
    let top = search.map(|r| r.max.y).expect("setup: the search of the settings is not shown");
    // painted, but under the top of the window: a clipped widget is still painted, at the place it scrolled to
    assert!(filter.is_some_and(|f| f.min.y >= top), "the filter of the keys scrolled away with the rows: it stands at {filter:?}, the window's search ends at y = {top}");
}
