pub mod aws;
pub mod battery;
pub mod character;
pub mod cmd_duration;
pub mod conda;
pub mod directory;
pub mod dotnet;
pub mod env_var;
pub mod git_branch;
pub mod git_commit;
pub mod git_state;
pub mod git_status;
pub mod go;
pub mod hg_branch;
pub mod hostname;
pub mod java;
pub mod jobs;
pub mod kubernetes;
pub mod memory_usage;
pub mod nix_shell;
pub mod nodejs;
pub mod package;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod status;
mod starship_root;
pub mod terraform;
pub mod time;
pub mod username;

pub use starship_root::*;
