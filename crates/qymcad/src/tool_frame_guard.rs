//! Guard on THE FRAME OF A TOOL: outside the frame (`qymcad_ui_state::tool_frame`), the crates of the workbenches lay no
//! area or window of their own for the fields of a tool and read no Enter or Esc of their own.
//!
//! A tool of the frame says what it needs, and the frame draws it and answers the keys. A tool that lays a box of its
//! own, or reads Enter or Esc itself, is where the tools of the workbenches drift apart - each its own Enter, its own
//! Esc, its own place for its words. An exception is named with its reason (`ALLOWED`); there is none.

#[cfg(test)]
mod tests {
    /// THE CRATES OF THE WORKBENCHES whose tools stand in the frame.
    const WORKBENCHES: [&str; 3] = ["qymcad-part/src", "qymcad-assembly/src", "qymcad-sketch/src"];

    /// What a tool of the frame does not do itself: lay an area, a box at the geometry or a window, read Enter or Esc.
    /// Each is matched as a name standing on its own, whatever path leads to it: `window(` imported, `qymcad_shell::window(`
    /// and `qymcad_ui_state::at_geometry(` are one door each.
    const FORBIDDEN: [&str; 5] = ["Area::new(", "window(", "at_geometry(", "Key::Enter", "Key::Escape"];

    /// THE DOORS a line of code opens: each forbidden name it holds where no letter, digit or `_` stands before it, so
    /// `window(` is caught behind `use` or a path and `is_window(` is not.
    fn doors(code: &str) -> Vec<&'static str> {
        FORBIDDEN.iter().copied().filter(|door| code.match_indices(door).any(|(at, _)| !code[..at].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_'))).collect()
    }

    /// THE DOORS ARE KNOWN WHATEVER PATH LEADS TO THEM. Reported behaviour: the guard forbade only the literal
    /// `Area::new(` and `qymcad_shell::window(`, so a box laid by `qymcad_ui_state::at_geometry` or a window opened by an
    /// imported `window(` passed.
    #[test]
    fn every_door_is_known_by_its_name() {
        /// A line of code and the door it opens.
        struct Sample {
            line: &'static str,
            door: &'static str,
        }
        for Sample { line, door } in [
            Sample { line: "let a = qymcad_ui_state::at_geometry(id);", door: "at_geometry(" },
            Sample { line: "at_geometry(id).show(ctx, |ui| {});", door: "at_geometry(" },
            Sample { line: "window(title, id, ctx).show(ctx, |ui| {});", door: "window(" },
            Sample { line: "qymcad_shell::window(title, id, ctx)", door: "window(" },
            Sample { line: "egui::Area::new(id)", door: "Area::new(" },
        ] {
            assert_eq!(doors(line), [door], "the door in {line:?} is not caught");
        }
        assert!(doors("fn is_window(x: u8) -> bool { my_at_geometry(x) }").is_empty(), "a name that only ends in a door is taken for one");
    }

    /// A function allowed to lay a box of its own or read a key of its own, and why.
    struct Allowed {
        path: &'static str,
        function: &'static str,
        why: &'static str,
    }

    /// Why each drawing tool of the sketch is allowed its own box.
    const ALLOWED: [Allowed; 0] = [];

    /// One place a tool does what the frame does: the file, the function it stands in and what it does.
    struct Found {
        path: String,
        function: String,
        what: &'static str,
        line: usize,
    }

    /// EVERY PLACE IN THE CRATES OF THE WORKBENCHES where a box is laid or a key read outside the frame: the working
    /// half of each file, a module of checks below it left out.
    fn found() -> Vec<Found> {
        let root = qymcad_i18n::ratchet::crates_root();
        let mut stack: Vec<std::path::PathBuf> = WORKBENCHES.iter().map(|w| root.join(w)).collect();
        let mut out = Vec::new();
        while let Some(dir) = stack.pop() {
            for e in std::fs::read_dir(&dir).expect("the sources read").flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                    continue;
                }
                if p.extension().is_none_or(|x| x != "rs") || qymcad_i18n::ratchet::is_test_module(&p) {
                    continue;
                }
                let path = p.strip_prefix(&root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                let work = ["#[cfg(test)]\nmod ", "#[cfg(test)]\npub(crate) mod ", "#[cfg(test)]\npub(in crate::gui) mod ", "#[cfg(test)]\npub(super) mod "]
                    .iter()
                    .fold(text.as_str(), |t, cut| t.split(cut).next().unwrap_or(""));
                let mut function = String::new();
                for (i, line) in work.lines().enumerate() {
                    let code = line.split("//").next().unwrap_or("");
                    if let Some(at) = code.find("fn ") {
                        let name: String = code[at + 3..].chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                        if !name.is_empty() {
                            function = name;
                        }
                    }
                    for what in doors(code) {
                        out.push(Found { path: path.clone(), function: function.clone(), what, line: i + 1 });
                    }
                }
            }
        }
        out
    }

    #[test]
    fn every_exception_says_why() {
        for a in ALLOWED {
            assert!(a.why.split_whitespace().count() >= 8, "the exception of {} in {} gives no reason", a.function, a.path);
        }
    }

    /// NO TOOL OF THE PART OR THE ASSEMBLY LAYS A BOX OR READS A KEY OF ITS OWN: it describes itself, and the frame draws
    /// it and answers Enter and Esc.
    #[test]
    fn no_tool_lays_a_box_or_reads_a_key_outside_the_frame() {
        let outside: Vec<String> =
            found().into_iter().filter(|f| !ALLOWED.iter().any(|a| a.path == f.path && a.function == f.function)).map(|f| format!("{}:{} in {}: {}", f.path, f.line, f.function, f.what)).collect();
        assert!(
            outside.is_empty(),
            "a tool lays a box or reads Enter or Esc of its own instead of describing itself to the frame of a tool (`tool_bar`, `tool_box`); {} of them:\n{}",
            outside.len(),
            outside.join("\n")
        );
    }

    /// AN EXCEPTION NOT NEEDED ANY MORE GOES: a drawing tool of the sketch moved onto the frame leaves the list with it.
    #[test]
    fn every_exception_still_stands() {
        let all = found();
        let stale: Vec<String> = ALLOWED.iter().filter(|a| !all.iter().any(|f| f.path == a.path && f.function == a.function)).map(|a| format!("{} in {}", a.function, a.path)).collect();
        assert!(stale.is_empty(), "the exception lays no box and reads no key any more - take it off the list: {stale:?}");
    }
}
