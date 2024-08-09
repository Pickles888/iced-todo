use crate::icon::Icon;

pub struct TodoItem {
    completed: bool,
    name: String,
    icon: Option<Icon>,
    important: bool,
}
