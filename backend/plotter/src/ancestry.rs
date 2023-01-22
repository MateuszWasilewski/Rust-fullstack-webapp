use std::collections::HashMap;
use std::rc::Rc;

use plotters::coord::Shift;
use plotters::style::ShapeStyle;
use plotters::{backend::BitMapBackend, prelude::IntoDrawingArea};
use plotters::prelude::{RGBColor, EmptyElement, Circle, BLACK, Text};
use anyhow::{Result, bail};
use plotters::prelude::*;

use types::AncestryNode;
use db::ConnectionDB;
use common::Photo;

#[derive(Default)]
struct Node {
  id: String,
  mother: Option<Rc<Node>>,
  father: Option<Rc<Node>>
}

fn get_graph(max_depth: i32, ancestry: Vec<AncestryNode>) -> Result<Rc<Node>> {
  let mut node_id = "";
  ancestry.iter().for_each(|animal| {
    if animal.depth == 0 {
      node_id = &animal.id;
    }
  });
  let mut nodes: HashMap<String, Rc<Node>> = HashMap::new();
  for depth in (0..max_depth).rev() {
    ancestry.iter().filter(|animal| animal.depth == depth)
    .for_each(|animal| {
      let mut node = Node::default();
      node.id = animal.id.clone();
      if let Some(father) = &animal.father {
        let father = nodes.get(father);
        if let Some(father) = father {
          node.father = Some(father.clone());
        }
      }
      if let Some(mother) = &animal.mother {
        let mother = nodes.get(mother);
        if let Some(mother) = mother {
          node.mother = Some(mother.clone());
        }
      }
      nodes.insert(animal.id.clone(), Rc::new(node));
    });
  }

  match nodes.get(node_id) {
    Some(node) => Ok(node.clone()),
    None => bail!("Node should be present")
  }
}

fn recursive_draw(root: &DrawingArea<BitMapBackend, Shift>, node: &Rc<Node>, y_draw: (i32, i32), x_now: i32, x_step: i32) -> Result<()>{
  let dot_and_label = |x: i32, y: i32, label: &String| {
    return EmptyElement::at((x, y))
        + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
        + Text::new(
            label.clone(),
            (-20, 5),
            ("sans-serif", 15.0).into_font(),
        );
  };
  let line = |p1: (i32, i32), p2: (i32, i32)| {
      Polygon::new(vec![p1, p2], ShapeStyle::from(&BLACK).filled())
  };
  let y_mid = (y_draw.0 + y_draw.1) / 2;
  root.draw(&dot_and_label(x_now, y_mid, &node.id))?;

  if node.father.is_some() || node.mother.is_some() {
    let vert_x = x_now + 50;
    let father_y = (y_draw.1 - y_draw.0) / 4 + y_draw.0;
    let mother_y = (y_draw.1 - y_draw.0) * 3/ 4 + y_draw.0;

    root.draw(&line((x_now, y_mid), (vert_x, y_mid)))?;
    root.draw(&line((vert_x, father_y), (vert_x, mother_y)))?;

    root.draw(&line((vert_x, father_y), (x_now + x_step, father_y)))?;
    root.draw(&line((vert_x, mother_y), (x_now + x_step, mother_y)))?;
  }
  
  if let Some(father) = &node.father {
    recursive_draw(root, father, (y_draw.0, y_mid), x_now + x_step, x_step)?;
  }
  if let Some(mother) = &node.mother {
    recursive_draw(root, mother, (y_mid, y_draw.1), x_now + x_step, x_step)?;
  }
  Ok(())
}

pub async fn generate_ancestry(id: &str, connection: &ConnectionDB) -> Result<Photo> {
  const MAX_DEPTH: i32 = 3;
  let ancestry = db::select::ancestry(id, MAX_DEPTH, connection).await?;
  let node = get_graph(MAX_DEPTH, ancestry)?;

  let path = format!("public/ancestry/{}.png", node.id);
  let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();

  root.fill(&RGBColor(240, 200, 200))?;

  recursive_draw(&root, &node, (0, 480), 150, 150)?;

  root.present()?;

  Ok(Photo {
    path: path.clone(),
    author: None
  })
}