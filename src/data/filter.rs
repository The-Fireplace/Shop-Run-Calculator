use std::sync::RwLockReadGuard;
use crate::data::{Data, Item, Location, Shop, ShopItem};

#[derive(Clone)]
pub struct Filter {
    location: Option<String>,
    shop: Option<String>,
    item: Option<String>,
    members: Option<bool>,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            location: None,
            shop: None,
            item: None,
            members: None,
        }
    }

    pub fn set_location(&mut self, location: &Location) {
        self.location = Some(location.get_name().to_string());
    }

    pub fn set_shop(&mut self, shop: &str) {
        self.shop = Some(shop.to_string());
    }

    pub fn set_item(&mut self, item: &str) {
        self.item = Some(item.to_string());
    }

    pub fn set_members(&mut self, members: bool) {
        self.members = Some(members);
    }

    pub fn clear_location(&mut self) {
        self.location = None;
    }

    pub fn clear_shop(&mut self) {
        self.shop = None;
    }

    pub fn clear_item(&mut self) {
        self.item = None;
    }

    pub fn clear_members(&mut self) {
        self.members = None;
    }

    pub fn clear_all(&mut self) {
        self.location = None;
        self.shop = None;
        self.item = None;
        self.members = None;
    }

    pub fn get_location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn get_shop(&self) -> Option<&str> {
        self.shop.as_deref()
    }

    pub fn get_item(&self) -> Option<&str> {
        self.item.as_deref()
    }

    pub fn get_members(&self) -> Option<bool> {
        self.members
    }

    fn get_source_locations(&self, data_reader: &RwLockReadGuard<Data>) -> Vec<Location> {
        let mut locations = data_reader.get_all_locations().clone();
        if let Some(location_name) = self.get_location() {
            locations = locations.into_iter().filter(|location| location.get_name() == location_name).collect();
        }
        if let Some(members) = self.get_members() {
            locations = locations.into_iter().filter(|location| location.is_members_only() == members).collect();
        }
        
        locations
    }
    
    fn get_source_items(&self, data_reader: &RwLockReadGuard<Data>) -> Vec<Item> {
        let mut items = data_reader.get_all_items().clone();
        if let Some(item_name) = self.get_item() {
            items = items.into_iter().filter(|item| item.get_name() == item_name).collect();
        }
        if let Some(members) = self.get_members() {
            items = items.into_iter().filter(|item| item.is_members_only() == members).collect();
        }
        
        items
    }
    
    pub fn get_available_shops(&self, data_reader: &RwLockReadGuard<Data>) -> Vec<Shop> {
        let mut shops = data_reader.get_all_shops().clone();
        let source_locations = self.get_source_locations(data_reader);
        let source_items = self.get_source_items(data_reader);
        let source_shop_items: Vec<ShopItem> = data_reader.get_all_shop_items().iter()
            .filter(|shop_item| source_items.iter().any(|item| item.get_id() == shop_item.get_item_id()))
            .cloned()
            .collect();
        shops = shops.into_iter()
            .filter(|shop| source_locations.iter().any(|location| location.get_name() == shop.get_location()))
            .filter(|shop| source_shop_items.iter().any(|shop_item| shop_item.get_shop() == shop.get_name()))
            .collect();
        
        shops
    }
}