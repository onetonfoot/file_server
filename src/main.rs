#![deny(warnings)]
use std::path::PathBuf;
use std::env::current_dir;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "7777")]
    port: u16,

    #[structopt(parse(from_os_str))]
    folder: Option<PathBuf>,
}


#[tokio::main]
async fn main() {

    let opt = Opt::from_args();

    let folder = match opt.folder {
        Some(folder) => folder,
        _ => current_dir().unwrap()
    };


    let dir = warp::fs::dir(folder);
    println!("Staring server on port 3030");
    let server = warp::serve(dir).run(([127, 0, 0, 1], opt.port));
    server.await;
}