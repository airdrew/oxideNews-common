extern crate oxide_news_common;

use oxide_news_common::Common;
use std::env;
use std::fs;

#[test]
fn test_empty()
{
    let url = "https://latenightlinux.com/feed/mp3";
    let folder_name = "podcasts";

    let home_path = env::home_dir()
        .expect("Error: cannot get the home directory.")
        .to_str()
        .expect("Error: cannon unwrap home path.")
        .to_string();

    let config_path = home_path + "/.config/oxideNews/test";

    let common = Common::init(config_path.as_str())
        .unwrap()
        .add(url,
             folder_name,
             true);

    let common_uw = common.expect("Error unwrapping common");

    let mut news = common_uw.clone()
                            .news();

    assert!(!news.folders()
                 .is_empty());

    let folders = news.folders();

    assert!(!folders.is_empty());

    let folder = folders.get_mut(folder_name)
                        .expect("Error unwrapping folder");

    let feed = folder.feeds()
                     .get_mut(url)
                     .expect("Error unwrapping feed");

    assert!(feed.podcast());
    assert_eq!(feed.title(),
               "Late Night Linux (MP3)"
                   .to_owned());
    assert_eq!(feed.description(),
               "Linux after dark"
                   .to_owned());
    assert!(feed.categories()
                .is_empty());
    assert!(!feed.episodes()
                 .is_empty());

    let rm_common = common_uw.clone()
                             .remove(url);

    let rm_common_uw = rm_common.expect("Error unwrapping rm_common");

    let mut rm_news = rm_common_uw.clone()
                                  .news();

    let rm_folders = rm_news.folders();

    assert!(rm_folders.is_empty());

    rm_common_uw.clone()
                .close()
                .expect("Error unwrapping close");

    let file = config_path + "/oxideNews.ron";
    fs::remove_file(file.as_str())
        .expect("Error unwrapping remove file");
}
