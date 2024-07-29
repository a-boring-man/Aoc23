use aoc_lib::{read_lines_byte, read_lines_string};

#[derive(Debug, Clone, Copy)]
struct Coordinate2D {
    pub horizontal: usize,
    pub vertical: usize,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct HeatMap {
    map: Vec<Vec<u8>>,
    horizontal_length: usize,
    vertical_length: usize,
}

#[derive(Debug, Clone)]
struct Path {
    pub coordinate: Coordinate2D,
    pub last_direction: Option<Direction>,
    pub last_last_direction: Option<Direction>,
    pub last_last_last_direction: Option<Direction>,
    pub sum: u32,
    pub has_been_diffused: bool,
}

#[derive(Debug, Clone)]
struct Factory {
    map: Vec<Vec<Vec<Path>>>,
    horizontal_length: usize,
    vertical_length: usize,
}

impl HeatMap {
    fn new_from_input(path: &str) -> Self {
        let map: Vec<Vec<u8>> = read_lines_string(path)
            .unwrap()
            .into_iter()
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();
        Self {
            horizontal_length: map.len(),
            vertical_length: map[0].len(),
            map,
        }
    }

    fn get_hotness(&self, horizontal_coordinate: usize, vertical_coordinate: usize) -> Option<u8> {
        if horizontal_coordinate > self.horizontal_length
        || vertical_coordinate > self.vertical_length
        {
            return None;
        }
        Some(self.map[vertical_coordinate][horizontal_coordinate])
    }
}

fn create_slice(heat_map: &HeatMap) -> Vec<Vec<Path>> {
    let mut slice = Vec::new();
    for i in 0..heat_map.vertical_length {
        slice.push(Vec::with_capacity(12))
    };
    slice
}

impl Factory {
    fn new(heat_map: &HeatMap) -> Self {
        let first_path = Path {
            coordinate: Coordinate2D {
                horizontal: 0,
                vertical: 0,
            },
            last_direction: None,
            last_last_direction: None,
            last_last_last_direction: None,
            sum: 0,
            has_been_diffused: false,
        };
        let mut map = Vec::new();
        for _i in 0..heat_map.horizontal_length {
            map.push(create_slice(heat_map))
        }
        map[0][0].push(first_path);
        Self {
            map,
            horizontal_length: heat_map.horizontal_length,
            vertical_length: heat_map.vertical_length,
        }
    }

    fn try_to_insert_path(&mut self, path: &Path) {
        let present_path = &mut self.map[path.coordinate.vertical][path.coordinate.horizontal]; // get the list of path that have been to this cell
        match present_path.iter_mut().find(|old_path| Path::can_they_go_to_the_same_place(&old_path, path)) { // check if there is a path that can go to the same place
            Some(same_path) => { // if yes then keep the smallest
                if same_path.sum > path.sum {
                    *same_path = path.to_owned();
                }
            },
            None => { // else add it to the cell
                present_path.push(path.to_owned());
            },
        }
    }

    fn there_are_still_path_that_need_to_be_propagate(&self) -> bool {
        self.map
            .iter()
            .flat_map(|line| line.iter())
            .flat_map(|cell| cell.iter())
            .any(|path| !path.has_been_diffused)
    }

    fn propagate_every_path(&mut self, heat_map: &HeatMap) {
        let new_paths_to_difuse: Vec<Path> = self.map.iter_mut().flat_map(|line| { // for each line in the grid
            line.iter_mut().flat_map(|cell| { // for each cell in a line
                if cell.len() > 12 { // just to see if i was right on my only 12 type of path prediction
                    // println!("HAAAA"); // turn out i was wrong/ made a mistake in can_they_go_to_the_same_place function
                }
                cell.iter_mut() // for each path in a cell
                    .filter(|path| !path.has_been_diffused) // find all path that have not been diffused
                    .flat_map(|path| {
                        let salut = path.generate_all_new_coordinate( // generate the new path based on the one found
                            self.horizontal_length - 1,
                            self.vertical_length - 1,
                            heat_map,
                        );
                        // println!("new_path: {:?}", salut);
                        salut
                    })
                })
        }).collect(); // a big vec of all the new paths that need to be inserted into the factory
        // println!("new_path to difuse: {:?}", new_paths_to_difuse);
        new_paths_to_difuse
            .iter() // for each path
            .for_each(|new_path| self.try_to_insert_path(new_path)); // insert/replace worst path in the factory
    }
}


impl Path {
    /// if two path can go to the exact same place then we just need to keep the one with the smallest sum
    /// this function can be greatly optimized, to reduce the number of potential different path to 12
    /// but i lake the brain power to do so now...
    fn can_they_go_to_the_same_place(first_path: &Path, second_path: &Path) -> bool {
        // println!("at leat they do the check");
        first_path.last_direction == second_path.last_direction
            && ((first_path.last_last_direction != first_path.last_direction
            && second_path.last_last_direction != second_path.last_direction) ||
                (first_path.last_last_direction == second_path.last_last_direction && (
                    first_path.last_last_last_direction != first_path.last_last_direction &&
                    second_path.last_last_last_direction != second_path.last_last_direction
                )) || (first_path.last_last_last_direction == second_path.last_last_last_direction && first_path.last_last_direction == second_path.last_last_direction)
        )
    }

    /// generate all the possible path based on the current coordinate and last direction
    fn generate_all_new_coordinate(
        &mut self,
        max_horizontal: usize,
        max_vertical: usize,
        heat_map: &HeatMap,
    ) -> Vec<Path> {
        let mut new_paths = Vec::new();
        let right = self.coordinate.horizontal + 1;
        if right <= max_horizontal // the position exist
            && (self.last_last_last_direction.is_none() // one of the last direction is non AKA this path as not yet walked 3 cell
                || self.last_last_direction.is_none()
                || self.last_direction.is_none()
                || !(self.last_direction == Some(Direction::Right) // he didn't just go right right right
                    && self.last_last_direction == Some(Direction::Right)
                    && self.last_last_last_direction == Some(Direction::Right))) && self.last_direction != Some(Direction::Left) // no 180 degree turn
        {
            let new_coordinate = Coordinate2D {
                horizontal: right,
                vertical: self.coordinate.vertical,
            };
            let mut new_path = self.clone();
            new_path.coordinate = new_coordinate;
            new_path.last_last_last_direction = self.last_last_direction;
            new_path.last_last_direction = self.last_direction;
            new_path.last_direction = Some(Direction::Right);
            new_path.sum += heat_map
                .get_hotness(new_coordinate.horizontal, new_coordinate.vertical)
                .expect("asking for wrong coordinate") as u32;
            new_paths.push(new_path);
        }
        let down = self.coordinate.vertical + 1;
        if down <= max_vertical
            && (self.last_last_last_direction.is_none()
                || self.last_last_direction.is_none()
                || self.last_direction.is_none()
                || !(self.last_direction == Some(Direction::Down)
                    && self.last_last_direction == Some(Direction::Down)
                    && self.last_last_last_direction == Some(Direction::Down))) && self.last_direction != Some(Direction::Up)
        {
            let new_coordinate = Coordinate2D {
                horizontal: self.coordinate.horizontal,
                vertical: down,
            };
            let mut new_path = self.clone();
            new_path.coordinate = new_coordinate;
            new_path.last_last_last_direction = self.last_last_direction;
            new_path.last_last_direction = self.last_direction;
            new_path.last_direction = Some(Direction::Down);
            new_path.sum += heat_map
                .get_hotness(new_coordinate.horizontal, new_coordinate.vertical)
                .expect("asking for wrong coordinate") as u32;
            new_paths.push(new_path);
        }
        let (left, overflowing) = self.coordinate.horizontal.overflowing_sub(1);
        match overflowing {
            false => {
                if self.last_last_last_direction.is_none()
                    || self.last_last_direction.is_none()
                    || self.last_direction.is_none()
                    || !(self.last_direction == Some(Direction::Left)
                        && self.last_last_direction == Some(Direction::Left)
                        && self.last_last_last_direction == Some(Direction::Left)) && self.last_direction != Some(Direction::Right)
                {
                    let new_coordinate = Coordinate2D {
                        horizontal: left,
                        vertical: self.coordinate.vertical,
                    };
                    let mut new_path = self.clone();
                    new_path.coordinate = new_coordinate;
                    new_path.last_last_last_direction = self.last_last_direction;
                    new_path.last_last_direction = self.last_direction;
                    new_path.last_direction = Some(Direction::Left);
                    new_path.sum += heat_map
                        .get_hotness(new_coordinate.horizontal, new_coordinate.vertical)
                        .expect("asking for wrong coordinate")
                        as u32;
                    new_paths.push(new_path);
                }
            }
            true => {}
        };
        let (up, overflowing) = self.coordinate.vertical.overflowing_sub(1);
        match overflowing {
            false => {
                if self.last_last_last_direction.is_none()
                    || self.last_last_direction.is_none()
                    || self.last_direction.is_none()
                    || !(self.last_direction == Some(Direction::Up)
                        && self.last_last_direction == Some(Direction::Up)
                        && self.last_last_last_direction == Some(Direction::Up)) && self.last_direction != Some(Direction::Down)
                {
                    let new_coordinate = Coordinate2D {
                        horizontal: self.coordinate.horizontal,
                        vertical: up,
                    };
                    let mut new_path = self.clone();
                    new_path.coordinate = new_coordinate;
                    new_path.last_last_last_direction = self.last_last_direction;
                    new_path.last_last_direction = self.last_direction;
                    new_path.last_direction = Some(Direction::Up);
                    new_path.sum += heat_map
                        .get_hotness(new_coordinate.horizontal, new_coordinate.vertical)
                        .expect("asking for wrong coordinate")
                        as u32;
                    new_paths.push(new_path);
                }
            }
            true => {}
        };
        self.has_been_diffused = true;
        new_paths
    }
}


fn main() {
    let heat_map = HeatMap::new_from_input("input/day17.txt"); // the path to the input
    println!("heat map: {heat_map:?}");
    let mut factory = Factory::new(&heat_map);
    println!("factory: {factory:?}");
    let mut number_of_propagation = 0;
    while factory.there_are_still_path_that_need_to_be_propagate() && number_of_propagation < 4000 {
        factory.propagate_every_path(&heat_map);
        println!("propagate: {number_of_propagation} time");
        number_of_propagation += 1;
        if number_of_propagation == 300 {
            // println!("mid process : {:?}", factory.map);
        }
    }
    let end_vec = factory.map[factory.vertical_length - 1][factory.horizontal_length - 1].clone();
    let best_path = end_vec.iter().min_by(|path1, path2| path1.sum.partial_cmp(&path2.sum).unwrap());
    println!(
        "end vec: {:?}",
        factory.map[factory.vertical_length - 1][factory.horizontal_length - 1]
    );
    println!("the best path sum was: {:?}", best_path.unwrap().sum);
}
