use std::f64::consts::PI;

const EXPANSION_RADIUS: f64 = 180.0;
const SPREAD: f64 = PI * 1.2;

/// Returns `(x, y, angle)` for each of `count` children fanned out around `(px, py)`.
/// Children face away from the grandparent using `parent_angle`.
pub fn place_children(px: f64, py: f64, parent_angle: f64, count: usize) -> Vec<(f64, f64, f64)> {
    let base_angle = parent_angle + PI;
    (0..count)
        .map(|i| {
            let angle = if count == 1 {
                base_angle
            } else {
                base_angle - SPREAD / 2.0 + i as f64 * SPREAD / (count - 1) as f64
            };
            (
                px + EXPANSION_RADIUS * angle.cos(),
                py + EXPANSION_RADIUS * angle.sin(),
                angle,
            )
        })
        .collect()
}
