use crate::{line2d::Line2D, point2d::Point2D};
use web_sys::CanvasRenderingContext2d;

pub struct Grid2D<Cell> {
    width: usize,
    height: usize,
    rows: usize,
    columns: usize,
    cells: Vec<Cell>,
    lines: Vec<Line2D>,
}

impl<Cell> Grid2D<Cell> {
    pub fn new(width: usize, height: usize, rows: usize, columns: usize) -> Self {
        let cells: Vec<Cell> = Vec::with_capacity(rows * columns);
        let mut lines = Vec::with_capacity(rows * columns);

        let cell_width = width / columns;
        let cell_height = height / rows;

        for idx in 1..rows {
            let x_from = 0.0;
            let x_to = width as f64;

            let y_from = (idx * cell_height) as f64;
            let y_to = y_from;

            lines.push(Line2D::from([Point2D::from([x_from, y_from]), Point2D::from([x_to, y_to])]));
        }

        for idx in 1..columns {
            let x_from = (idx * cell_width) as f64;
            let x_to = x_from;

            let y_from = 0.0;
            let y_to = height as f64;

            lines.push(Line2D::from([Point2D::from([x_from, y_from]), Point2D::from([x_to, y_to])]));
        }

        return Grid2D {
            width,
            height,
            rows,
            columns,
            cells,
            lines,
        };
    }

    pub fn get(&self, target: usize) -> Option<&Cell> {
        return self.cells.get(target);
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        for line in self.lines.as_slice() {
            ctx.begin_path();
            ctx.move_to(line.a.x, line.a.y);
            ctx.line_to(line.b.x, line.b.y);
            ctx.close_path();
            ctx.stroke();
        }
    }
}
