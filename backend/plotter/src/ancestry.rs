
use plotters::style::ShapeStyle;
use plotters::{backend::BitMapBackend, prelude::IntoDrawingArea};
use plotters::prelude::{RGBColor, EmptyElement, Circle, BLACK, Text};
use anyhow::Result;
use plotters::prelude::*;


pub async fn get_ancestry(id: &str) -> Result<()> {
  let path = format!("public/ancestry/{id}.png");
  let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();

  root.fill(&RGBColor(240, 200, 200))?;

  let dot_and_label = |x: i32, y: i32, label| {
      return EmptyElement::at((x, y))
          + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
          + Text::new(
              label,
              (0, 5),
              ("sans-serif", 15.0).into_font(),
          );
  };

  root.draw(&dot_and_label(200, 200, "some text"))?;

  root.present()?;

  Ok(())
}