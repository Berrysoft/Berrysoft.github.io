use chrono::{Datelike, FixedOffset, Local};
use clap::Parser as ClapParser;
use pulldown_cmark::{Event, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::path::PathBuf;

#[derive(Debug, ClapParser)]
#[clap(about, version, author)]
struct Opt {
    #[clap()]
    input: String,
    #[clap(long)]
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();
    let mut input = PathBuf::from("blogdata");
    input.push(opt.input);
    let mut ch = {
        let rss_file = File::open("blogdata/feed.xml")?;
        let rss_file = BufReader::new(rss_file);
        rss::Channel::read_from(rss_file)?
    };
    let now = Local::now();
    ch.last_build_date = Some(now.with_timezone(&FixedOffset::east(8 * 3600)).to_rfc2822());
    ch.generator = Some("pages::blog".to_string());
    let description = {
        let mut blog_file = File::open(&input)?;
        let mut text = String::new();
        blog_file.read_to_string(&mut text)?;
        let parser = Parser::new(&text);
        parser
            .filter_map(|e| {
                if let Event::Text(text) = e {
                    Some(text.to_string())
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_default()
    };
    let new_filename = format!(
        "{}_{}_{}_{}",
        input
            .file_stem()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default(),
        now.year(),
        now.month(),
        now.day()
    );
    let mut new_input = input.clone();
    new_input.set_file_name(format!(
        "{}.{}",
        new_filename,
        input
            .extension()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default()
    ));
    std::fs::rename(input, new_input)?;
    ch.items.push({
        let mut item = rss::Item::default();
        item.set_title(opt.title);
        item.set_link(format!("{}{}", ch.link, new_filename));
        item.set_description(description);
        item.set_guid(
            rss::GuidBuilder::default()
                .permalink(false)
                .value(new_filename)
                .build(),
        );
        item.set_pub_date(ch.last_build_date.clone());
        item
    });
    {
        let rss_file = File::options().write(true).open("blogdata/feed.xml")?;
        let rss_file = BufWriter::new(rss_file);
        ch.pretty_write_to(rss_file, b' ', 2)?;
    }
    Ok(())
}
