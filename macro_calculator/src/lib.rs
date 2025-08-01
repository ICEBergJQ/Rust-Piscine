pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_calories: f64 = 0.0;
    let mut total_fats: f64 = 0.0;
    let mut total_carbs: f64 = 0.0;
    let mut total_proteins: f64 = 0.0;

    for food in foods {
        total_calories += food.calories.1.replace("kcal", "").parse::<f64>().unwrap_or(0.0) * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
    }

    json::object! {
        "cals" =>  (total_calories * 100.0 ).round() / 100.0,
        "carbs" => (total_carbs*100.0).round() / 100.0,
        "proteins" => (total_proteins*100.0).round() / 100.0,
        "fats" => (total_fats * 100.0).round() / 100.0,
    }
}