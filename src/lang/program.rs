use {
    super::*,
    crate::{
        dom::{self, Append, Attrs},
    },
    std::str::FromStr,
    web_sys::*,
};

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl FromStr for Program {
    type Err = SyntaxError;
    fn from_str(code: &str) -> Result<Self, SyntaxError> {
        let mut statements = Vec::new();
        for line in code.split('\n') {
            match line.parse() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(SyntaxError::Empty) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(Self { statements })
    }
}

impl Program {
    pub fn compile(&self) -> SvgElement {
        let svg = dom::svg();
        svg.set_attr("viewBox", "0 0 100 100");
        let mut pos = Point::new(50.0, 50.0);
        let mut color = Color::black;
        let mut points: Vec<Point> = Vec::new();
        macro_rules! end {
            () => {
                match points.len() {
                    0 => {}
                    1 => {
                        // dessiner un point
                    }
                    2 => {
                        let line = dom::line();
                        line.set_attr("x1", points[0].x.to_string());
                        line.set_attr("y1", points[0].y.to_string());
                        line.set_attr("x2", points[1].x.to_string());
                        line.set_attr("y2", points[1].y.to_string());
                        line.set_attr("style", format!("stroke:{}", color.name()));
                        line.set_attr("stroke-linecap", "round");
                        svg.append(line);
                    }
                    _ => {
                        let poly = dom::polyline();
                        let spoints: Vec<String> = points.iter()
                            .map(|p| format!("{},{}", p.x, p.y))
                            .collect();
                        poly.set_attr("points", spoints.join(" "));
                        poly.set_attr("style", format!("stroke:{}", color.name()));
                        poly.set_attr("fill", "none");
                        poly.set_attr("stroke-linejoin", "round");
                        poly.set_attr("stroke-linecap", "round");
                        svg.append(poly);
                    }
                }
                points.clear();
            }
        }
        for statement in &self.statements {
            match statement {
                Statement::Color(c) => {
                    end!();
                    color = *c;
                }
                Statement::MoveTo(point) => {
                    end!();
                    points.push(*point);
                    pos = *point;
                }
                Statement::LineTo(point) => {
                    if points.is_empty() {
                        points.push(pos);
                    }
                    points.push(*point);
                    pos = *point;
                }
            }
        }
        end!();
        svg
    }
}
