extern crate oxide_news_common;

use oxide_news_common::Common;

#[test]
fn initialize()
{
    assert!(Common::init("data")
                .is_ok());
}

#[test]
fn add()
{
    let common = Common::init("/home/phnxrbrn/.oxideNews")
        .unwrap()
        .add("https://latenightlinux.com/feed/mp3",
             "podcasts",
             true);

    assert!(common.is_ok());

    let mut news = common.unwrap()
                         .news();
    assert!(!news.folders()
                 .is_empty());
}

#[test]
fn remove()
{
    let url = "https://latenightlinux.com/feed/mp3";
    let add_common = Common::init("/home/phnxrbrn/.oxideNews")
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

    assert!(folders.is_empty());
}
