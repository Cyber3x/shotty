use ratatui::style::Style;

/// Represents a style that can be conditionally applied to another `Style`.
///
/// # Example
/// ```rust
/// use ratatui::style::{Style, Color};
/// let patch = PatchStyle::new(true, Style::default().fg(Color::Red));
/// ```
pub struct PatchStyle {
    condition: bool,
    style: Style,
}

impl PatchStyle {
    /// Creates a new `PatchStyle`.
    ///
    /// # Parameters
    /// - `condition`: Whether the style should be applied.
    /// - `style`: The `Style` to apply.
    pub fn new(condition: bool, style: Style) -> Self {
        Self { condition, style }
    }
}

/// Applies a series of `PatchStyle`s to a base `Style`.
///
/// Only patches with `condition == true` will be applied. Patches are applied in
/// the order they appear in the `patch_styles` vector.
///
/// # Parameters
/// - `base_style`: The original `Style` to modify.
/// - `patch_styles`: A vector of `PatchStyle`s to apply.
///
/// # Returns
/// A new `Style` with all applicable patches applied.
pub fn compose_style(mut base_style: Style, patch_styles: Vec<PatchStyle>) -> Style {
    patch_styles
        .iter()
        .filter(|style| style.condition)
        .map(|style| style.style)
        .for_each(|style| base_style = base_style.patch(style));

    base_style
}
