use serde::{Deserialize, Serialize};
/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypeweather>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: Box<str>,
    /// A color of the area background in the ARGB format
    pub background_color: i64,
}
impl StoryAreaTypeWeather {
    /// Creates a new `StoryAreaTypeWeather`.
    ///
    /// # Arguments
    /// * `temperature` - Temperature, in degree Celsius
    /// * `emoji` - Emoji representing the weather
    /// * `background_color` - A color of the area background in the ARGB format
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<Box<str>>, T2: Into<i64>>(
        temperature: T0,
        emoji: T1,
        background_color: T2,
    ) -> Self {
        Self {
            temperature: temperature.into(),
            emoji: emoji.into(),
            background_color: background_color.into(),
        }
    }

    /// Temperature, in degree Celsius
    #[must_use]
    pub fn temperature<T: Into<f64>>(mut self, val: T) -> Self {
        self.temperature = val.into();
        self
    }

    /// Emoji representing the weather
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = val.into();
        self
    }

    /// A color of the area background in the ARGB format
    #[must_use]
    pub fn background_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.background_color = val.into();
        self
    }
}
