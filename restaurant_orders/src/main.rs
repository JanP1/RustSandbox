use restaurant::Restaurant;

mod restaurant; 

fn main() {

    let mut restaurant = Restaurant::new();
    
    restaurant.show_menu();

    restaurant.take_order("Burger".to_string());
    restaurant.take_order("Pizza".to_string());

    restaurant.process_orders();

}
