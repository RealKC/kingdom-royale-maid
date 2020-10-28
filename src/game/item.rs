use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub edible: bool,
    pub weapon: bool,
}

impl Item {
    pub const FOOD_NAME: &'static str = "Food bar";

    fn food() -> Self {
        Self {
            name: Self::FOOD_NAME.to_string(),
            edible: true,
            weapon: false,
        }
    }

    fn knife() -> Self {
        Self {
            name: "Knife".to_string(),
            edible: false,
            weapon: true,
        }
    }

    fn watch(colour: String) -> Self {
        Self {
            name: format!("{} watch", colour),
            edible: false,
            weapon: false,
        }
    }

    fn tablet() -> Self {
        Self {
            name: "Tablet".to_string(),
            edible: false,
            weapon: false,
        }
    }

    fn ballpoint_pen() -> Self {
        Self {
            name: "Ballpoint pen".to_string(),
            edible: false,
            weapon: false,
        }
    }

    fn memo_book() -> Self {
        Self {
            name: "Memo book".to_string(),
            edible: false,
            weapon: false,
        }
    }
}

type Count = u8;

#[derive(Debug)]
pub struct Items {
    items: Vec<(Count, Item)>,
}

impl Items {
    pub fn new(watch_colour: String) -> Self {
        Self {
            items: vec![
                (7, Item::food()),
                (1, Item::watch(watch_colour)),
                (1, Item::knife()),
                (1, Item::tablet()),
                (1, Item::memo_book()),
                (1, Item::ballpoint_pen()),
            ],
        }
    }

    #[instrument]
    pub fn get_item(&self, item_name: &str) -> &(Count, Item) {
        for item in self.items.iter() {
            if item.1.name.contains(item_name) {
                return item;
            }
        }

        error!("Item::get_item should be called with valid item names only!!!");
        unreachable!();
    }

    #[instrument]
    pub fn get_item_mut(&mut self, item_name: &str) -> &mut (Count, Item) {
        for item in self.items.iter_mut() {
            if item.1.name.contains(item_name) {
                return item;
            }
        }

        error!("Items::get_item_mut should be called with valid item names only!!!");
        unreachable!();
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push((1, item))
    }
}
