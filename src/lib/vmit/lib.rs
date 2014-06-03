#![crate_type = "lib"]
#![crate_id = "vmit#0.1"]

use std::os::getcwd;
use std::owned::Box;

pub struct Workspace {
    dir: Box<Path>
}

impl Workspace {

    pub fn new(dir: &Path) -> Workspace {
        Workspace { dir: box dir.clone()}
    }

    pub fn from_pwd() -> Workspace {
        Workspace::new(&std::os::getcwd())
    }
}

impl std::fmt::Show for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hi: {}", self.dir.display())
    }
}

#[cfg(test)]
mod test {

    use std::io::TempDir;
    use super::Workspace;

    #[test]
    fn test_workspace() {
        match TempDir::new("vmit") {
            Some(tmpdir) => {
                println!("{}", tmpdir.path().display());
                let ws = Workspace::new(tmpdir.path());


            }
            None => {
              println!("ups!");
            }
        }
        assert_eq!(1, 3);
    }
}

