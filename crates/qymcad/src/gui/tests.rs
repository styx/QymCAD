//! THE GUI TESTS, lifted out of `gui.rs`.
//!
//! This is a CHILD module of `gui` rather than a neighbouring file: `App`'s private fields are visible to the
//! owning module and its descendants, so the headless flows keep working with the internal state directly.
#![cfg(test)]
// DO NOT REMOVE: rustc calls this import unused, yet without it the file does not compile - it is what lets the
// nested test modules see the `gui` names by the `super::` path (a private import is available to descendants).
#[allow(unused_imports)]
use super::*;

/// THE INTERFACE IN ENGLISH while the check runs, and the language before put back when it ends - a check that fails
/// half-way included: the language is the process's, and a check left in English would hand it to the next.
pub(crate) struct InEnglish {
    before: String,
}

impl InEnglish {
    pub(crate) fn new() -> Self {
        let before = qymcad_i18n::language();
        qymcad_i18n::set_language("en");
        Self { before }
    }
}

impl Drop for InEnglish {
    fn drop(&mut self) {
        qymcad_i18n::set_language(&self.before);
    }
}

mod command_flow_tests;

mod section_clip_tests;

mod gizmo_ring_sign_tests;

mod section_drag_tests;

mod pick_depth_tests;

mod gizmo_math_tests;

mod sketch_conflict_ui_tests;

mod brep_warmup_tests;

mod sketch_plane_pick_frames_tests;

mod faces_part_in_light;

mod the_box_of_a_joint_is_laid_in_sections;

mod a_box_follows_what_it_sizes;

mod a_tool_with_nothing_to_apply_closes_plainly;

mod a_sketch_tool_outside_a_sketch_says_so;

mod the_doors_of_a_tool;

mod a_box_in_the_corner_stands_clear_of_a_bar;

mod the_floor_fades_round_the_view;

mod parts_far_apart_draw_clean;

mod the_camera_holds_after_a_far_view;

pub(in crate::gui) mod a_quick_rebuild_does_not_blink;

mod nothing_changed_rebuilds_nothing;

mod a_hinge_follows_its_parameter;

mod a_cut_through_both_ways_reopens_both_ways;

mod a_cut_on_a_datum_follows_its_parameter_in_the_window;

mod a_reference_found_as_the_other_piece_is_shown;

mod a_reference_lost_by_an_edit_is_shown;

mod every_tool_takes_every_curve;

mod two_fillets_meet_on_a_side;

mod a_figure_is_carried_whole;

mod a_figure_opens_its_size;

mod a_figure_shows_no_badge_of_its_own;

mod a_projected_ellipse_follows_its_body;

mod a_spline_survives_a_circle_deleted_by_hand;

mod a_reference_size_reads_the_curve_as_it_stands;

mod a_corner_at_a_crossing_keeps_its_two_sides;

mod a_fillet_of_a_spline_holds_when_dragged;

mod an_offset_follows_its_source;

mod the_offset_of_a_closed_spline_is_light;

mod a_point_snapped_to_a_centre_line_is_held_on_it;

mod a_circle_shows_its_centre_lines;

mod a_trim_preview_shows_any_curve;

mod a_curve_under_the_pointer_is_the_one_cut;

mod a_point_drawn_on_a_curve_is_held_on_it;

mod a_picked_curve_of_any_kind_is_lit;

mod a_corner_of_two_crossing_curves_is_taken_like_one_of_lines;

mod a_circle_is_drawn_in_each_of_its_modes;

mod a_spline_closes_on_its_first_node;

mod a_filleted_spline_drags_lightly;

mod a_ring_of_a_large_spline_is_picked_lightly;

mod the_copy_of_an_offset_shows_no_nodes;

mod a_line_drawn_by_a_curve_touches_it;

mod a_tangent_arc_goes_on_from_any_curve;

mod a_crossing_of_any_curve_is_snapped_to;

mod a_whole_ellipse_is_cut_as_any_curve;

mod a_blend_built_short_advises_what_builds;

mod the_box_of_a_rectangle_goes_with_its_tool;

mod a_feature_value_is_named_in_its_box;

mod a_number_of_the_bar_is_a_field_of_the_box;

mod the_boxes_look;

mod the_tool_in_hand_after_its_work;

mod the_key_of_the_tool_in_hand_puts_it_down;

mod a_key_pressed_again_takes_the_next_way;

mod a_window_leaves_the_menus_to_be_opened;

mod esc_closes_the_window_in_front;

mod finish_climbs_the_tree;

mod profiles_are_taken_by_a_box;

mod the_author_is_supported;
