use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "subquery", about = "Subquery CLI")]
pub enum Opt {
  /// Login subquery
  Login {
    /// Connect sid
    #[structopt(long)]
    sid: String,
  },
}
