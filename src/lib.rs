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

#![warn(missing_docs)]

extern crate ron;
extern crate rss;
#[macro_use]
extern crate derive_builder;
extern crate serde;
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
use std::fs::File;
use std::io::prelude::*;

pub struct Common
{
    filename: String,
    news: News,
}

impl Common
{
    pub fn init(&self,
                dir: &str)
        -> Result<Common, Error>
    {
        let filename = format!("{}/oxideNews.ron",
                               dir);

        let news = match File::open(filename.clone()) {
            Ok(file) => {
                let mut file = file;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                from_str(content.as_str())?
            }
            Err(_) => {
                NewsBuilder::default()
                    .build()
                    .unwrap()
            }
        };

        Ok(Common { filename: filename.clone(),
                    news: news, })
    }
    pub fn add(&mut self,
               url: &str,
               folder_name: &str,
               podcast: bool)
        -> Result<i64, Error>
    {
        let feed = Channel::from_url(url)?;

        let title = feed.title();
        let link = feed.link();
        let description = feed.description();
        let image = match feed.image() {
            None => None,
            Some(x) => Some(String::from(x.url())),
        };

        let mut categories = Vec::new();
        for category in feed.categories() {
            categories.push(String::from(category.name()));
        }

        let mut episodes = Vec::new();
        for item in feed.items() {
            let i_title = match item.title() {
                None => None,
                Some(x) => Some(String::from(x)),
            };
            let i_link = match item.link() {
                None => None,
                Some(x) => Some(String::from(x)),
            };
            let i_description = match item.description() {
                None => None,
                Some(x) => Some(String::from(x)),
            };
            let i_author = match item.author() {
                None => None,
                Some(x) => Some(String::from(x)),
            };
            let i_pub_date = match item.pub_date() {
                None => None,
                Some(x) => Some(String::from(x)),
            };
            let i_media = match item.enclosure() {
                None => None,
                Some(x) => Some(String::from(x.url())),
            };

            episodes.push(EpisodeBuilder::default()
                              .read(false)
                              .position(None)
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

        match self.news
                    .clone()
                    .folders()
                    .get(folder_name) {
            None => {
                let mut feeds: HashMap<String, Feed> = HashMap::new();
                feeds.insert(String::from(feed.link()),
                             news_feed);

                let folder = FolderBuilder::default()
                    .feeds(feeds)
                    .build()
                    .unwrap();

                self.news
                    .clone()
                    .folders()
                    .insert(String::from(folder_name),
                            folder);
            }
            Some(x) => {
                x.clone()
                 .feeds()
                 .insert(String::from(feed.link()),
                         news_feed);
            }
        }

        Ok(0)
    }

    pub fn update(&mut self) -> Result<i64, Error>
    {
        for folder in self.news
                          .clone()
                          .folders()
                          .values_mut()
        {
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

                    for ep in feed.episodes()
                                  .iter_mut()
                    {
                        if ep.title() == title && ep.description() == description {
                            flag = true;
                        }
                    }

                    if !flag {
                        let i_title = match item.title() {
                            None => None,
                            Some(x) => Some(String::from(x)),
                        };
                        let i_link = match item.link() {
                            None => None,
                            Some(x) => Some(String::from(x)),
                        };
                        let i_description = match item.description() {
                            None => None,
                            Some(x) => Some(String::from(x)),
                        };
                        let i_author = match item.author() {
                            None => None,
                            Some(x) => Some(String::from(x)),
                        };
                        let i_pub_date = match item.pub_date() {
                            None => None,
                            Some(x) => Some(String::from(x)),
                        };
                        let i_media = match item.enclosure() {
                            None => None,
                            Some(x) => Some(String::from(x.url())),
                        };

                        feed.episodes()
                            .push(EpisodeBuilder::default()
                                      .read(false)
                                      .position(None)
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
        Ok(0)
    }

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
                for ep in feed.episodes()
                              .iter_mut()
                {
                    if ep.title() == episode.title() && ep.description() == episode.description() {
                        ep.set_position(position);
                    }
                }
            }
        }
    }

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
                for ep in feed.episodes()
                              .iter_mut()
                {
                    if ep.title() == episode.title() && ep.description() == episode.description() {
                        ep.set_read(read);
                    }
                }
            }
        }
    }

    pub fn remove(&mut self,
                  url: &str)
        -> Result<i64, Error>
    {
        for folder in self.news
                          .clone()
                          .folders()
                          .values_mut()
        {
            folder.feeds()
                  .remove(url);
        }
        Ok(0)
    }

    pub fn close(&self) -> Result<i64, Error>
    {
        let filename = self.filename
                           .clone();
        let mut file = File::create(filename)?;

        let news = to_string(&self.news)?;

        file.write_all(news.as_bytes())?;
        Ok(0)
    }
}
