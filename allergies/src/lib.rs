pub struct Allergies {
    binary_array: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGEN_NUM: usize = 8;
const ALLERGIES: [Allergen; ALLERGEN_NUM] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let mut binary_array = vec![0; ALLERGEN_NUM];
        for i in 0..ALLERGEN_NUM {
            binary_array[i] = score % 2;
            score /= 2;
        }

        Self { binary_array }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.binary_array[*allergen as usize] == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGIES
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}
