use iced::widget::text;
use iced::widget::Text;
use iced::Element;
use iced::Task;

#[derive(Debug, Clone)]
pub enum AppMessage {

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
		text("Hello")
			.into()
	}
}