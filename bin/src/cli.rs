use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub year: Option<usize>,
    #[clap(short, long)]
    pub day: Option<usize>,
}
