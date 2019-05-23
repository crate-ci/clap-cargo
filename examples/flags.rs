use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
struct Cli {
    #[structopt(flatten)]
    manifest: clap_cargo::Manifest,
    #[structopt(flatten)]
    workspace: clap_cargo::Workspace,
    #[structopt(flatten)]
    features: clap_cargo::Features,
}

fn main() {
    let args = Cli::from_args();
    println!("args = {:#?}", args);
}
