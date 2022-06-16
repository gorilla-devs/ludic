pub use faber::*;
pub use forgic::*;
pub use quantum_mc::*;
pub use vanel::*;

pub mod converters;
pub mod util;

use neon::prelude::{ModuleContext, NeonResult};

#[neon::main]
pub fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    ctx.export_function("getFabricVersions", converters::faber::get_fabric_versions)?;
    return Ok(());
}

pub mod tests {
    #[test]
    fn test_faber() {
        let run = faber::get_fabric_versions();
        let out = futures::executor::block_on(run);
        println!("{:?}", out);
    }

    #[test]
    fn test_forgic() {
        let run = forgic::get_forge_versions();
        let out = futures::executor::block_on(run);
        println!("{:?}", out);
    }

    #[test]
    fn test_quantum() {
        let run = quantum_mc::get_quilt_versions();
        let out = futures::executor::block_on(run);
        println!("{:?}", out);
    }

    #[test]
    fn test_vanel() {
        let run = vanel::get_minecraft_versions();
        let out = futures::executor::block_on(run);
        println!("{:?}", out);
    }
}
