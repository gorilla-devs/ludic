pub use faber::*;
pub use forgic::*;
pub use quantum::*;
pub use vanel::*;

pub mod tests {
    pub use faber::*;
    pub use forgic::*;
    pub use futures::executor::block_on;
    pub use quantum::*;
    pub use vanel::*;

    #[test]
    pub fn test_faber() {
        let run = get_fabric_versions();
        let out = block_on(run);
        println!("{:?}", out);
    }

    #[test]
    pub fn test_forgic() {
        let run = get_forge_versions();
        let out = block_on(run);
        println!("{:?}", out);
    }

    #[test]
    pub fn test_quantum() {
        let run = get_quilt_versions();
        let out = block_on(run);
        println!("{:?}", out);
    }

    #[test]
    pub fn test_vanel() {
        let run = get_minecraft_versions();
        let out = block_on(run);
        println!("{:?}", out);
    }
}
