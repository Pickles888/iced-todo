use super::TodoItemWidget;

#[derive(Debug, Clone)]
pub enum Filter {
    All,
    Uncomplete,
    Completed,
}

impl Filter {
    pub fn match_filter(&self, todoitem: TodoItemWidget) -> bool {
        match *self {
            Filter::All => true,
            Filter::Uncomplete => !todoitem.completed,
            Filter::Completed => todoitem.completed,
        }
    }
}
