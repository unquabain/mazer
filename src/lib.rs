pub mod space;
pub mod space_square;
pub mod space_meta;
pub mod node;
pub mod edge;
pub mod error;
pub mod render;

use wasm_bindgen::prelude::*;

#[cfg(feature="wasm")]
struct WasmLogger;


#[cfg(feature="wasm")]
static WASM_LOGGER: WasmLogger = WasmLogger;

#[cfg(feature="wasm")]
impl log::Log for WasmLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let args = record.args();
            let args = format_args!("{}", args);
            let args = match record.target() {
                "" => args,
                _ => format_args!("{}: {}", record.target(), record.args()),
            };
            let file: String;
            let line: u32;
            let args = match record.file() {
                Some(f) => {
                    file = f.to_string();
                    match record.line() {
                        Some(l) => {
                            line = l;
                            format_args!("{}:{} {}", &file, line, args)
                        },
                        None => format_args!("{} {}", &file, args),
                    }
                },
                None => args,
            };
            match record.level() {
                log::Level::Error => {
                    gloo::console::error!(format!("{}", args));
                },
                log::Level::Warn => {
                    gloo::console::warn!(format!("{}", args));
                },
                log::Level::Info => {
                    gloo::console::info!(format!("{}", args));
                },
                log::Level::Debug => {
                    gloo::console::debug!(format!("{}", args));
                }
                log::Level::Trace => {
                    gloo::console::trace!(format!("{}", args));
                },
            }
        }
    }
    fn flush(&self) {}
}

#[cfg(feature="wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    log::set_logger(&WASM_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
}

mod internal {
    use crate::{
        space::*,
        render::*,
        space_square::*,
        space_meta::*,
    };
    pub fn maze_square(seed: u64, width: usize, height: usize, scale: f32) -> (String, usize, usize) {
        use rand::SeedableRng;
        use rand_chacha::ChaCha12Rng;
        let mut rng = ChaCha12Rng::seed_from_u64(seed);
        let space = SpaceSquare::new(height, width);
        let (nodes, mut edges) = space.layout(6, &mut rng).unwrap();
        let (start, end) = space.get_endpoints(&mut rng);
        let end_zone = nodes[end].group.unwrap();
        let mut meta = SpaceMeta::new(&space, &edges, &nodes);
        let (_meta_nodes, mut meta_edges) = meta.layout(1, &mut rng).unwrap();
        meta.open_gateways(&meta_edges, &mut edges, &mut rng);

        let mut solution_zones = 1;
        let solution_length = {
            let mut solution_length = 0;
            let mut start = start;
            let mut start_zone = nodes[start].group.unwrap();
            let last_end = end;
            let meta_start = meta.zone_index(start_zone).unwrap();
            let meta_end = meta.zone_index(end_zone).unwrap();
            for zone in meta.solve(&mut meta_edges, meta_start, meta_end) {
                let end_edge = meta.gateway(zone).unwrap();
                edges[end_edge].solution = true;
                let (end, new_start): (Vec<_>, Vec<_>) = space.edge_nodes(end_edge)
                                       .map(|nid| (nid, nodes[nid].group.unwrap()))
                                       .partition(|(_nid, gid)| *gid == start_zone);
                let end = end.into_iter().next().unwrap();
                let end = end.0;
                let new_start = new_start.into_iter().next().unwrap();
                solution_length += space.solve(&mut edges, start, end).count();
                start = new_start.0;
                start_zone = new_start.1;
                solution_zones += 1;
            }
            solution_length + space.solve(&mut edges, start, last_end).count()
        };
        let svg = render_svg_2d(&space, &edges, &nodes, width, height, scale, start, end);
        (format!("{}", svg), solution_zones, solution_length)
    }
}


#[cfg(feature="wasm")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MazeSquareResult {
    pub svg: String,
    pub solution_zones: usize,
    pub solution_length: usize,
}

#[cfg(feature="wasm")]
#[wasm_bindgen]
pub fn maze_square(seed: u64, width: usize, height: usize, scale: f32) -> JsValue {
    let (svg, solution_zones, solution_length) = internal::maze_square(seed, width, height, scale);
    serde_wasm_bindgen::to_value(&MazeSquareResult {
        svg,
        solution_zones,
        solution_length,
    }).unwrap()
}

#[cfg(not(feature="wasm"))]
pub fn maze_square(seed: u64, width: usize, height: usize, scale: f32) -> (String, usize, usize) {
    internal::maze_square(seed, width, height, scale)
}
