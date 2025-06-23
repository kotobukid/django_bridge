pub type ColorThemeStatic = (&'static str, &'static str, &'static str, &'static str);
pub const COLOR_THEMES: &[ColorThemeStatic; 6] = &[("White", "#fff1b4", "#f5d872", "#fffdf0"),
("Blue", "#b4ceff", "#6b9eff", "#e6f0ff"),
("Red", "#ffb4b4", "#ff7a7a", "#ffe6e6"),
("Black", "rgb(176, 150, 255)", "rgb(139, 101, 255)", "rgb(225, 217, 255)"),
("Green", "#ccffb4", "#8eff66", "#e8ffe0"),
("Colorless", "#cfcfcf", "#a0a0a0", "#f0f0f0"),];