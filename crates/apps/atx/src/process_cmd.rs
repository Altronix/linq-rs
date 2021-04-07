use clap::ArgMatches;
use futures::executor::block_on;
use futures::prelude::*;
use linq::error::*;
use linq::io::Io;
use linq::Request;

async fn process(request: Request) -> Result<String> {
    let mut linq = Io::new();
    let metadata = linq.scan().await?;
    let result = linq.request(&metadata[0].serial, request).await;
    linq.close().unwrap();
    result.map_err(|x| LinqError::from(x))
    // result.into()
}

pub fn process_cmd(cli: &ArgMatches) -> Result<String> {
    let path = cli.value_of("path").unwrap();
    let method = cli.value_of("method").unwrap();
    let protocol = cli.value_of("protocol").unwrap();
    // let data = m
    // .value_of("file")
    // .map_or(cli.value_of("data"), |f| f.unwrap());

    let request = match method {
        // "POST" => Request::Post(path.into(), data.into()),
        "DELETE" => linq::Request::Delete(path.into()),
        _ => linq::Request::Get(path.into()),
    };
    let proto = cli.value_of("protocol").unwrap();
    if !(proto == "usb") {
        panic!("protocol {} not supported", proto);
    };

    block_on(process(request))
}
