use iced::widget::{button, column, row, text};
use iced::Theme;
use iced::{executor, Alignment, Application, Command, Element};

use crate::types::feeds::FeedMeta;

use super::widgets::{Feed, Feeds};

pub struct AppLayout {
    value: i32,
    feeds: Option<Feeds>,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    FeedsFound(Result<Vec<FeedMeta>, String>),
}

impl Application for AppLayout {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                feeds: None,
            },
            Command::perform(Feeds::fetch_feeds(), Message::FeedsFound),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
            Message::DecrementPressed => {
                self.value -= 1;
                Command::none()
            }
            Message::FeedsFound(feeds) => match feeds {
                Err(_) => Command::none(),
                Ok(data) => {
                    let feed_list = data
                        .iter()
                        .map(|n| Feed::new(n.feed_url.to_owned()))
                        .collect();
                    self.feeds = Some(Feeds::new(feed_list));
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let column = column![].padding(20).align_items(Alignment::Center);
        match self.feeds.as_ref() {
            Some(feeds) => row![text("Left Column"), column.push(feeds.view())].into(),
            None => row![text("Left Column"), column].into(),
        }
    }
}

// impl AppLayout {
//     fn render_feeds(feeds: &Vec<FeedMeta>) -> Element<Message> {
//         feeds
//             .iter()
//             .fold(Column::new().spacing(10), |col, content| {
//                 col.push(text(content.to_owned().feed_url))
//             })
//             .into()
//     }
// }
