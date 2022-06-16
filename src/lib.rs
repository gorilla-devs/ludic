pub use faber::*;
pub use forgic::*;
pub use quantum::*;
pub use vanel::*;

pub mod tests {
    pub use std::{
        fs::{create_dir, File},
        io::Write,
    };

    pub use faber::*;
    pub use forgic::*;
    pub use futures::executor::block_on;
    pub use quantum::*;
    pub use vanel::*;

    pub fn test_faber() -> faber::MavenVersionResult {
        let run = get_fabric_versions();
        let out = block_on(run);
        println!("{:?}", out);
        return out;
    }

    pub fn test_forgic() -> forgic::MavenVersionResult {
        let run = get_forge_versions();
        let out = block_on(run);
        println!("{:?}", out);
        return out;
    }

    pub fn test_quantum() -> quantum::MavenVersionResult {
        let run = get_quilt_versions();
        let out = block_on(run);
        println!("{:?}", out);
        return out;
    }

    pub fn test_vanel() -> MinecraftVersionResult {
        let run = get_minecraft_versions();
        let out = block_on(run);
        println!("{:?}", out);
        return out;
    }

    #[test]
    pub fn test_all() {
        let faber_test = test_faber();
        let forgic_test = test_forgic();
        let quantum_test = test_quantum();
        let vanel_test = test_vanel();

        let faber_out = serde_json::to_string_pretty(&faber_test).unwrap();
        let forgic_out = serde_json::to_string_pretty(&forgic_test).unwrap();
        let quantum_out = serde_json::to_string_pretty(&quantum_test).unwrap();
        let vanel_out = serde_json::to_string_pretty(&vanel_test).unwrap();

        create_dir("tests").unwrap();

        let mut faber_file = File::create("tests/faber.json").unwrap();
        let mut forgic_file = File::create("tests/forgic.json").unwrap();
        let mut quantum_file = File::create("tests/quantum.json").unwrap();
        let mut vanel_file = File::create("tests/vanel.json").unwrap();

        faber_file.write_all(faber_out.as_bytes()).unwrap();
        forgic_file.write_all(forgic_out.as_bytes()).unwrap();
        quantum_file.write_all(quantum_out.as_bytes()).unwrap();
        vanel_file.write_all(vanel_out.as_bytes()).unwrap();
    }
}
