//! A BODY RENAMED IN THE TREE KEEPS ITS NAME THROUGH A FILLET ADDED ON IT.
//!
//! Reported behaviour: Body 1 renamed to "Housing" in the tree, a fillet of R2 added on a vertical edge, and the tree
//! showed "Body 2". The rename goes in by F2 and the fillet by its tool, each in a real frame, and the name is read
//! off the window.
#[cfg(test)]
mod tests {
    use crate::gui::hand::Hand;
    use crate::gui::{App, Sel};

    #[test]
    fn a_fillet_keeps_the_name_typed_in_the_tree() {
        let mut app = App::default();
        app.set.gpu_viewport = false;
        app.waiting.splash_until = None;
        let si = app.create_sketch_on(qymcad_core::feature::SketchPlane::default());
        app.project.add_rect_entity(si, 0.0, 0.0, 20.0, 20.0, qymcad_core::feature::Purpose::Real);
        app.project.regen_sketch(si);
        app.finish_sketch_edit();
        app.chosen.sel = Sel::Sketch(si);
        Hand::new(&mut app).look_at([10.0, 10.0, 10.0], 8.0).tool(1).set("height", 20.0).enter();
        let body = app.project.timeline.iter().rev().find_map(|n| n.kind.body()).expect("the cube");

        let mut hand = Hand::new(&mut app);
        assert!(hand.double_click_word("Part 1"), "the part is not in the tree");
        assert!(hand.press_word("Body 1", egui::pos2(0.0, 0.0)), "the body is not in the tree of its part");
        let all = egui::Event::Key { key: egui::Key::A, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::COMMAND };
        hand.key(egui::Key::F2).frame(vec![all, egui::Event::Text("Housing".into())]).key(egui::Key::Enter);
        drop(hand);
        let shown = app.project.mesh_index(body).map(|mi| app.project.mesh_name(mi));
        assert_eq!(shown.as_deref(), Some("Housing"), "F2, the typing and Enter did not rename the body");

        let edge = app.project.regen_edges[&body].iter().find(|e| (e.a[2] - e.b[2]).abs() > 1.0).map(|e| e.mid).expect("a vertical edge");
        qymcad_ui_state::select_body(&mut app.project, &mut app.chosen.sel, &mut app.viewing.view, body);
        let mut hand = Hand::new(&mut app);
        hand.look_at([10.0, 10.0, 10.0], 8.0).tool(4).click(edge).set("radius", 2.0).enter();
        assert!(hand.shows("Housing"), "the tree lost the name the body was given");
        assert!(!hand.shows("Body 2"), "the tree named the filleted body by a count of records");
        drop(hand);
        let fillet = app.project.timeline.iter().rev().find_map(|n| n.kind.body()).expect("the fillet");
        assert_ne!(fillet, body, "the fillet was not added");
    }
}
