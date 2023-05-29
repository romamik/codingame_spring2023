use std::{convert::TryInto, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
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
            _ => panic!("Invalid cell type {value}"),
        }
    }
}

#[derive(Default, Debug)]
struct GameState {
    types: Vec<CellType>,                // types by index
    resources: Vec<i32>,                 // resources by index
    neighbours: Vec<[Option<usize>; 6]>, // indexes of neighbouring cells by index
    my_ants: Vec<i32>,                   // ant count by index
    opp_ants: Vec<i32>,                  // opponent ant count by index
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
        visited[index as usize] = true;
        for &neighbour in &game_state.neighbours[index as usize] {
            if let Some(neighbour) = neighbour {
                if !visited[neighbour] {
                    queue.push((neighbour, len + 1));
                }
            }
        }
    }
    panic!("No way from {} to {}", from, to);
}

fn find_cells_by_type(game_state: &GameState, cell_type: CellType) -> Vec<usize> {
    game_state
        .types
        .iter()
        .enumerate()
        .filter_map(|(index, &t)| if t == cell_type { Some(index) } else { None })
        .collect()
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // amount of hexagonal cells in this map
    let mut game_state = GameState::default();
    for _ in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line
            .split(" ")
            .map(|it| it.trim())
            .collect::<Vec<_>>();
        let _type = parse_input!(inputs[0], i32); // 0 for empty, 1 for eggs, 2 for crystal
        let initial_resources = parse_input!(inputs[1], i32); // the initial amount of eggs/crystals on this cell
        let neighbours: Vec<_> = (0..6)
            .map(|i| parse_input!(inputs[i + 2], i32))
            .map(|i| if i < 0 { None } else { Some(i as usize) })
            .collect(); // the index of the neighbouring cell for each direction
        game_state.types.push(_type.into());
        game_state.resources.push(initial_resources);
        game_state.neighbours.push(neighbours.try_into().unwrap());
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_bases = parse_input!(input_line, usize);
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let my_base_index = parse_input!(i, usize);
        game_state.my_bases.push(my_base_index);
    }
    assert_eq!(number_of_bases, game_state.my_bases.len());
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let opp_base_index = parse_input!(i, usize);
        game_state.opp_bases.push(opp_base_index);
    }
    assert_eq!(number_of_bases, game_state.opp_bases.len());

    // game loop
    loop {
        game_state.resources.clear();
        game_state.my_ants.clear();
        game_state.opp_ants.clear();
        for i in 0..number_of_cells as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            eprintln!("{inputs:?}");
            game_state.resources.push(parse_input!(inputs[0], i32)); // the current amount of eggs/crystals on this cell
            game_state.my_ants.push(parse_input!(inputs[1], i32)); // the amount of your ants on this cell
            game_state.opp_ants.push(parse_input!(inputs[2], i32)); // the amount of opponent ants on this cell
        }

        let crystals = find_cells_by_type(&game_state, CellType::Crystal);
        eprint!("crystals: {:?}", crystals);
        let mut base_crystal_distances = vec![];
        for base in &game_state.my_bases {
            for crystal in &crystals {
                let len = find_way_len(&game_state, *base, *crystal);
                base_crystal_distances.push((*base, *crystal, len, game_state.resources[*crystal]));
            }
        }
        base_crystal_distances.sort_by_key(|(_, _, len, _)| *len);
        eprintln!("base_crystal_distances: {:?}", base_crystal_distances);

        // for now just use the nearest crystal
        base_crystal_distances.retain(|(_, _, _, resources)| *resources > 0);
        let (base, crystal, _len, _resources) = base_crystal_distances[0];
        println!("LINE {base} {crystal} 1")

        // WAIT | LINE <sourceIdx> <targetIdx> <strength> | BEACON <cellIdx> <strength> | MESSAGE <text>
        //println!("WAIT");
    }
}
