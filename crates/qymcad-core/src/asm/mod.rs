//! Assembly solver: the positional problem of placing bodies so that their mates hold.
//!
//! What is hand-written here matters more than how much. Hand-written linear algebra (Gaussian
//! elimination without rank detection) and hand-written derivatives both fail silently: a degenerate
//! system is solved arbitrarily, and a wrong derivative shows up as "the body landed in the wrong
//! place", which is indistinguishable from bad geometry.
//!
//! The linear algebra and the graph are taken from established libraries:
//!
//! * `nalgebra` — linear algebra, SVD for rank and null space, quaternions, `Isometry3`;
//! * `petgraph` — the mate graph: connected components, traversal from grounded bodies.
//!
//! Local: what each mate means (`problem`), its analytic derivative (`solve::jacobian`, checked
//! against a numerical derivative for every constraint), and the damped Gauss-Newton step around
//! the SVD (`iterate`).

pub mod bridge;
pub mod connector;
pub mod decompose;
pub mod frame;
pub mod iterate;
pub mod joint;
pub mod problem;
pub mod solve;

pub use frame::{Anchor, Pose};
pub use problem::{Body, BodyId, Constraint, Problem};
