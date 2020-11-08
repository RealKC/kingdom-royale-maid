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
    memo_book: MemoBook,
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
            memo_book: MemoBook::new(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (Count, Item)> {
        self.items.iter()
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

    pub fn memo_book(&self) -> &MemoBook {
        &self.memo_book
    }

    pub fn memo_book_mut(&mut self) -> &mut MemoBook {
        &mut self.memo_book
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push((1, item))
    }
}

#[derive(Debug)]
pub struct MemoBook {
    notes: Vec<Note>,
    notes_ripped_from_self: u32,
    ripped_notes_gotten_from_others: u32,
}

#[derive(Debug)]
pub struct Note {
    pub text: String,
    pub when: String,
    pub ripped: bool,
}

impl MemoBook {
    const MAX_NOTES: u32 = 128; // PONDER: Should this be configurable per game?

    pub fn new() -> Self {
        Self {
            notes: vec![],
            notes_ripped_from_self: 0,
            ripped_notes_gotten_from_others: 0,
        }
    }

    pub fn add_note(&mut self, text: String, when: String) -> Result<(), String> {
        if self.notes.len() < (Self::MAX_NOTES - self.notes_ripped_from_self) as usize {
            self.notes.push(Note {
                text,
                when,
                ripped: false,
            });
        } else {
            return Err("you can't add any more notes in your memo book".into());
        }

        Ok(())
    }

    pub fn add_ripped_note(&mut self, note: Note) {
        self.ripped_notes_gotten_from_others += 1;
        self.notes.push(Note {
            ripped: true,
            ..note
        });
    }

    pub fn number_of_written_notes(&self) -> usize {
        self.notes.len()
    }

    pub fn get_note(&self, idx: usize) -> Option<&Note> {
        self.notes.get(idx)
    }

    pub fn rip_note(&mut self, idx: usize) -> Option<Note> {
        self.notes_ripped_from_self += 1;
        if idx < (Self::MAX_NOTES - self.notes_ripped_from_self) as usize {
            Some(self.notes.remove(idx))
        } else {
            None
        }
    }
}
