//! A BODY KEEPS ITS NAME THROUGH THE FEATURES ADDED ON IT.
//!
//! Reported behaviour: Body 1 renamed to "Housing", a fillet of R2 added, and the tree showed "Body 2". A feature
//! that consumes a body makes a new body id, and the record of the new id was named by the count of all body
//! records in the document: the name given by hand stayed on the consumed record, and a shell turned "Body 1" into
//! "Body 2", the first body of a second part was "Body 3".
//!
//! The name lives on the record of the body the line of one-body operations starts from (`Project::name_root`). The
//! pieces of a split start lines of their own and, until renamed, are called by their source and their place:
//! "Housing (1)", "Housing (2)".
use qymcad_core::geom::Point2;
use qymcad_core::model::{Id, Project};

fn brick(p: &mut Project, name: &str) -> Id {
    let sid = p.add_line_sketch(name, vec![Point2::new(0.0, 0.0), Point2::new(20.0, 0.0), Point2::new(20.0, 20.0), Point2::new(0.0, 20.0)], true);
    let si = p.sketch_index(sid).unwrap();
    p.regen_sketch(si);
    if let Some(o) = p.sketch_owner(sid) {
        p.set_active_component(Some(o));
    }
    p.add_sketch_node(sid, name);
    let closed: Vec<Id> = p.sketches[si].contour_ids.iter().copied().filter(|c| p.contour_profile_xy(*c).is_some()).collect();
    let b = p.add_extrude_multi(sid, closed, 20.0, qymcad_core::feature::Reach::Forward, 0.0, vec![]);
    qymcad_testkit::regenerate(p);
    b
}

fn name_of(p: &Project, body: Id) -> String {
    p.mesh_name(p.mesh_index(body).expect("the body has a record"))
}

fn rename(p: &mut Project, body: Id, name: &str) {
    let mi = p.mesh_index(body).expect("the body has a record");
    p.set_mesh_name(mi, name.into());
}

#[test]
fn a_fillet_keeps_the_name_given_by_hand() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    rename(&mut p, base, "Housing");
    let fillet = p.add_fillet(base, 2.0, vec![]);
    qymcad_testkit::regenerate(&mut p);
    assert!(p.regen_faces.contains_key(&fillet), "the fillet was built");
    assert_eq!(name_of(&p, fillet), "Housing", "the filleted body is the same body with one more feature");
}

#[test]
fn a_shell_keeps_the_generated_name() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    assert_eq!(name_of(&p, base), "name-body#1");
    let shell = p.add_shell(base, 1.0, vec![], false);
    qymcad_testkit::regenerate(&mut p);
    assert!(p.regen_faces.contains_key(&shell), "the shell was built");
    assert_eq!(name_of(&p, shell), "name-body#1", "a shell does not make Body 1 into Body 2");
}

#[test]
fn the_first_body_of_a_second_part_is_body_2() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    p.add_fillet(base, 2.0, vec![]);
    qymcad_testkit::regenerate(&mut p);
    let second = p.add_component("Second");
    p.set_active_component(Some(second));
    let other = brick(&mut p, "sq2");
    assert_eq!(name_of(&p, other), "name-body#2", "two bodies in the document, whatever number of records the first one left");
}

#[test]
fn a_name_given_after_the_fillet_survives_deleting_the_fillet() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    let fillet = p.add_fillet(base, 2.0, vec![]);
    qymcad_testkit::regenerate(&mut p);
    rename(&mut p, fillet, "Housing");
    p.delete_feature_op(fillet);
    qymcad_testkit::regenerate(&mut p);
    assert_eq!(name_of(&p, base), "Housing", "the body the tree shows again is the one that was renamed");
}

#[test]
fn the_pieces_of_a_split_keep_the_name_with_their_place() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    rename(&mut p, base, "Housing");
    let pieces = p.add_split_body(base, 0, 0, 5.0, 2);
    qymcad_testkit::regenerate(&mut p);
    assert!(pieces.iter().all(|b| p.regen_faces.contains_key(b)), "both pieces were built");
    let names: Vec<String> = pieces.iter().map(|&b| name_of(&p, b)).collect();
    assert_eq!(names, ["Housing (1)", "Housing (2)"]);
    let fillet = p.add_fillet(pieces[1], 1.0, vec![]);
    qymcad_testkit::regenerate(&mut p);
    assert_eq!(name_of(&p, fillet), "Housing (2)", "a feature on a piece keeps the name of the piece");
}

#[test]
fn a_piece_renamed_on_its_own_leaves_the_others_following_the_source() {
    let mut p = Project::default();
    p.new_document();
    let base = brick(&mut p, "sq");
    let pieces = p.add_split_body(base, 0, 0, 5.0, 2);
    qymcad_testkit::regenerate(&mut p);
    rename(&mut p, pieces[0], "Lid");
    rename(&mut p, base, "Housing");
    assert_eq!(name_of(&p, pieces[0]), "Lid");
    assert_eq!(name_of(&p, pieces[1]), "Housing (2)", "a piece not renamed follows its source");
}
