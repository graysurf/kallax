use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug)]
pub struct Config {
    #[clap(long = "node-key-file-path", help = "Node key file path")]
    pub node_key_file_path: PathBuf,

    #[clap(long = "tracker-grpc-endpoint", help = "Tracker gRPC endpoint")]
    pub tracker_grpc_endpoint: http::Uri,

    #[clap(long = "rootchain-name", help = "Rootchain name")]
    pub rootchain_name: String,

    #[clap(long = "rootchain-spec-file-path", help = "Rootchain spec file path")]
    pub rootchain_spec_file_path: PathBuf,

    #[clap(long = "leafchain-name", help = "Leafchain name")]
    pub leafchain_name: Option<String>,

    #[clap(long = "leafchain-spec-file-path", help = "Leafchain spec file path")]
    pub leafchain_spec_file_path: Option<PathBuf>,

    #[clap(long = "keystore-directory-path", help = "Keystore directory path")]
    pub keystore_directory_path: PathBuf,

    #[clap(long = "session-key-mnemonic-phrase", help = "Session key mnemonic phrase")]
    pub session_key_mnemonic_phrase: String,

    #[clap(long = "node-name", help = "Node name")]
    pub node_name: String,
}
