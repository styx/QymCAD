//! A WINDOW OPENED LEAVES THE MENUS TO BE OPENED. egui puts a window with no place of its own at (16, 16) of the
//! screen, over the menu bar, and the grab of its corner takes the click meant for "File". Reported behaviour: with the
//! settings open, a click on the middle of "File" opened nothing.
#[cfg(test)]
mod tests {
    use crate::gui::hand::Hand;
    use crate::gui::App;
    use crate::i18n::tr;

    /// A window, and the clicks through the interface that open it.
    struct Opened {
        title: &'static str,
        path: &'static [&'static str],
    }

    const WINDOWS: &[Opened] = &[
        Opened { title: "win-settings", path: &["menu-windows", "menu-settings"] },
        Opened { title: "win-parts-library", path: &["menu-windows", "win-parts-library"] },
        Opened { title: "win-params", path: &["wb-params"] },
        Opened { title: "doc-props-title", path: &["menu-file", "file-doc-props"] },
        Opened { title: "help-title", path: &["menu-help", "help-title"] },
        Opened { title: "win-about", path: &["menu-help", "help-about"] },
    ];

    /// What a click on the middle of "File" did with one window open, or why the window was not reached.
    fn file_menu_with(window: &Opened) -> Result<(), String> {
        let mut app = App::default();
        let mut hand = Hand::new(&mut app);
        hand.press_word(&tr("start-close"), egui::Pos2::ZERO);
        // each word is sought below the one pressed before it: the entry "Help" of the menu "Help" is the same word
        let mut near = egui::Pos2::ZERO;
        let pressed = window.path.iter().all(|w| {
            let hit = hand.press_word(&tr(w), near);
            near = hand.written_at(&tr(w)).map_or(near, |r| r.left_bottom() + egui::vec2(0.0, 20.0));
            hit
        });
        if !pressed || hand.window_titled(&tr(window.title)).is_none() {
            return Err(format!("setup: {} did not open by {:?}", window.title, window.path));
        }
        let file = hand.written_at(&tr("menu-file")).ok_or("setup: no \"File\" on screen")?;
        if hand.click_screen(file.center()).shows(&tr("file-new")) {
            Ok(())
        } else {
            Err(format!("with {} open, a click on the middle of \"File\" did not open the menu", window.title))
        }
    }

    #[test]
    fn every_window_leaves_the_menus_to_be_opened() {
        let _en = crate::gui::tests::InEnglish::new();
        let failed: Vec<String> = WINDOWS.iter().filter_map(|w| file_menu_with(w).err()).collect();
        assert!(failed.is_empty(), "{}", failed.join("\n"));
    }
}
