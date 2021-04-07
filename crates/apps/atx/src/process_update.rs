use clap::ArgMatches;
use futures::executor::block_on;
use futures::prelude::*;
use linq::error::*;
use linq::io::Io;
use std::{io, io::prelude::*};

fn print_bar(bar: &Vec<char>) {
    for c in bar {
        print!("{}", c);
    }
}

fn print_status(count: usize, total: usize) {
    let weight: f32 = 50 as f32 / total as f32;
    let distance = weight * count as f32;
    let distance = distance as usize;
    let start = vec!['['; 1];
    let progress = vec!['#'; distance];
    let remaining = vec![' '; 50 - distance];
    let end = vec![']'; 1];
    print_bar(&start);
    print_bar(&progress);
    print_bar(&remaining);
    print_bar(&end);
    print!("{esc}[0G", esc = 27 as char);
    print!("{:>3.3}:{:<3.3}  ", count, total);
    io::stdout().flush().ok().expect("cloud not flush stdout");
}

async fn process(linq: &mut Io, f: &str, image: u8) -> Result<String> {
    let metadata = linq.scan().await?;
    if metadata.len() > 0 {
        linq.update_file_path(&metadata[0].serial, f, image)?
            .try_for_each(|x| {
                let (count, total) = (x.0, x.1);
                print_status(count, total);
                future::ready(Ok(()))
            })
            .await?;
    }
    Ok("Complete!".to_owned())
}
pub fn process_update(cli: &ArgMatches) -> Result<String> {
    let p = cli.value_of("file").unwrap();

    // Currently only USB supported at the moment
    let proto = cli.value_of("protocol").unwrap();
    if !(proto == "usb") {
        panic!("protocol {} not supported", proto);
    };

    let image = if cli.value_of("image").unwrap() == "firmware" {
        0
    } else {
        1
    };

    let mut linq = Io::new();
    let result = block_on(process(&mut linq, p, image));
    linq.close().unwrap();
    result
}
