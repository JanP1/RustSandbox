#![allow(dead_code)]

pub mod orders;
pub mod menu;



use orders::Order;
use menu::Menu;


pub struct Restaurant {
    menu: Menu,
    orders: Vec<Order>,
}


impl Restaurant {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            orders: Vec::new(),
        }
    }


    pub fn show_menu(&self){
        self.menu.display();
    }


    pub fn take_order(&mut self, item: String){
        if self.menu.is_available(&item){
            let order = Order::new(item.clone());
            self.orders.push(order);
            println!("Order taken - {}", item);
        } else {
            println!("Sorry, {} is not available on the menu.", item);  
        }
    }

    pub fn process_orders(&self){
        for order in &self.orders {
            println!("Processing order {}", order.item);
        }
    }

}
