#![allow(dead_code)]


pub struct Menu {
    items: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self{
            items: vec![
                "Burger".to_string(),
                "Pizza".to_string(),
                "HotDog".to_string(),
            ]
        }
    }
    


    // Tutaj wyswietla cale menu, musi byc przez referencje, bo nie chcemy
    // tworzyc kopii ani nie chcemy zeby wartosc zostala przejeta przez funkcje
    // i po tym jak funkcja juz jest wywolana wartosc przestalaby istniec

    pub fn display(&self) {
        println!("Menu");
        for item in &self.items {
            println!("- {}", item);
        }
    }



    pub fn is_available(&self, item: &String) -> bool{
        self.items.contains(item)
    }
}

