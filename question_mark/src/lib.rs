pub struct Four {
    pub fourth_layer: Option<u16>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct One {
    pub first_layer: Option<Two>,
}
impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer
            .and_then(|two| two.second_layer)
            .and_then(|three| three.third_layer)
            .and_then(|four| four.fourth_layer)
    }
}
