use flow::User;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base_dir = dirs::home_dir().unwrap().join("Flow");
    let user = User::new("alice", &base_dir).expect("User not created, error encountered.");

    println!("User created: {:?}", user);
    println!("Workspace path: {}", user.workspace_path.display());

    Ok(())
}
