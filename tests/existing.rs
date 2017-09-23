extern crate oxide_news_common;

use oxide_news_common::Common;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_init()
{
    let path = PathBuf::from("./tests/data");
    let common = Common::init(fs::canonicalize(&path)
                                  .unwrap()
                                  .to_str()
                                  .unwrap());
    assert!(common.is_ok());
}

#[test]
fn test_add()
{
    let path = PathBuf::from("./tests/data/");
    let common = Common::init(fs::canonicalize(&path)
                                  .unwrap()
                                  .to_str()
                                  .unwrap())
                 .unwrap()
                 .add("http://feeds.feedburner.com/linuxunplugged",
                      "podcasts",
                      true);

    assert!(common.is_ok());

    let mut news = common.unwrap()
                         .news();
    assert!(!news.folders()
                 .is_empty());
}

#[test]
fn test_remove()
{
    let url = "http://feeds.feedburner.com/linuxunplugged";
    let path = PathBuf::from("./tests/data");
    let add_common = Common::init(fs::canonicalize(&path)
                                      .unwrap()
                                      .to_str()
                                      .unwrap())
                     .unwrap()
                     .add(url,
                          "podcasts",
                          true);

    assert!(add_common.is_ok());
    let add_cmn_uw = add_common.unwrap();

    assert!(!add_cmn_uw.clone()
                       .news()
                       .folders()
                       .is_empty());

    let rm_common = add_cmn_uw.clone()
                              .remove(url);

    let rm_cmn_uw = rm_common.unwrap();

    let mut rm_news = rm_cmn_uw.clone()
                               .news();

    let folders = rm_news.folders();

    assert!(!folders.is_empty());
}
