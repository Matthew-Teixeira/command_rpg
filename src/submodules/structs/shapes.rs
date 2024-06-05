#[derive(Debug)]
pub enum Unit {
    Feet,
    Inches,
    Yards,
    Meters,
}
#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub unit: Unit,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f32 {
        self.width * 2.0 + self.height * 2.0
    }

    pub fn feet_to_inches(&mut self) {
        self.width = self.width * 12.0;
        self.height = self.height * 12.0;
        self.unit = self.unit_to_enum("inches");
    }

    pub fn feet_to_yards(&mut self) {
        self.width = self.width / 3.0;
        self.height = self.height / 3.0;
        self.unit = self.unit_to_enum("yards");
    }

    fn unit_to_enum(&self, unit: &str) -> Unit {
        match unit {
            "feet" => Unit::Feet,
            "inches" => Unit::Inches,
            "yards" => Unit::Yards,
            "meters" => Unit::Meters,
            &_ => todo!(),
        }
    }

    /*   pub fn unit_to_string(unit: Unit) -> &'static str {
        match unit {
            Unit::Feet => "feet",
            Unit::Inches => "inches",
            Unit::Yards => "yards",
            Unit::Meters => "meters",
            &_ => todo!()
        }
    } */
}
