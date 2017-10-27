extern crate oxide_news_common;

use oxide_news_common::Common;
use std::env;
use std::fs;

#[test]
fn test_existing()
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
    common.unwrap()
          .close()
          .expect("Error unwrapping close");

    let new_common = Common::init(config_path.as_str())
        .unwrap()
        .add("http://feeds.feedburner.com/linuxunplugged",
             "podcasts",
             true);
    assert!(new_common.is_ok());

    let new_common_uw = new_common.unwrap();

    let rm_common = new_common_uw.clone()
                                 .remove(url);

    let rm_common_uw = rm_common.unwrap();

    rm_common_uw.clone()
                .close()
                .expect("Error unwrapping close");

    let file = config_path + "/oxideNews.ron";
    fs::remove_file(file.as_str())
        .expect("Error unwrapping remove file");
}
