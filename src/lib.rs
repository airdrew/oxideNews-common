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

//! The common library to be used by all ui rss reader.

#![warn(missing_docs)]

extern crate ron;
extern crate rss;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod news;

use errors::Error;
use news::{Episode, EpisodeBuilder, Feed, FeedBuilder, FolderBuilder, News, NewsBuilder};
use ron::de::from_str;
use ron::ser::pretty::to_string;
use rss::Channel;
use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

/// Common object containing the dir path to the oxideNews ron file and the News object contained within.
#[derive(Clone, Debug)]
pub struct Common
{
    filename: String,
    news: News,
}

impl Common
{
    /// Initialize the oxideNews ron file based on the path.
    pub fn init(dir: &str) -> Result<Common, Error>
    {
        if !Path::new(dir)
            .exists()
        {
            DirBuilder::new()
                .recursive(true)
                .create(dir)?;
        }

        let filename = format!("{}/oxideNews.ron",
                               dir);

        let path = Path::new(&filename);

        let news: News = match File::open(path) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut content = String::new();
                buf_reader.read_to_string(&mut content)?;
                from_str(content.as_str())?
            }
            Err(_) => {
                File::create(path)?;
                NewsBuilder::default()
                    .folders(HashMap::new())
                    .build()
                    .unwrap()
            }
        };

        Ok(Common { filename: filename.clone(),
                    news: news, })
    }
    /// Add a feed to the the oxideNews ron file.
    pub fn add(&mut self,
               url: &str,
               folder_name: &str,
               podcast: bool)
        -> Result<Common, Error>
    {
        let feed = Channel::from_url(url)?;

        let title = feed.title();
        let description = feed.description();
        let image = match feed.image() {
            None => String::new(),
            Some(x) => String::from(x.url()),
        };

        let mut categories = Vec::new();
        for category in feed.categories() {
            categories.push(String::from(category.name()));
        }

        let mut episodes = Vec::new();
        for item in feed.items() {
            let i_title = match item.title() {
                None => String::new(),
                Some(x) => String::from(x),
            };
            let i_link = match item.link() {
                None => String::new(),
                Some(x) => String::from(x),
            };
            let i_description = match item.description() {
                None => String::new(),
                Some(x) => String::from(x),
            };
            let i_author = match item.author() {
                None => String::new(),
                Some(x) => String::from(x),
            };
            let i_pub_date = match item.pub_date() {
                None => String::new(),
                Some(x) => String::from(x),
            };
            let i_media = match item.enclosure() {
                None => String::new(),
                Some(x) => String::from(x.url()),
            };

            episodes.push(EpisodeBuilder::default()
                              .read(false)
                              .position(0)
                              .title(i_title)
                              .link(i_link)
                              .description(i_description)
                              .author(i_author)
                              .pub_date(i_pub_date)
                              .media(i_media)
                              .build()
                              .unwrap());
        }

        let news_feed = FeedBuilder::default()
            .title(title)
            .description(description)
            .image(image)
            .categories(categories)
            .episodes(episodes)
            .podcast(podcast)
            .build()
            .unwrap();

        let folders = self.news
                          .clone()
                          .folders()
                          .clone();
        let news = match folders.get(folder_name) {
            None => {
                let mut feeds: HashMap<String, Feed> = HashMap::new();
                feeds.insert(String::from(url),
                             news_feed);

                let folder = FolderBuilder::default()
                    .feeds(feeds)
                    .build()
                    .unwrap();

                let mut folders = folders.clone();
                folders.insert(String::from(folder_name),
                               folder);

                NewsBuilder::default()
                    .folders(folders)
                    .build()
                    .unwrap()
            }
            Some(x) => {
                let mut folder = x.clone();
                let mut feeds = folder.feeds()
                                      .clone();

                feeds.insert(String::from(url),
                             news_feed);

                NewsBuilder::default()
                    .folders(folders.clone())
                    .build()
                    .unwrap()
            }
        };

        Ok(Common { filename: self.filename
                                  .clone(),
                    news: news.clone(), })
    }
    /// Update all subscribed feeds in the oxideNews ron file.
    pub fn update(&mut self) -> Result<Common, Error>
    {
        let mut news_a = self.news
                             .clone();
        let mut folders = news_a.folders()
                                .clone();
        for folder in folders.values_mut() {
            for (link, feed) in folder.feeds()
                                      .iter_mut()
            {
                let chan = Channel::from_url(link)?;
                for item in chan.items() {
                    let mut flag = false;

                    let title = match item.title() {
                        None => None,
                        Some(x) => Some(String::from(x)),
                    };
                    let description = match item.description() {
                        None => None,
                        Some(x) => Some(String::from(x)),
                    };

                    for ep in &mut feed.episodes() {
                        let e_title = match title.clone() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };

                        let e_desc = match description.clone() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };

                        if ep.title() == e_title && ep.description() == e_desc {
                            flag = true;
                        }
                    }

                    if !flag {
                        let i_title = match item.title() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };
                        let i_link = match item.link() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };
                        let i_description = match item.description() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };
                        let i_author = match item.author() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };
                        let i_pub_date = match item.pub_date() {
                            None => String::new(),
                            Some(x) => String::from(x),
                        };
                        let i_media = match item.enclosure() {
                            None => String::new(),
                            Some(x) => String::from(x.url()),
                        };

                        feed.episodes()
                            .push(EpisodeBuilder::default()
                                      .read(false)
                                      .position(0)
                                      .title(i_title)
                                      .link(i_link)
                                      .description(i_description)
                                      .author(i_author)
                                      .pub_date(i_pub_date)
                                      .media(i_media)
                                      .build()
                                      .unwrap());
                    }
                }
            }
        }

        let news = NewsBuilder::default()
            .folders(folders)
            .build()
            .unwrap();

        Ok(Common { filename: self.filename
                                  .clone(),
                    news: news, })
    }
    /// Set the time position for the podcast in the episode in the oxideNews ron file.
    pub fn set_position(&mut self,
                        episode: Episode,
                        position: i64)
    {
        for folder in self.news
                          .clone()
                          .folders()
                          .values_mut()
        {
            for feed in folder.feeds()
                              .values_mut()
            {
                for ep in &mut feed.episodes() {
                    if ep.title() == episode.title() && ep.description() == episode.description() {
                        ep.set_position(position);
                    }
                }
            }
        }
    }
    /// Set the episode as read in the oxideNews ron file.
    pub fn mark_read(&mut self,
                     episode: Episode,
                     read: bool)
    {
        for folder in self.news
                          .clone()
                          .folders()
                          .values_mut()
        {
            for feed in folder.feeds()
                              .values_mut()
            {
                for ep in &mut feed.episodes() {
                    if ep.title() == episode.title() && ep.description() == episode.description() {
                        ep.set_read(read);
                    }
                }
            }
        }
    }
    /// Remove a feed from the the oxideNews ron file.
    pub fn remove(&mut self,
                  url: &str)
        -> Result<Common, Error>
    {
        let mut folders_a = self.news
                                .clone()
                                .folders()
                                .clone();

        let mut key: Option<&str> = None;
        for (text, folder) in &mut folders_a {
            folder.feeds()
                  .remove(url);

            if folder.feeds()
                     .is_empty()
            {
                key = Some(text);
            }
        }

        let mut folders_b = self.news
                                .clone()
                                .folders()
                                .clone();

        if key.is_some() {
            folders_b.remove(key.unwrap());
        }

        let news = NewsBuilder::default()
            .folders(folders_b.clone())
            .build()
            .unwrap();

        Ok(Common { filename: self.filename
                                  .clone(),
                    news: news, })
    }
    /// Close and write to the file system.
    pub fn close(&self) -> Result<i64, Error>
    {
        let filename = self.filename
                           .clone();
        let mut file = File::create(filename)?;

        let news = to_string(&self.news)?;

        file.write_all(news.as_bytes())?;
        Ok(0)
    }
    /// Retrieve the News.
    pub fn news(self) -> News
    {
        self.news
    }
}
