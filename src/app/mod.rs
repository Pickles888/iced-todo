use iced::{
    executor,
    widget::{button, container, row, scrollable, Column},
    Application, Command, Element, Renderer, Theme,
};
use todo_widgets::{TodoListWidget, TodoMessage};

mod icons;
mod todo_widgets;

pub struct Todo {
    lists: Vec<TodoListWidget>,
    list: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Item(usize, TodoMessage),
    ListBarPressed(usize),
}

impl Application for Todo {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let test = TodoListWidget::new("Test");
        (
            Self {
                lists: vec![test],
                list: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ListBarPressed(list_index) => {
                self.list = list_index;

                Command::none()
            }
            Message::Item(list_index, message) => {
                self.lists.get_mut(list_index).unwrap().update(message)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let todo_lists_bar = container(scrollable(
            Column::from_vec(
                self.lists
                    .iter()
                    .enumerate()
                    .map(|(index, list)| {
                        button(&*list.name)
                            .on_press(Message::ListBarPressed(index))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .padding(10)
            .width(150),
        ));

        let todo_list = self
            .lists
            .get(self.list)
            .unwrap()
            .view()
            .map(|message| Message::Item(self.list, message));

        row![todo_lists_bar, todo_list].into()
    }
}
