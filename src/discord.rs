use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize)]
struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct Thumbnail {
    url: String
}

#[derive(Debug, Clone, Serialize)]
struct EmbedEntry {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub author: Option<EmbedAuthor>,
    pub thumbnail: Option<Thumbnail>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Embed {
    username: Option<String>,
    content: Option<String>,
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<EmbedEntry>
}

impl Embed {
    pub fn new() -> Self {
        Self {
            content: None,
            username: None,
            avatar_url: None,
            embeds: Vec::new(),
        }
    }

    fn ensure_embed_entry(&mut self) {
        if self.embeds.is_empty() {
            self.embeds.push(EmbedEntry {
                title: None,
                description: None,
                url: None,
                color: None,
                fields: Vec::new(),
                footer: None,
                author: None,
                thumbnail: None,
                image: None,
            });
        }
    }

    pub fn username<T: Into<String>>(mut self, username: T) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn content<T: Into<String>>(mut self, content: T) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn avatar_url<T: Into<String>>(mut self, avatar_url: T) -> Self {
        self.avatar_url = Some(avatar_url.into());
        self
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].title = Some(title.into());
        self
    }

    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].description = Some(description.into());
        self
    }

    pub fn url<T: Into<String>>(mut self, url: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].url = Some(url.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].color = Some(color);
        self
    }

    pub fn field<T: Into<String>, U: Into<String>>(mut self, name: T, value: U, inline: bool) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    pub fn footer<T: Into<String>>(mut self, text: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: None,
        });
        self
    }

    pub fn footer_with_icon<T: Into<String>, U: Into<String>>(mut self, text: T, icon_url: U) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: Some(icon_url.into()),
        });
        self
    }

    pub fn author<T: Into<String>>(mut self, name: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].author = Some(EmbedAuthor {
            name: name.into(),
            url: None,
            icon_url: None,
        });
        self
    }

    pub fn author_with_url<T: Into<String>, U: Into<String>>(mut self, name: T, url: U) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].author = Some(EmbedAuthor {
            name: name.into(),
            url: Some(url.into()),
            icon_url: None,
        });
        self
    }

    pub fn author_with_icon<T: Into<String>, U: Into<String>>(mut self, name: T, icon_url: U) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].author = Some(EmbedAuthor {
            name: name.into(),
            url: None,
            icon_url: Some(icon_url.into()),
        });
        self
    }

    pub fn thumbnail<T: Into<String>>(mut self, url: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].thumbnail = Some(Thumbnail {
            url: url.into()
        });
        self
    }

    pub fn image<T: Into<String>>(mut self, url: T) -> Self {
        self.ensure_embed_entry();
        self.embeds[0].image = Some(url.into());
        self
    }
}

impl Default for Embed {
    fn default() -> Self {
        Self::new()
    }
}

impl Embed {
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const YELLOW: u32 = 0xFFFF00;
    pub const PURPLE: u32 = 0x800080;
    pub const ORANGE: u32 = 0xFFA500;
    pub const PINK: u32 = 0xFFC0CB;
}
