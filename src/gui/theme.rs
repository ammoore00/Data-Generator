use iced::application;
use iced::application::Appearance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApplicationTheme {
    #[default]
    Default
}

impl application::StyleSheet for ApplicationTheme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> Appearance {
        todo!()
    }
}