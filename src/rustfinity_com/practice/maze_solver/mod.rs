// https://www.rustfinity.com/practice/rust/challenges/maze-solver/description

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::default;

#[derive(Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Debug, Default)]
enum Tile {
  #[default]
  Wall,
  OpenPath,
}

#[derive(Debug)]
enum Flag {
  Start,
  End,
}

type Coord = (usize, usize);

#[derive(Debug, Default)]
struct Coordinate {
  x: usize,
  y: usize,
}

impl From<Coord> for Coordinate {
  fn from(coord: Coord) -> Self {
    Coordinate {
      x: coord.0,
      y: coord.1,
    }
  }
}

impl Into<Coord> for Coordinate {
  fn into(self) -> Coord {
    (self.x, self.y)
  }
}

impl Coordinate {
  fn new(x: usize, y: usize) -> Coordinate {
    Coordinate { x, y }
  }

  fn top(&self) -> Option<Self> {
    if self.y > 0 {
      return Some(Self::new(self.x, self.y - 1));
    }
    None
  }

  fn right(&self, end: Coord) -> Option<Self> {
    if self.x < end.0 {
      return Some(Self::new(self.x + 1, self.y));
    }
    None
  }

  fn bottom(&self, end: Coord) -> Option<Self> {
    if self.y < end.1 {
      return Some(Self::new(self.x, self.y + 1));
    }
    None
  }

  fn left(&self) -> Option<Self> {
    if self.x > 0 {
      return Some(Self::new(self.x - 1, self.y));
    }
    None
  }
}

impl PartialEq for Coordinate {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

#[derive(Debug, Default)]
struct NeighborsCoordinateHeuristicPriority {
  coordinate: Coordinate,
  heuristic_score: i32,
  priority: u8,
}

impl NeighborsCoordinateHeuristicPriority {
  fn new(coordinate: Coordinate) -> Self {
    Self {
      coordinate,
      ..Default::default()
    }
  }

  fn update(&mut self, heuristic_score: i32, priority: u8) -> &Self {
    self.heuristic_score = heuristic_score;
    self.priority = priority;
    self
  }
}

#[derive(Debug, Default)]
struct NodeNeighbors {
  top: Option<NeighborsCoordinateHeuristicPriority>,
  bottom: Option<NeighborsCoordinateHeuristicPriority>,
  right: Option<NeighborsCoordinateHeuristicPriority>,
  left: Option<NeighborsCoordinateHeuristicPriority>,
}

type NodeMap = HashMap<Coord, Node>;

impl NodeNeighbors {
  fn init(current: Coord, end: Coord) -> Self {
    let coordinate = Coordinate::from(current);
    let top_coordinate = coordinate.top();
    let right_coordinate = coordinate.right(end);
    let bottom_coordinate = coordinate.bottom(end);
    let left_coordinate = coordinate.left();
    let mut top = None;
    let mut right = None;
    let mut bottom = None;
    let mut left = None;
    if top_coordinate.is_some() {
      top = Some(NeighborsCoordinateHeuristicPriority::new(
        top_coordinate.unwrap(),
      ));
    }
    if right_coordinate.is_some() {
      right = Some(NeighborsCoordinateHeuristicPriority::new(
        right_coordinate.unwrap(),
      ));
    }
    if bottom_coordinate.is_some() {
      bottom = Some(NeighborsCoordinateHeuristicPriority::new(
        bottom_coordinate.unwrap(),
      ));
    }
    if left_coordinate.is_some() {
      left = Some(NeighborsCoordinateHeuristicPriority::new(
        left_coordinate.unwrap(),
      ));
    }
    Self {
      top,
      right,
      bottom,
      left,
    }
  }

  fn update(&mut self, node_map: &NodeMap) -> &Self {
    todo!();
    self
  }
}

#[derive(Debug, Default)]
struct Node {
  coordinate: Coordinate,
  neighbors: NodeNeighbors,
  heuristic_score: i32,
  next_visit_priority: Vec<Coord>,
  tile: Tile,
  flag: Option<Flag>,
  is_visited: bool,
  from: Option<Coordinate>,
}

impl Node {
  fn calculate_heuristic_score(current: Coord, end: Coord) -> i32 {
    i32::abs(end.0 as i32 - current.0 as i32) + i32::abs(end.1 as i32 - current.1 as i32)
  }

  fn init(coord: Coord, tile: Tile, end: Coord) -> Self {
    Self {
      coordinate: Coordinate::from(coord),
      neighbors: NodeNeighbors::init(coord, end),
      heuristic_score: Self::calculate_heuristic_score(coord, end),
      next_visit_priority: Vec::new(),
      tile,
      ..Default::default()
    }
  }

  fn init_with_flag(coord: Coord, tile: Tile, end: Coord, flag: Option<Flag>) -> Self {
    Self {
      flag,
      ..Self::init(coord, tile, end)
    }
  }

  fn mark_as_visited(&mut self) -> &Self {
    self.is_visited = true;
    self
  }

  fn set_from(&mut self, coord: Coordinate) -> &Self {
    if self.from.is_none() {
      self.from = Some(coord);
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
enum NodeLinkedList {
  Nil,
  Content(Node, Box<NodeLinkedList>),
}

impl NodeLinkedList {
  fn new() -> Self {
    Self::Nil
  }

  fn prepend(self, node: Node) -> Self {
    Self::Content(node, Box::new(self))
  }

  fn len(&self) -> u32 {
    match *self {
      Self::Content(_, ref tail) => 1 + tail.len(),
      Self::Nil => 0,
    }
  }
}

#[derive(Debug)]
struct Crawler {
  current_node: Node,
  nodes_crawled: NodeLinkedList,
}

impl Crawler {
  fn new(current_node: Node) -> Self {
    Self {
      current_node,
      nodes_crawled: NodeLinkedList::new(),
    }
  }

  fn crawl(&mut self, end: Coord) -> &Self {
    let top_coordinate = self.current_node.coordinate.top();
    let right_coordinate = self.current_node.coordinate.right(end);
    let bottom_coordinate = self.current_node.coordinate.bottom(end);
    let left_coordinate = self.current_node.coordinate.left();
    if top_coordinate.is_some() {}
    todo!();
    self
  }
}

#[derive(Debug)]
struct Maze {
  node_map: NodeMap,
  crawler: Crawler,
}

impl Maze {
  fn new(node_map: NodeMap, crawler: Crawler) -> Self {
    Self { node_map, crawler }
  }

  fn init(vec: Vec<Vec<char>>, start: Coord, end: Coord) -> Self {
    let flag = |coord: Coord| -> Option<Flag> {
      if coord == start {
        return Some(Flag::Start);
      } else if coord == end {
        return Some(Flag::End);
      }
      None
    };

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
        let node = Node::init_with_flag(coord, tile, end, flag(coord));
        node_map.insert(coord, node);
        y += 1;
      }
      x += 1;
    }
    Self::new(
      node_map,
      Crawler::new(Node::init(start, Tile::OpenPath, end)),
    )
  }

  fn find_node_by_coord(&self, coord: Coord) -> Option<&Node> {
    self.node_map.get(&coord)
  }

  fn solve(&self) -> Vec<Coord> {
    let vec = Vec::new();
    vec
  }
}

pub fn solve_maze(maze: Vec<Vec<char>>, start: Coord, end: Coord) -> Vec<Coord> {
  Maze::init(maze, start, end).solve()
}

#[test]
fn test() {
  let maze = vec![
    vec!['S', '.', '#', '#', '#'],
    vec!['#', '.', '#', '.', '.'],
    vec!['#', '.', '.', '.', '#'],
    vec!['#', '#', '#', '.', '#'],
    vec!['#', '#', '#', 'E', '#'],
  ];
  let start = (0, 0);
  let end = (4, 3);

  let _ = solve_maze(maze, start, end);
  // assert_eq!(
  //   path,
  //   vec![
  //     (0, 0), // starting point
  //     (0, 1), // right
  //     (1, 1), // down
  //     (2, 1), // down
  //     (2, 2), // right
  //     (2, 3), // right
  //     (3, 3), // down
  //     (4, 3)  // down
  //   ]
  // );
}
