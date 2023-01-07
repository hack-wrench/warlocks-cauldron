use super::dependencies::*;


/// Enum for init path-gen struct
pub enum PlatformType { Win, Linux }

impl PlatformType {
    /// Detect current system platform
    pub fn detect() -> Self {
        match cfg!(windows) {
            true => Self::Win,
            false => Self::Linux,
        }
    }
}

/// Struct that provides methods and property for generate paths.
pub struct Path {
    is_win: bool
}

impl Path {
    /// Initialize path-gen with platfrom specify
    pub fn new(platform: PlatformType) -> Self {
        Self { 
            is_win: match platform {
                PlatformType::Win => true,
                _ => false,
            }
        }
    }

    /// Generate path from paths
    fn path_gen<const N: usize>(&self, paths: [&str; N]) -> String {
        paths.iter().join(&self.delimetr())
    }

    /// Get path delimetr
    pub fn delimetr(&self) -> String {
        match self.is_win {
            true => "\\",
            false => "/",
        }.to_string()
    }

    /// Generate a root dir path
    pub fn root(&self) -> String {
        match self.is_win {
            true => "C:\\Windows",
            false => "/",
        }.to_string()
    }

    /// Generate a home path
    pub fn home(&self) -> String {
        match self.is_win {
            true => "C:\\Users",
            false => "/home",
        }.to_string()
    }

    /// Generate a random user
    pub fn user(&self) -> String {
        let user = get_random_element(USERNAMES.iter());
        match self.is_win {
            true => user[0..1].to_uppercase() + &user[1..],
            false => user.to_string(),
        }
    }

    /// Generate a random path to user's folders
    pub fn users_folder(&self) -> String {
        let folder = get_random_element(FOLDERS.iter());
        self.path_gen([&self.home(), &self.user(), &folder])
    }

    /// Generate a random path to development directory
    pub fn dev_dir(&self) -> String {
        let folder = get_random_element(vec!["Development", "Dev"].into_iter());
        let stack = get_random_element(PROGRAMMING_LANGS.iter());
        self.path_gen([&self.home(),  &self.user(), folder, stack])
    }

    /// Generate a random path to project directory
    pub fn project_dir(&self) -> String {
        let project = get_random_element(PROJECT_NAMES.iter());
        self.path_gen([&self.dev_dir(), project])
    }
}
