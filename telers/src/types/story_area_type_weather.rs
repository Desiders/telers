use serde::Serialize;

/// Describes a story area containing weather information. Currently, a story can have up to 3 weather areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypeweather>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
    /// A color of the area background in the ARGB format
    pub background_color: i32,
}

impl StoryAreaTypeWeather {
    pub fn new(temperature: f64, emoji: String, background_color: i32) -> Self {
        Self {
            temperature,
            emoji,
            background_color,
        }
    }

    pub fn temperature(self, val: f64) -> Self {
        Self {
            temperature: val,
            ..self
        }
    }

    pub fn emoji(self, val: String) -> Self {
        Self { emoji: val, ..self }
    }

    pub fn background_color(self, val: i32) -> Self {
        Self {
            background_color: val,
            ..self
        }
    }
}
