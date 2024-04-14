use std::sync::{Arc, RwLock};

pub type Database = Arc<RwLock<Data>>;

#[derive(Clone)]
pub struct Data {
    locations: Vec<Location>,
    shops: Vec<Shop>,
    items: Vec<Item>,
    shop_items: Vec<ShopItem>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            locations: vec![Location {
                name: "Varrock".to_string(),
                members_only: false,
            }, Location {
                name: "Falador".to_string(),
                members_only: false,
            }, Location {
                name: "Ardougne".to_string(),
                members_only: true,
            }],
            shops: vec![Shop {
                location: "Varrock".to_string(),
                name: "General Store".to_string(),
                members_only: false,
            }],
            items: vec![Item {
                id: 1,
                name: "Bronze Dagger".to_string(),
                ge_price: 15,
                members_only: false,
                stackable: false,
                demand: Demand::High,
            }],
            shop_items: vec![ShopItem {
                shop: "General Store".to_string(),
                item_id: 1,
                price: 2,
                stock: 10,
            }]
        }
    }
    
    pub fn get_all_locations(&self) -> &Vec<Location> {
        &self.locations
    }
    
    pub fn get_all_shops(&self) -> &Vec<Shop> {
        &self.shops
    }
    
    pub fn get_all_items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[derive(Clone)]
pub struct Location {
    name: String,
    members_only: bool,
}

impl Location {
    pub fn new(name: String, members_only: bool) -> Location {
        Location {
            name,
            members_only,
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn is_members_only(&self) -> bool {
        self.members_only
    }
}

#[derive(Clone)]
pub struct Shop {
    name: String,
    location: String,
    members_only: bool,
}

#[derive(Clone)]
pub enum Demand {
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Clone)]
pub struct Item {
    name: String,
    id: u32,
    ge_price: u32,
    members_only: bool,
    stackable: bool,
    demand: Demand,
}

#[derive(Clone)]
pub struct ShopItem {
    shop: String,
    item_id: u32,
    price: u32,
    stock: u32,
}