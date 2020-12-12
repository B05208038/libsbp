use structopt::StructOpt;
use tokio::runtime;

#[cfg(all(not(windows), not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Convert binary SBP data to JSON.
///
/// Typical usage:
///
///     cat sbp.dat | sbp2json
///
/// Or combined with socat:
///
///     socat tcp:192.168.1.222:55555 - | sbp2json
#[derive(Debug, StructOpt)]
#[structopt(name = "sbp2json", verbatim_doc_comment)]
pub struct Options {
    /// Try to be compatible with the float formatting of the Haskell version of sbp2json
    #[structopt(long = "float-compat")]
    float_compat: bool,

    /// Print debugging messages to standard error
    #[structopt(short = "d", long)]
    debug: bool,
}

fn main() {
    let options = Options::from_args();

    if options.debug {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let rt = runtime::Builder::new_multi_thread().build().unwrap();

    rt.block_on(async {
        let stdin = sbp2json::stdin();
        let stdout = sbp2json::stdout();

        if options.float_compat {
            sbp::codec::sbp2json(stdin, stdout, sbp::codec::HaskellishFloatFormatter {}).await
        } else {
            sbp::codec::sbp2json(stdin, stdout, sbp::codec::CompactFormatter {}).await
        }
    })
    .unwrap()
}
