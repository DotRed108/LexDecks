#[derive(Clone)]
pub struct Shadow {
    pub color: Color,
    pub color_intensity: u8,
    pub inset: bool,
    pub horizontal_offset: String,
    pub vertical_offset: String,
    pub blur_radius: String,
    pub spread_radius: String,
    pub shadow: Option<Box<Shadow>>,
}

impl Shadow {
    pub fn new(color: Color, horizontal_offset: impl ToString, vertical_offset: impl ToString, blur_radius: impl ToString) -> Self {

        Self { 
            color,
            horizontal_offset: horizontal_offset.to_string(),
            vertical_offset: vertical_offset.to_string(),
            blur_radius: blur_radius.to_string(),
            color_intensity: 100,
            ..Default::default()
        }
    }

    pub fn light() -> Self {
        let mut light_shadow = Shadow::new(Color::Winter2, "2px", "2px", "1px");
        light_shadow.color_intensity = 60;
        return light_shadow;
    }

    
    pub fn dark() -> Self {
        let mut dark_shadow = Shadow::new(Color::MidnightBlack, 0, "1px", "1px");
        dark_shadow.color_intensity = 40;
        return dark_shadow;
    }

    pub fn add_shadow(&mut self, new_shadow: Shadow) -> Shadow {
        match &self.shadow {
            Some(box_of_shadow) => {
                let mut old_shadow = box_of_shadow.clone();
                self.shadow = Some(Box::new(old_shadow.add_shadow(new_shadow)));
            },
            None => {self.shadow = Some(Box::new(new_shadow));},
        }

        return self.clone()
    }

    pub fn css(&self) -> String {
        let mut css = if self.inset {
            "inset ".to_string()
        } else {
            "".to_string()
        };
        css.push_str(&format!("{} {} {} {} {}",self.horizontal_offset, self.vertical_offset, self.blur_radius, self.spread_radius, self.color.rgba(self.color_intensity)));
        
        let shadow = match self.shadow.clone() {
            Some(bubububox) => *bubububox,
            None => return css,
        };

        css.push_str(", ");
        css.push_str(&shadow.css());
        return css;
    }

    pub fn surrounding_shadow(color: Color, css_size: &str) -> Shadow {
        let negative = &format!("calc(-1 *{css_size})");
        let mut shadow = Shadow::new(color, css_size, css_size, 0);
        let shadow2 = Shadow::new(color, css_size, negative, 0);
        let shadow3 = Shadow::new(color, negative, css_size, 0);
        let shadow4 = Shadow::new(color, negative, negative, 0);

        shadow.add_shadow(shadow2);
        shadow.add_shadow(shadow3);
        shadow.add_shadow(shadow4);

        shadow
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self { 
            color: Default::default(), 
            color_intensity: 100, 
            inset: Default::default(),
            horizontal_offset: Default::default(),
            vertical_offset: Default::default(),
            blur_radius: Default::default(),
            spread_radius: Default::default(),
            shadow: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Color {
    Mint,
    MidnightBlack,
    FrenchGray,
    Winter1,
    Winter2,
    Winter3,
    Winter4,
    Red,
    LightGray,
    OffWhite,
    DarkSlate,
    #[default]
    White,
    Jonquil,
}

impl Color {
    pub fn hex(&self) -> String {
        let hex = match self{
            Color::Mint => "#15F5BA",
            Color::MidnightBlack => "#25282B",
            Color::FrenchGray => "#B5BEC6",
            Color::Winter1 => "#C5FFF8",
            Color::Winter2 => "#96EFFF",
            Color::Winter3 => "#5FBDFF",
            Color::Winter4 => "#7B66FF",
            Color::Red => "#A30B37",
            Color::LightGray => "#eee",
            Color::OffWhite => "#F4F4F4",
            Color::DarkSlate => "#416163",
            Color::White => "#FFFFFF",
            Color::Jonquil => "#F9C80E",
        };
        hex.to_string()
    }

    pub fn rgb(&self) -> String {
        let rgb = match self {
            Color::Mint => "21, 245, 186",
            Color::MidnightBlack => "37, 40, 43",
            Color::FrenchGray => "181, 190, 198",
            Color::Winter1 => "197, 255, 248",
            Color::Winter2 => "150, 239, 255",
            Color::Winter3 => "95, 189, 255",
            Color::Winter4 => "123, 102, 255",
            Color::Red => "255, 10, 35",
            Color::LightGray => "238, 238, 238",
            Color::OffWhite => "244, 244, 244",
            Color::DarkSlate => "65, 97, 99",
            Color::White => "255, 255, 255",
            Color::Jonquil => "249, 200, 14",
        };
        rgb.to_string()
    }

    pub fn rgba(&self, percentage: u8) -> String {
        let intensity = if percentage > 100 {
            eprintln!("Percentage must be between 1, 100");
            100
        } else {
            percentage
        };

        let intensity = intensity as f32 / 100.0;

        return format!("rgba({}, {})", self.rgb(), intensity);
    }
}
