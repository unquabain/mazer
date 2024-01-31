mod mazeparams;

use mazeparams::MazeParams;
use std::thread;
use super::graph::Graph;
use super::graph::Automaton;
use super::render::SVG;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use querystring;
use rand_chacha::ChaCha8Rng;
use tracing::{event, Level, instrument};

#[derive(Debug)]
pub struct Server {
    pub addr: String,
}

impl Server {

    #[instrument]
    fn handle_read(mut stream: &TcpStream) -> Result<MazeParams, String> {
        let mut maze: MazeParams = Default::default();
        let mut buf = [0u8 ;4096];
        if let Err(_) = stream.read(&mut buf) {
            return Err("could not read stream".to_string());
        }
        let mut headers = [httparse::EMPTY_HEADER; 32];
        let mut req = httparse::Request::new(&mut headers);
        if let Err(err) = req.parse(&buf) {
            event!(Level::ERROR, "could not parse request: {:?}", String::from_utf8_lossy(&buf));
            return Err(format!("could not parse request: {}", err));
        }
        let (_path, query) = match req.path.unwrap_or_default().split_once('?') {
            Some((path, query)) => (path, query),
            None => ("", ""),
        };

        for param in querystring::querify(query) {
            match param.0 {
                "width" => {
                    match param.1.parse::<usize>() {
                        Ok(w) => maze.width = w,
                        Err(_) => return Err("could not parse width".to_string()),
                    }
                },
                "height" => {
                    match param.1.parse::<usize>() {
                        Ok(h) => maze.height = h,
                        Err(_) => return Err("could not parse height".to_string()),
                    }
                },
                "solution" => maze.solution = true,
                "seed" => {
                    maze.write_seed(&param.1)?;
                },
                _ => (),
            }
        }

        Ok(maze)
    }

    #[instrument]
    fn handle_write(mut stream: TcpStream, maze: MazeParams) {
        let uri = format!("{}", &maze);
        let MazeParams{width, height, seed, solution, ..} = maze;
        let mut graph = Graph::new(width, height);
        let mut rng: ChaCha8Rng = rand::SeedableRng::from_seed(seed);
        let start = graph.space.rand_coord(&mut rng);
        let end = graph.space.rand_coord(&mut rng);
        let mut auto = Automaton::new(&mut graph);
        if let Err(err) = auto.init(&mut rng) {
            panic!("could not init: {}", err);
        }
        if let Err(err) = auto.set_start(start) {
            panic!("could not set start: {}", err);
        }
        if let Err(err) = auto.set_start(end) {
            panic!("could not set end: {}", err);
        }
        let svg = SVG::new(&graph, start, end, solution, &uri);
        let prefix = "HTTP/1.1 200 OK\r\nContent-Type: image/svg+xml; charset=UTF-8\r\n\r\n".to_string();
        let prefix = prefix + &svg.to_string();
        match stream.write(prefix.as_bytes()) {
            Ok(_) => event!(Level::INFO, "Response sent"),
            Err(e) => event!(Level::ERROR, "Failed sending response: {}", e),
        }
    }

    #[instrument]
    fn handle_err(mut stream: TcpStream, err: String) {
        event!(Level::ERROR, err);
        let resp = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n".to_string();
        let resp = resp + &err;
        match stream.write(resp.as_bytes()) {
            Ok(_) => event!(Level::WARN, "Error sent"),
            Err(e) => event!(Level::ERROR, "Failed sending error: {}", e),
        }
    }

    #[instrument]
    fn handle_client(stream: TcpStream) {
        match Self::handle_read(&stream) {
            Ok(maze) => Self::handle_write(stream, maze),
            Err(s) => Self::handle_err(stream, s),
        }
    }

    pub fn new(addr: String) -> Self {
        Self{
            addr,
        }
    }

    #[instrument]
    pub fn serve(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        event!(Level::INFO, "Listening for connections on port {}", 8080);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        Self::handle_client(stream)
                    });
                }
                Err(e) => {
                    event!(Level::ERROR, "Unable to connect: {}", e);
                }
            }
        }
    }

}
