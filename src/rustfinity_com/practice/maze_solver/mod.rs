// https://www.rustfinity.com/practice/rust/challenges/maze-solver/description

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::convert::From;

#[derive(Clone, Default, PartialEq)]
enum Tile {
  #[default]
  Wall,
  OpenPath,
}

type Coord = (usize, usize);

#[derive(Clone, Default)]
struct Coordinate(usize, usize);

impl From<&Coord> for Coordinate {
  fn from(coord: &Coord) -> Self {
    Coordinate(coord.0, coord.1)
  }
}

impl Coordinate {
  fn top(&self) -> Option<Coord> {
    if self.1 > 0 {
      return Option::from((self.0, self.1 - 1));
    }
    None
  }

  fn right(&self) -> Option<Coord> {
    Option::from((self.0 + 1, self.1))
  }

  fn bottom(&self) -> Option<Coord> {
    Option::from((self.0, self.1 + 1))
  }

  fn left(&self) -> Option<Coord> {
    if self.0 > 0 {
      return Option::from((self.0 - 1, self.1));
    }
    None
  }
}

impl PartialEq for Coordinate {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

#[derive(Clone, Default)]
struct CoordHeuristic {
  coord: Coord,
  heuristic_score: Option<i32>,
}

impl CoordHeuristic {
  fn new(coord: Coord) -> Self {
    Self {
      coord,
      heuristic_score: None,
    }
  }
}

#[derive(Clone, Default)]
struct Neighbors {
  top: Option<CoordHeuristic>,
  bottom: Option<CoordHeuristic>,
  right: Option<CoordHeuristic>,
  left: Option<CoordHeuristic>,
}

type NodeMap = HashMap<Coord, Node>;

impl Neighbors {
  fn init(current: Coord) -> Self {
    let init_coord_heuristic = |coord: Option<Coord>| match coord {
      Some(coord) => Option::from(CoordHeuristic::new(coord)),
      None => None,
    };
    let coordinate = Coordinate::from(&current);
    let top = init_coord_heuristic(coordinate.top());
    let right = init_coord_heuristic(coordinate.right());
    let bottom = init_coord_heuristic(coordinate.bottom());
    let left = init_coord_heuristic(coordinate.left());
    Self {
      top,
      right,
      bottom,
      left,
    }
  }

  fn update_coord_heuristic(&mut self, node_map: &HashMap<Coord, Node>) -> &Self {
    let try_update = |ch: &mut Option<CoordHeuristic>| {
      if let Some(ch) = ch {
        if let Some(node) = node_map.get(&ch.coord) {
          if node.tile == Tile::OpenPath {
            ch.heuristic_score = Option::from(node.heuristic_score);
          }
        }
      }
    };
    try_update(&mut self.top);
    try_update(&mut self.right);
    try_update(&mut self.bottom);
    try_update(&mut self.left);
    if self.top.is_some() {
      if self.top.clone().unwrap().heuristic_score.is_none() {
        self.top = None;
      }
    }
    if self.right.is_some() {
      if self.right.clone().unwrap().heuristic_score.is_none() {
        self.right = None;
      }
    }
    if self.bottom.is_some() {
      if self.bottom.clone().unwrap().heuristic_score.is_none() {
        self.bottom = None;
      }
    }
    if self.left.is_some() {
      if self.left.clone().unwrap().heuristic_score.is_none() {
        self.left = None;
      }
    }
    self
  }
}

#[derive(Clone, Default)]
struct Node {
  coordinate: Coordinate,
  neighbors: Neighbors,
  heuristic_score: i32,
  tile: Tile,
  is_visited: bool,
  from: Option<Coord>,
}

impl Node {
  fn calculate_heuristic_score(current: &Coord, end: &Coord) -> i32 {
    i32::abs(end.0 as i32 - current.0 as i32) + i32::abs(end.1 as i32 - current.1 as i32)
  }

  fn init(coord: Coord, tile: Tile, end: &Coord) -> Self {
    Self {
      coordinate: Coordinate::from(&coord),
      neighbors: Neighbors::init(coord),
      heuristic_score: Self::calculate_heuristic_score(&coord, end),
      tile,
      ..Default::default()
    }
  }

  fn mark_as_visited(&mut self) -> &Self {
    self.is_visited = true;
    self
  }

  fn set_from(&mut self, coord: Coord) -> &Self {
    if self.from.is_none() {
      self.from = Option::from(coord);
    }
    self
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.coordinate == other.coordinate
  }
}

#[derive(Debug)]
struct Crawler {
  current_node: Coord,
  nodes_crawled: Vec<Coord>,
}

impl Crawler {
  fn new(current_node: Coord) -> Self {
    Self {
      current_node,
      nodes_crawled: vec![current_node],
    }
  }

  fn crawl(&mut self, node_map: &mut HashMap<Coord, Node>, end: &Coord) -> Vec<Coord> {
    self.nodes_crawled = vec![self.current_node];

    let mut is_dead_end = false;
    loop {
      if is_dead_end {
        self.nodes_crawled.clear();
        break;
      }
      if self.current_node == *end {
        break;
      }
      let mut node = node_map.get(&self.current_node).unwrap().to_owned();
      let mut neighbors: Vec<CoordHeuristic> = Vec::new();
      let mut try_add = |ch: &Option<CoordHeuristic>| {
        if let Some(ch) = ch {
          neighbors.push(ch.clone());
        }
      };
      try_add(&node.neighbors.top);
      try_add(&node.neighbors.right);
      try_add(&node.neighbors.bottom);
      try_add(&node.neighbors.left);
      neighbors.sort_by(|a, b| a.heuristic_score.cmp(&b.heuristic_score));

      let mut i = 0;
      let mut is_forward = false;
      loop {
        if i == neighbors.len() {
          break;
        }
        let mut next_node = node_map.get(&neighbors[i].coord).unwrap().to_owned();
        if !next_node.is_visited {
          is_forward = true;
          node.mark_as_visited();
          node_map.insert(self.current_node, node.to_owned());
          next_node.set_from(self.current_node);
          node_map.insert(neighbors[i].coord, next_node.to_owned());
          self.nodes_crawled.push(neighbors[i].coord);
          self.current_node = neighbors[i].coord;
          break;
        }
        i += 1;
      }
      if !is_forward {
        let prev_coord = node.from;
        if prev_coord.is_some() {
          node.mark_as_visited();
          node_map.insert(self.current_node, node.to_owned());
          self.nodes_crawled.pop();
          self.current_node = prev_coord.unwrap();
        } else {
          is_dead_end = true;
        }
      }
    }
    self.nodes_crawled.clone()
  }
}

struct Maze {
  node_map: NodeMap,
  crawler: Crawler,
  end: Coord,
}

impl Maze {
  fn new(node_map: NodeMap, crawler: Crawler, end: Coord) -> Self {
    Self {
      node_map,
      crawler,
      end,
    }
  }

  fn init(vec: Vec<Vec<char>>, start: Coord, end: Coord) -> Self {
    let mut node_map: NodeMap = HashMap::new();
    let mut x = 0;
    loop {
      if x == vec.len() {
        break;
      }
      let inner_maze = vec.get(x).unwrap();
      let mut y = 0;
      loop {
        if y == inner_maze.len() {
          break;
        }
        let coord = (x, y);
        let tile = match inner_maze.get(y).unwrap() {
          '.' | 'S' | 'E' => Tile::OpenPath,
          _ => Tile::Wall,
        };
        let node = Node::init(coord, tile, &end);
        node_map.insert(coord, node);
        y += 1;
      }
      x += 1;
    }
    let temp_node_map = node_map.clone();
    for m in node_map.iter_mut() {
      m.1.neighbors.update_coord_heuristic(&temp_node_map);
    }
    Self::new(node_map, Crawler::new(start), end)
  }

  fn solve(&mut self) -> Vec<Coord> {
    self.crawler.crawl(&mut self.node_map, &self.end)
  }
}

pub fn solve_maze(maze: Vec<Vec<char>>, start: Coord, end: Coord) -> Vec<Coord> {
  Maze::init(maze, start, end).solve()
}

#[test]
fn test() {
  let maze = vec![
    vec!['S', '.', '.', '#', '#'],
    vec!['#', '.', '#', '.', '.'],
    vec!['#', '.', '.', '.', '#'],
    vec!['#', '.', '#', '.', '#'],
    vec!['#', '.', '#', 'E', '#'],
  ];
  let start = (0, 0);
  let end = (4, 3);
  let path = solve_maze(maze, start, end);
  assert_eq!(
    path,
    vec![
      (0, 0), // starting point
      (0, 1), // right
      (1, 1), // down
      (2, 1), // down
      (2, 2), // right
      (2, 3), // right
      (3, 3), // down
      (4, 3) // down
    ]
  );

  let maze = vec![
    vec!['S', '#', '#', '#', '#'],
    vec!['#', '#', '#', '#', '#'],
    vec!['#', '#', '#', '#', '#'],
    vec!['#', '#', '#', '#', '#'],
    vec!['#', '#', '#', 'E', '#']
  ];
  let path = solve_maze(maze, start, end);
  assert_eq!(
    path,
    vec![]
  );

  let maze = vec![
    vec!['S', '.', '.', '#', 'E'],
    vec!['#', '#', '.', '#', '#'],
    vec!['#', '.', '.', '.', '#'],
    vec!['#', '#', '#', '.', '#'],
    vec!['#', '#', '#', '#', '#']
  ];
  let end = (0, 4);
  let path = solve_maze(maze, start, end);
  assert_eq!(
    path,
    vec![]
  );
}
