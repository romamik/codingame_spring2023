use std::{collections::VecDeque, convert::TryInto, io};

// parse single value
macro_rules! parse_single {
    ($s:expr, $t:ty) => {
        $s.trim().parse::<$t>().unwrap()
    };
}

// parse space separted values
macro_rules! parse_vec {
    ($s:expr, $t:ty) => {{
        let s = $s;
        let split = s.split_whitespace();
        let vec: Vec<$t> = split.map(|s| s.trim().parse::<$t>().unwrap()).collect();
        vec
    }};
}

// parse tuple of space separted values
macro_rules! parse_tuple {
    ($s:expr, $($t:ty),*) => {
		  {
			let s=$s;
            let split = s.split_whitespace();
            let mut iter = split.into_iter();
            (
                $(
                    iter.next().unwrap().trim().parse::<$t>().unwrap(),
                )*
            )
        }};
}

fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellType {
    Empty = 0,
    Egg = 1,
    Crystal = 2,
}

impl From<i32> for CellType {
    fn from(value: i32) -> Self {
        match value {
            0 => CellType::Empty,
            1 => CellType::Egg,
            2 => CellType::Crystal,
            _ => panic!("Invalid cell type {}", value),
        }
    }
}

#[derive(Default, Debug)]
struct GameState {
    types: Vec<CellType>,                // types by index
    resources: Vec<usize>,               // resources by index
    neighbours: Vec<[Option<usize>; 6]>, // indexes of neighbouring cells by index
    my_ants: Vec<usize>,                 // ant count by index
    opp_ants: Vec<usize>,                // opponent ant count by index
    my_bases: Vec<usize>,                // indexes of my bases
    opp_bases: Vec<usize>,               // indexes of opponent bases
}

fn find_way_len(game_state: &GameState, from: usize, to: usize) -> usize {
    let mut visited = vec![false; game_state.types.len()];
    let mut queue = vec![(from, 0)];
    while let Some((index, len)) = queue.pop() {
        if index == to {
            return len;
        }
        visited[index] = true;
        for &neighbour in &game_state.neighbours[index] {
            if let Some(neighbour) = neighbour {
                if !visited[neighbour] {
                    queue.push((neighbour, len + 1));
                }
            }
        }
    }
    panic!("No way from {} to {}", from, to);
}

fn find_nearest_cell(
    game_state: &GameState,
    from: usize,
    cell_predicate: &impl Fn(usize) -> bool,
) -> Option<(usize, usize)> {
    let mut visited = vec![false; game_state.types.len()];
    let mut queue = VecDeque::from([(from, 0)]);
    while let Some((index, len)) = queue.pop_front() {
        if index != from && cell_predicate(index) {
            return Some((index, len));
        }
        visited[index] = true;
        for &neighbour in &game_state.neighbours[index] {
            if let Some(neighbour) = neighbour {
                if !visited[neighbour] {
                    queue.push_back((neighbour, len + 1));
                }
            }
        }
    }
    None
}

fn find_nearest_cell_multi(
    game_state: &GameState,
    from: &[usize],
    cell_predicate: &impl Fn(usize) -> bool,
) -> Option<(usize, usize, usize)> {
    let mut best = None;
    for from in from {
        if let Some((index, len)) = find_nearest_cell(game_state, *from, cell_predicate) {
            if let Some((_best_from, _best_index, best_len)) = best {
                if len < best_len {
                    best = Some((*from, index, len));
                }
            } else {
                best = Some((*from, index, len));
            }
        }
    }
    best
}

fn find_cells_by_type(game_state: &GameState, cell_type: CellType) -> Vec<usize> {
    game_state
        .types
        .iter()
        .enumerate()
        .filter_map(|(index, &t)| if t == cell_type { Some(index) } else { None })
        .collect()
}

fn main() {
    let mut game_state = GameState::default();

    let number_of_cells = parse_single!(read_line(), usize); // amount of hexagonal cells in this map

    for _ in 0..number_of_cells {
        let input = parse_vec!(read_line(), i32);
        if let [_type, resources, neighbors @ ..] = input.as_slice() {
            let neighbors: [i32; 6] = neighbors.try_into().unwrap();
            game_state.types.push((*_type).into());
            game_state.resources.push(*resources as usize);
            game_state.neighbours.push(neighbors.map(|i| {
                if i < 0 {
                    None
                } else {
                    assert!(i >= 0 && i < number_of_cells as i32);
                    Some(i.try_into().unwrap())
                }
            }));
        } else {
            panic!("Invalid input")
        };
    }

    let number_of_bases = parse_single!(read_line(), usize);

    game_state.my_bases = parse_vec!(read_line(), usize);
    assert_eq!(number_of_bases, game_state.my_bases.len());

    game_state.opp_bases = parse_vec!(read_line(), usize);
    assert_eq!(number_of_bases, game_state.opp_bases.len());

    // game loop
    loop {
        game_state.resources.clear();
        game_state.my_ants.clear();
        game_state.opp_ants.clear();
        for _i in 0..number_of_cells {
            let (resources, my_ants, opp_ants) = parse_tuple!(read_line(), usize, usize, usize);
            game_state.resources.push(resources); // the current amount of eggs/crystals on this cell
            game_state.my_ants.push(my_ants); // the amount of your ants on this cell
            game_state.opp_ants.push(opp_ants); // the amount of opponent ants on this cell
        }

        //we are finding nearest cell to cells that we already have chain to, than add it to chain until run out of ants
        let mut commands = vec![];
        let mut visited = game_state.my_bases.clone();
        let mut ants: usize = game_state.my_ants.iter().sum();
        let ants_k = 2;
        loop {
            if let Some((from, to, len)) = find_nearest_cell_multi(&game_state, &visited, &|i| {
                game_state.types[i] != CellType::Empty
                    && game_state.resources[i] > 0
                    && !visited.contains(&i)
            }) {
                let use_ants = ants_k * len;
                if ants >= use_ants {
                    ants -= ants_k * len;
                } else if commands.len() == 0 {
                    ants = 0;
                } else {
                    break;
                }

                visited.push(to);
                commands.push(format!("LINE {} {} 1", from, to));
            } else {
                break;
            }
        }

        println!("{}", commands.join(";"));

        // WAIT | LINE <sourceIdx> <targetIdx> <strength> | BEACON <cellIdx> <strength> | MESSAGE <text>
        //println!("WAIT");
    }
}
