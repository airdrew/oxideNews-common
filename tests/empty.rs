extern crate oxide_news_common;

use oxide_news_common::Common;

#[test]
fn test_init()
{
    let common = Common::init("/tests");
    assert!(common.is_ok());
}

#[test]
fn test_add()
{
    let common = Common::init("/tests")
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
fn test_remove()
{
    let url = "https://latenightlinux.com/feed/mp3";
    let add_common = Common::init("/tests")
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

#[test]
fn test_feed()
{
    let url = "https://latenightlinux.com/feed/mp3";
    let folder_name = "podcasts";
    let add_common = Common::init("/tests")
        .unwrap()
        .add(url,
             folder_name,
             true);

    assert!(add_common.is_ok());
    let mut news = add_common.unwrap()
                             .news();
    let folders = news.folders();

    assert!(!folders.is_empty());

    let folder = folders.get_mut(folder_name)
                        .unwrap();

    let feeds = folder.feeds();
    let feed = feeds.get_mut(url)
                    .unwrap();

    assert!(feed.podcast());
    assert_eq!(feed.title(),
               "Late Night Linux (MP3)"
                   .to_owned());
    assert_eq!(feed.description(),
               "Linux after dark"
                   .to_owned());
    // assert_eq!(feed.image(),
    //           Some("https://latenightlinux.com/wp-content/uploads/2016/12/cropped-favicon-32x32.png"
    //                    .to_owned()));
    assert!(feed.categories()
                .is_empty());
    assert!(!feed.episodes()
                 .is_empty());
}
