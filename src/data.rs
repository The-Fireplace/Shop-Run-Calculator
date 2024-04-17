use std::sync::{Arc, RwLock};
use crate::data::filter::Filter;

pub mod filter;

pub type Database = Arc<RwLock<Data>>;

#[derive(Clone)]
pub struct Data {
    locations: Vec<Location>,
    shops: Vec<Shop>,
    items: Vec<Item>,
    shop_items: Vec<ShopItem>,
    filter: Filter,
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
            }],
            filter: Filter::new(),
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
    
    pub fn get_all_shop_items(&self) -> &Vec<ShopItem> {
        &self.shop_items
    }
    
    pub fn get_filter(&self) -> &Filter {
        &self.filter
    }
    
    pub fn get_mut_filter(&mut self) -> &mut Filter {
        &mut self.filter
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

impl Shop {
    pub fn new(name: String, location: String, members_only: bool) -> Shop {
        Shop {
            name,
            location,
            members_only,
        }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_location(&self) -> &str {
        &self.location
    }
    
    pub fn is_members_only(&self) -> bool {
        self.members_only
    }
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

impl Item {
    pub fn new(name: String, id: u32, ge_price: u32, members_only: bool, stackable: bool, demand: Demand) -> Item {
        Item {
            name,
            id,
            ge_price,
            members_only,
            stackable,
            demand,
        }
    }
    
    pub fn get_id(&self) -> u32 {
        self.id
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_ge_price(&self) -> u32 {
        self.ge_price
    }
    
    pub fn is_members_only(&self) -> bool {
        self.members_only
    }
    
    pub fn is_stackable(&self) -> bool {
        self.stackable
    }
    
    pub fn get_demand(&self) -> &Demand {
        &self.demand
    }
}

#[derive(Clone)]
pub struct ShopItem {
    shop: String,
    item_id: u32,
    price: u32,
    stock: u32,
}

impl ShopItem {
    pub fn new(shop: String, item_id: u32, price: u32, stock: u32) -> ShopItem {
        ShopItem {
            shop,
            item_id,
            price,
            stock,
        }
    }
    
    pub fn get_shop(&self) -> &str {
        &self.shop
    }
    
    pub fn get_item_id(&self) -> u32 {
        self.item_id
    }
    
    pub fn get_price(&self) -> u32 {
        self.price
    }
    
    pub fn get_stock(&self) -> u32 {
        self.stock
    }
}