use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub edible: bool,
    pub weapon: bool,
}

impl Item {
    fn food() -> Self {
        Self {
            name: "Food bar".to_string(),
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
}

type Count = u8;

#[derive(Debug)]
pub struct Items {
    items: Vec<(Count, Item)>,
}

impl Items {
    pub fn new() -> Self {
        Self {
            items: vec![
                (7, Item::food()),
                (1, Item::watch("".to_string())),
                (1, Item::knife()),
                (1, Item::tablet()),
            ],
        }
    }

    #[instrument]
    pub fn get_item(&self, item_name: &str) -> &(Count, Item) {
        for item in self.items.iter() {
            if item.1.name == item_name {
                return item;
            }
        }

        error!("Item::get_item should be called with valid item names only!!!");
        unreachable!();
    }

    #[instrument]
    pub fn get_item_mut(&mut self, item_name: &str) -> &mut (Count, Item) {
        for item in self.items.iter_mut() {
            if item.1.name == item_name {
                return item;
            }
        }

        error!("Items::get_item_mut should be called with valid item names only!!!");
        unreachable!();
    }
}
