//! Just assets bundled in the binary

// NOTE these take around 200K so it's fine to just bundle them
pub const FONT: &[u8] = include_bytes!("../assets/UbuntuMono-Regular.ttf");
pub const FONT_LICENSE: &str = include_str!("../assets/UbuntuMono-Regular.LICENSE.txt");
