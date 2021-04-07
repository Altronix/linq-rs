use clap::ArgMatches;
use futures::executor::block_on;
use linq;
use linq::io::Io;
use linq::error::*;
use linq::Request;

async fn process(linq: &mut Io, request: Vec<Request>) -> Result<String> {
    let metadata = linq.scan().await?;
    if metadata.len() > 0 {
        for r in request.into_iter() {
            let _response = linq.request(&metadata[0].serial, r).await?;
        }
    }
    Ok("Complete!".to_owned())
}

pub fn process_ipconfig(cli: &ArgMatches) -> Result<String> {
    let mut requests: Vec<Request> = vec![];
    if let Some(arg) = cli.value_of("ip") {
        let data = format!("{{\"ip\":\"{}\"}}", arg);
        requests.push(Request::post_raw("/ATX/network/ipConfig/ip", data))
    }
    if let Some(arg) = cli.value_of("sn") {
        let data = format!("{{\"sn\":\"{}\"}}", arg);
        requests.push(Request::post_raw("/ATX/network/ipConfig/sn", data))
    }
    if let Some(arg) = cli.value_of("gw") {
        let data = format!("{{\"gw\":\"{}\"}}", arg);
        requests.push(Request::post_raw("/ATX/network/ipConfig/gw", data))
    }
    requests.push(Request::post_raw("/ATX/exe/save", "{\"save\":1}"));
    if cli.is_present("reboot") {
        requests.push(Request::post_raw("/ATX/exe/reboot", "{\"reboot\":1}"));
    }

    // Currently only USB supported at the moment
    let proto = cli.value_of("protocol").unwrap();
    if !(proto == "usb") {
        panic!("protocol {} not supported", proto);
    };

    let mut linq = Io::new();
    let result = block_on(process(&mut linq, requests));
    linq.close().unwrap();
    result
}
