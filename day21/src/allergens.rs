

pub struct Label {
    pub ingredients: Vec<String>,
    pub allergens: Vec<String>,
}

impl Label {
    fn new(ing: Vec<String>, allerg: Vec<String>) -> Label {
        Label {
            ingredients: ing,
            allergens: allerg,
        }
    }
}
