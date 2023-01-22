

pub struct AncestryNode {
  pub id: String,
  pub mother: Option<String>,
  pub father: Option<String>,
  pub depth: i32,
}