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

use std::collections::HashMap;

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct News
{
    folders: HashMap<String, Folder>,
}

impl News
{
    pub fn folders(&mut self) -> &mut HashMap<String, Folder>
    {
        &mut self.folders
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Folder
{
    feeds: HashMap<String, Feed>,
}

impl Folder
{
    pub fn feeds(&mut self) -> &mut HashMap<String, Feed>
    {
        &mut self.feeds
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Feed
{
    title: String,
    description: String,
    podcast: bool,
    image: Option<String>,
    categories: Vec<String>,
    episodes: Vec<Episode>,
}

impl Feed
{
    pub fn title(&self) -> String
    {
        self.title
            .clone()
    }
    pub fn description(&self) -> String
    {
        self.description
            .clone()
    }
    pub fn podcast(&self) -> bool
    {
        self.podcast
    }
    pub fn image(&self) -> Option<String>
    {
        self.image
            .clone()
    }
    pub fn categories(&self) -> Vec<String>
    {
        self.categories
            .clone()
    }
    pub fn episodes(&self) -> Vec<Episode>
    {
        self.episodes
            .clone()
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct Episode
{
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
    pub_date: Option<String>,
    media: Option<String>,
    position: Option<i64>,
    read: bool,
}

impl Episode
{
    pub fn title(&self) -> Option<String>
    {
        self.title
            .clone()
    }
    pub fn link(&self) -> Option<String>
    {
        self.link
            .clone()
    }
    pub fn description(&self) -> Option<String>
    {
        self.description
            .clone()
    }
    pub fn author(&self) -> Option<String>
    {
        self.author
            .clone()
    }
    pub fn pub_date(&self) -> Option<String>
    {
        self.pub_date
            .clone()
    }
    pub fn media(&self) -> Option<String>
    {
        self.media
            .clone()
    }
    pub fn position(&self) -> Option<i64>
    {
        self.position
            .clone()
    }
    pub fn set_position<V>(&mut self,
                           position: V)
    where
        V: Into<Option<i64>>,
    {
        self.position = position.into();
    }
    pub fn read(&self) -> bool
    {
        self.read
    }
    pub fn set_read(&mut self,
                    read: bool)
    {
        self.read = read;
    }
}
