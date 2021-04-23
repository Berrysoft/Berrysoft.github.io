#![feature(with_options)]

use chrono::{FixedOffset, Local};
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    let mut ch = {
        let rss_file = File::open("static/blogdata/feed.xml").unwrap();
        let rss_file = BufReader::new(rss_file);
        rss::Channel::read_from(rss_file).unwrap()
    };
    ch.last_build_date = Some(
        Local::now()
            .with_timezone(&FixedOffset::east(8 * 3600))
            .to_rfc2822(),
    );
    ch.generator = Some("pages::blog".to_string());
    {
        let rss_file = File::with_options()
            .write(true)
            .open("static/blogdata/feed.xml")
            .unwrap();
        let rss_file = BufWriter::new(rss_file);
        ch.pretty_write_to(rss_file, b' ', 2).unwrap();
    }
}
