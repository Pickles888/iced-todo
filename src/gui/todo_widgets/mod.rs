use filter::Filter;
use todo_item::ItemMessage;

mod filter;
pub mod todo_item;
pub mod todo_list;

#[derive(Debug, Clone)]
pub enum TodoMessage {
    Item(usize, ItemMessage),
    InputEdit(String),
    SetFilter(Filter),
    NewSubmitted,
}
