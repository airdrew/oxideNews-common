// This file is part of oxideNews-common
//
// Copyright © 2017 red-oxide Developers
//
// his program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//

//! The News object and its builders, setters, and getters.

use std::collections::HashMap;

/// The News object that contains elements from the rss feed per category.
#[derive(Debug, Default, Clone, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct News
{
    folders: HashMap<String, Folder>,
}

impl News
{
    /// Get the folders from the News object.
    pub fn folders(&mut self) -> &mut HashMap<String, Folder>
    {
        &mut self.folders
    }
}

/// Folder object containing the feeds per specified category.
#[derive(Debug, Default, Clone, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct Folder
{
    feeds: HashMap<String, Feed>,
}

impl Folder
{
    /// Get the feeds from the Folder object.
    pub fn feeds(&mut self) -> &mut HashMap<String, Feed>
    {
        &mut self.feeds
    }
}

/// Feed object containing elements from the Rss object.
#[derive(Debug, Default, Clone, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct Feed
{
    title: String,
    description: String,
    podcast: bool,
    image: String,
    categories: Vec<String>,
    episodes: Vec<Episode>,
}

impl Feed
{
    /// Get the Rss feeds title.
    pub fn title(&self) -> String
    {
        self.title
            .clone()
    }
    /// Get the Rss feeds description.
    pub fn description(&self) -> String
    {
        self.description
            .clone()
    }
    /// Get a flag denoting if this feed is for podcasts.
    pub fn podcast(&self) -> bool
    {
        self.podcast
    }
    /// Get the Rss feeds image link.
    pub fn image(&self) -> String
    {
        self.image
            .clone()
    }
    /// Get the Rss feeds categories.
    pub fn categories(&self) -> Vec<String>
    {
        self.categories
            .clone()
    }
    /// Get the Rss feeds items contained within the Episode object.
    pub fn episodes(&self) -> Vec<Episode>
    {
        self.episodes
            .clone()
    }
}

/// Episode object containing elements from the Rss Item object.
#[derive(Debug, Default, Clone, Deserialize, Serialize, Builder)]
#[builder(setter(into), default)]
pub struct Episode
{
    title: String,
    link: String,
    description: String,
    author: String,
    pub_date: String,
    media: String,
    position: i64,
    read: bool,
}

impl Episode
{
    /// Get the Rss Item title.
    pub fn title(&self) -> String
    {
        self.title
            .clone()
    }
    /// Get the Rss Item link.
    pub fn link(&self) -> String
    {
        self.link
            .clone()
    }
    /// Get the Rss Item description.
    pub fn description(&self) -> String
    {
        self.description
            .clone()
    }
    /// Get the Rss Item author.
    pub fn author(&self) -> String
    {
        self.author
            .clone()
    }
    /// Get the Rss Item pub date.
    pub fn pub_date(&self) -> String
    {
        self.pub_date
            .clone()
    }
    /// Get the Rss Item enclosure link.
    pub fn media(&self) -> String
    {
        self.media
            .clone()
    }
    /// Get the position of the podcast.
    pub fn position(&self) -> i64
    {
        self.position
    }
    /// Set the position of the podcast.
    pub fn set_position(&mut self,
                        position: i64)
    {
        self.position = position.into();
    }
    /// Get the flag denoting if the item has been read or podcast listened to.
    pub fn read(&self) -> bool
    {
        self.read
    }
    /// Set the flag noting if the item has been read or podcast listened to.
    pub fn set_read(&mut self,
                    read: bool)
    {
        self.read = read;
    }
}
