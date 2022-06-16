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
        block_on(run);
    }

    #[test]
    pub fn test_forgic() {
        let run = get_forge_versions();
        block_on(run);
    }

    #[test]
    pub fn test_quantum() {
        let run = get_quilt_versions();
        block_on(run);
    }
}
