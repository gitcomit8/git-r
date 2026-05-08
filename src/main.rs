use std::fs;
use std::path::Path;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let command = args.get(1).map(|s| s.as_str());

	match command {
		Some("init") => init_repository(),
		_ => println!("Usage: git-r <command>"),
	}
}

fn init_repository() {
	let dot_git = Path::new(".git");
	if dot_git.exists() {
		println!("Reinitialized existing Git repository in {} ",dot_git.display());
		return;
	}

	// Create dir hierarchy
	let dirs = [".git/objects", ".git/refs/heads"];
	for dir in dirs {
		fs::create_dir_all(dir).expect("Failed to create directory");
	}

	// Create HEAD symref
	fs::write(".git/HEAD", "ref: refs/heads/master\n").expect("Failed to write HEAD");

	println!("Initialized empty Git repository in {}",fs::canonicalize(dot_git).unwrap().display());
}