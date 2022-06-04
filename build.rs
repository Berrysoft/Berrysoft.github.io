use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local, TimeZone};
use pulldown_cmark::{Event, Parser};
use std::fs::{read_dir, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

fn find_first_commit<'a>(p: &Path) -> Result<DateTime<Local>> {
    let history = std::process::Command::new("git")
        .args(["log", "--format=%at", "--follow", &p.to_string_lossy()])
        .output()?;
    let history = unsafe { String::from_utf8_unchecked(history.stdout) };
    let last_line = history
        .split('\n')
        .filter(|s| !s.is_empty())
        .last()
        .unwrap();
    Ok(Local.timestamp(last_line.parse()?, 0))
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blogdata/*");

    let mut ch = {
        let rss_file = File::open("blogdata/feed.xml")?;
        let rss_file = BufReader::new(rss_file);
        rss::Channel::read_from(rss_file)?
    };
    if let Ok(rss_file) = File::open("dist/blogdata/feed.xml") {
        let rss_file = BufReader::new(rss_file);
        if let Ok(output_ch) = rss::Channel::read_from(rss_file) {
            if ch.items.len() == output_ch.items.len() {
                return Ok(());
            }
        }
    }
    let now = Local::now();
    ch.last_build_date = Some(now.with_timezone(&FixedOffset::east(8 * 3600)).to_rfc2822());
    ch.items.clear();

    let mut files = vec![];
    for f in read_dir("blogdata")? {
        let f = f?;
        if f.file_type()?.is_file() {
            let p = f.path();
            if p.extension().and_then(|s| s.to_str()) == Some("md") {
                let pub_time = find_first_commit(&p)?;
                files.push((p, pub_time));
            }
        }
    }
    files.sort_by_key(|(_, t)| *t);
    for (p, pub_date) in files {
        let description = {
            let mut blog_file = File::open(&p)?;
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
        let filename = p
            .with_extension("")
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default()
            .into_owned();
        ch.items.push(
            rss::ItemBuilder::default()
                .title(filename.clone())
                .link(format!("{}{}", ch.link, filename))
                .description(description)
                .guid(
                    rss::GuidBuilder::default()
                        .permalink(false)
                        .value(filename)
                        .build(),
                )
                .pub_date(pub_date.to_rfc2822())
                .build(),
        );
    }
    {
        let rss_file = File::options()
            .write(true)
            .create(true)
            .open("blogdata/feed.xml")?;
        let rss_file = BufWriter::new(rss_file);
        ch.pretty_write_to(rss_file, b' ', 2)?;
    }
    Ok(())
}
