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

fn find_nearest_cell_multi(
    game_state: &GameState,
    from: &[usize],
    cell_predicate: &impl Fn(usize) -> bool,
) -> Option<Vec<usize>> {
    let mut visited = vec![None; game_state.types.len()];
    let mut queue = VecDeque::new();
    fn is_start_pt(visited: &[Option<usize>], i: usize) -> bool {
        matches!(visited[i], Some(j) if j == i)
    }
    for &index in from {
        queue.push_back(index);
        visited[index] = Some(index);
    }
    while let Some(index) = queue.pop_front() {
        if cell_predicate(index) && !is_start_pt(&visited, index) {
            let mut result = vec![];
            let mut index = index;
            while let Some(prev_index) = visited[index] {
                if is_start_pt(&visited, index) {
                    break;
                }
                result.push(index);
                index = prev_index;
            }
            return Some(result);
        }
        for &neighbour in &game_state.neighbours[index] {
            if let Some(neighbour) = neighbour {
                if visited[neighbour].is_none() {
                    visited[neighbour] = Some(index);
                    queue.push_back(neighbour);
                }
            }
        }
    }
    None
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
        let (_my_score, _op_score) = parse_tuple!(read_line(), usize, usize);

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
        let mut visited = game_state.my_bases.clone();
        let total_ants: usize = game_state.my_ants.iter().sum();
        let mut free_ants = total_ants;
        let ants_k = 2;
        while let Some(new_points) = find_nearest_cell_multi(&game_state, &visited, &|i| {
            game_state.types[i] != CellType::Empty && game_state.resources[i] > 0
        }) {
            eprintln!("new_points: {:?}", new_points);
            eprintln!("free_ants: {:?}", free_ants);
            let need_ants = new_points.len() * ants_k;
            if free_ants >= need_ants || visited.len() == game_state.my_bases.len() {
                free_ants -= need_ants.min(free_ants);
                visited.extend(new_points);
            } else {
                break;
            }
        }

        struct Beackon {
            index: usize,
            strength: usize,
            resources: usize,
            //my_ants: usize,
            opp_ants: usize,
        }

        let mut beackons = visited
            .iter()
            .map(|&i| Beackon {
                index: i,
                strength: ants_k, // how many ants we want to send to this cell
                resources: game_state.resources[i],
                //my_ants: game_state.my_ants[i], // how many ants we have on this cell
                opp_ants: game_state.opp_ants[i],
            })
            .collect::<Vec<_>>();

        for b in beackons.iter_mut() {
            if b.strength < b.opp_ants && b.resources > 0 {
                let diff = b.opp_ants - b.strength;
                if free_ants >= diff {
                    b.strength += diff;
                    free_ants -= diff;
                }
            }
        }

        let commands = beackons
            .iter()
            .map(|b| format!("BEACON {} {}", b.index, b.strength))
            .collect::<Vec<_>>();

        println!("{}", commands.join(";"));

        // WAIT | LINE <sourceIdx> <targetIdx> <strength> | BEACON <cellIdx> <strength> | MESSAGE <text>
        //println!("WAIT");
    }
}
