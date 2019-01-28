use gltf::Gltf;
use gltf::Error;

fn main() -> Result<(), Error> {
  let gltf = Gltf::open("assets/mesh/tank.gltf")?;
  for scene in gltf.scenes() {
      println!("** SCENE **");
      for node in scene.nodes() {
          println!(
              "Node #{} has {} children",
              node.index(),
              node.children().count(),
          );
      }
  }
  Ok(())
}
