use clap::Parser;

use serde::Deserialize;

#[derive(Parser, Deserialize, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// list of chars to use for hints <fjghdkslaemuvitywoqpcbnxz>
    #[arg(long)]
    pub chars: Option<String>,

    /// set the window background color <rrggbb>
    #[arg(long)]
    pub window_background_color: Option<String>,

    /// set the window background opacity <0-1.0>
    #[arg(long)]
    pub window_background_opacity: Option<f64>,

    /// set the label background color <rrggbb>
    #[arg(long)]
    pub label_background_color: Option<String>,

    /// set the label background opacity <0-1.0>
    #[arg(long)]
    pub label_background_opacity: Option<f64>,

    /// set the label text color <rrggbb>
    #[arg(long)]
    pub label_text_color: Option<String>,

    /// set the label background color <rrggbb>
    #[arg(long)]
    pub focused_background_color: Option<String>,

    /// set the focused background opacity <0-1.0>
    #[arg(long)]
    pub focused_background_opacity: Option<f64>,

    /// set the focused text color <rrggbb>
    #[arg(long)]
    pub focused_text_color: Option<String>,

    /// set the font family
    #[arg(long)]
    pub font_family: Option<String>,

    /// set the font weight
    #[arg(long)]
    pub font_weight: Option<String>,

    /// set the font size, see: https://www.w3.org/TR/css-fonts-3/#font-size-prop
    #[arg(long)]
    pub font_size: Option<String>,

    /// set the label padding-x <px>
    #[arg(long)]
    pub label_padding_x: Option<i32>,

    /// set the label padding-y <px>
    #[arg(long)]
    pub label_padding_y: Option<i32>,

    /// set the boreder radius <px>
    #[arg(long)]
    pub label_border_radius: Option<i32>,

    /// set the label margin-x <px>
    #[arg(long)]
    pub label_margin_x: Option<i32>,

    /// set the label margin-y <px>
    #[arg(long)]
    pub label_margin_y: Option<i32>,
}

impl Args {
    // ugh
    pub fn merge(&mut self, other: &Self) {
        if other.window_background_color.is_some() {
            self.window_background_color = other.window_background_color.clone();
        }
        if other.window_background_opacity.is_some() {
            self.window_background_opacity = other.window_background_opacity;
        }
        if other.label_background_color.is_some() {
            self.label_background_color = other.label_background_color.clone();
        }
        if other.label_background_opacity.is_some() {
            self.label_background_opacity = other.label_background_opacity;
        }
        if other.label_text_color.is_some() {
            self.label_text_color = other.label_text_color.clone();
        }
        if other.focused_background_color.is_some() {
            self.focused_background_color = other.focused_background_color.clone();
        }
        if other.focused_background_opacity.is_some() {
            self.focused_background_opacity = other.focused_background_opacity;
        }
        if other.focused_text_color.is_some() {
            self.focused_text_color = other.focused_text_color.clone();
        }
        if other.font_family.is_some() {
            self.font_family = other.font_family.clone();
        }
        if other.font_weight.is_some() {
            self.font_weight = other.font_weight.clone();
        }
        if other.font_size.is_some() {
            self.font_size = other.font_size.clone();
        }
        if other.label_padding_x.is_some() {
            self.label_padding_x = other.label_padding_x;
        }
        if other.label_padding_y.is_some() {
            self.label_padding_y = other.label_padding_y;
        }
        if other.label_border_radius.is_some() {
            self.label_border_radius = other.label_border_radius;
        }
        if other.label_margin_x.is_some() {
            self.label_margin_x = other.label_margin_x;
        }
        if other.label_margin_y.is_some() {
            self.label_margin_y = other.label_margin_y;
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            chars: Some("fjghdkslaemuvitywoqpcbnxz".to_string()),
            window_background_color: Some("1d1f21".to_string()),
            window_background_opacity: Some(0.2),
            label_background_color: Some("1d1f21".to_string()),
            label_background_opacity: Some(1.0),
            label_text_color: Some("c5c8c6".to_string()),
            focused_background_color: Some("285577".to_string()),
            focused_background_opacity: Some(1.0),
            focused_text_color: Some("ffffff".to_string()),
            font_family: Some("monospace".to_string()),
            font_weight: Some("bold".to_string()),
            font_size: Some("medium".to_string()),
            label_padding_x: Some(4),
            label_padding_y: Some(0),
            label_border_radius: Some(4),
            label_margin_x: Some(4),
            label_margin_y: Some(2),
        }
    }
}
