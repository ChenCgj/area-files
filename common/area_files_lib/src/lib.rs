extern crate core;

mod server;
pub mod user;
pub mod file_mgr;
mod token_mgr;
pub mod protocol;

pub use server::Server;
pub use user::User;

pub mod test_mod {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    }
}
