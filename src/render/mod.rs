use crate::graph;
use std::fmt;

pub struct SVG<'a> {
    graph: &'a graph::Graph,
    start: (usize, usize),
    end: (usize, usize),
    solution: bool,
    url: &'a str,
}

impl<'a> SVG<'a> {
    pub fn new(graph: &'a graph::Graph, start: (usize, usize), end: (usize, usize), solution: bool, url: &'a str) -> SVG<'a> {
        Self{
            graph,
            start,
            end,
            solution,
            url,
        }
    }
}

impl fmt::Display for SVG<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let g = &self.graph;
        let s = &g.space;


        write!(
            f,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="100%" viewBox="-1 -1 {width} {height}">
"##,
            width=s.width+2,
            height=s.height+2,
        )?;

        write!(f, "{}", r##"  <style>.edge {
    stroke: black;
    stroke-width: 0.2;
  }
  .label {
    font-size: 0.6pt;
    font-family: sans-serif;
    font-weight: bold;
    text-align: center;
    vertical-align: middle;
    text-anchor: middle;
    fill: red;
  }
  .solution {
    font-size: 0.6pt;
    font-family: sans-serif;
    font-weight: bold;
    text-align: center;
    text-decoration: underline;
    vertical-align: middle;
    text-anchor: middle;
    fill: blue;
  }
  .marker {
    fill: blue;
  }
  .endpoint {
    stroke: red;
    stroke-width: 0.2;
    fill: none;
  }
</style>
"##)?;

        for x in 0..s.width {
            let [n, ..] = s.edges((x, 0));
            let n = g.edges[n];

            if let graph::Direction::Closed = n.direction {
                writeln!(f, r##"  <path d="m{} 0 l1 0" class="edge"/>"##, x)?;
            }
        }
        for y in 0..s.height {
            let [_, w, ..] = s.edges((0, y));
            let w = g.edges[w];

            if let graph::Direction::Closed = w.direction {
                writeln!(f, r##"  <path d="m0 {} l0 1" class="edge"/>"##, y)?;
            }
        }

        for c in 0..s.num_cells() {
            let coords = s.coords(c);
            let (x, y) = coords;
            let [.., e, s] = s.edges(coords);
            let e = g.edges[e];
            let s = g.edges[s];
            if let graph::Direction::Closed = s.direction {
                writeln!(f, r##"  <path d="m{} {} l1 0" class="edge"/>"##, x, y+1)?;
            } else if self.solution && s.solution {
                writeln!(f, r##"  <circle class="marker" cx="{}.5" cy="{}" r="0.2"/>"##, x, y+1)?;
            }
            if let graph::Direction::Closed = e.direction {
                writeln!(f, r##"  <path d="m{} {} l0 1" class="edge"/>"##, x+1, y)?;
            } else if self.solution & e.solution {
                writeln!(f, r##"  <circle class="marker" cx="{}" cy="{}.5" r="0.2"/>"##, x+1, y)?;
            }
        }

        writeln!(f, r##"  <text class="label" x="{}.5" y="{}.8">S</text>"##, self.start.0, self.start.1)?;
        writeln!(f, r##"  <circle class="endpoint" cx="{}.5" cy="{}.5" r="0.65"/>"##, self.start.0, self.start.1)?;
        writeln!(f, r##"  <text class="label" x="{}.5" y="{}.8">E</text>"##, self.end.0, self.end.1)?;
        writeln!(f, r##"  <circle class="endpoint" cx="{}.5" cy="{}.5" r="0.65"/>"##, self.end.0, self.end.1)?;
        writeln!(
            f,
            r##"<a href="{}"><text class="solution" x="{}" y="-0.2">link</text></a>"##,
            self.url.replace("&", "&amp;"),
            s.width/2,
        )?;
        if !self.solution {
            writeln!(
                f,
                r##"<a href="{}&amp;solution=true"><text class="solution" x="{}" y="{}.8">solution</text></a>"##,
                self.url.replace("&", "&amp;"),
                s.width/2,
                s.height,
            )?;
        }

        write!(f, "</svg>")
    }
}
