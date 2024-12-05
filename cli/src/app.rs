use iced::futures::stream::unfold;
use iced::stream;
use iced::widget::button;
use iced::widget::text;
use iced::widget::Text;
use iced::widget::column;
use iced::Element;
use iced::Subscription;
use iced::Task;
use tokio::sync::mpsc;

// pub fn channel_subscription(
//     mut receiver: mpsc::Receiver<String>,
// ) -> Subscription<String> {
// 	stream::channel(100, f)

//     unfold((), (), move |()| async {
//         match receiver.recv().await {
//             Some(message) => Some((message, ())),
//             None => None,
//         }
//     })
// }

#[derive(Debug, Clone)]
pub enum AppMessage {
	Test
}

pub struct App {

}

impl App {
	pub fn new() -> (Self, Task<AppMessage>) {
		(Self {}, Task::none())
	}

	pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
		Task::none()
	}

	pub fn view(&self) -> Element<AppMessage> {
		column![
			text("Peers")
			.size(40),
			button("Test").on_press(AppMessage::Test)
		].into()
	}

	pub fn subscription(&self) -> Subscription<AppMessage> {
		println!("subscription");
		Subscription::none()
	}
}