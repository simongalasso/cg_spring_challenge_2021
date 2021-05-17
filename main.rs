/* CHANGELOGS
    - fix range bug
*/

use std::io;
use std::time::{Instant};
use std::fmt;
use rand::Rng;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().expect("P0"))
}

#[derive(Copy, Clone, Debug)]
struct Tree {
    is_mine: bool,
    size: i32,
    asleep: bool,
}

impl Tree {
    fn new() -> Self {
        return Self {
            is_mine: false,
            size: 0,
            asleep: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    richness: i32,
    tree: Option<Tree>,
    neighbors: [i32; 6]
}

impl Cell {
    fn new() -> Self {
        return Self {
            richness: 0,
            tree: None,
            neighbors: [0; 6]
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Action {
    cell_id: usize,
    seed_cell_id: usize,
    action: String
}

impl Action {
    fn new(action: String) -> Self {
        return Self {
            cell_id: 0,
            seed_cell_id: 0,
            action: action
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.action[..] {
            "SEED" => write!(f, "{} {} -> {}", self.action, self.cell_id, self.seed_cell_id),
            "WAIT" => write!(f, "WAIT"),
            _ => write!(f, "{} {}", self.action, self.cell_id)
        }
    }
}

#[derive(Clone, Debug)]
struct Game {
    turn: usize,
    day: i32,
    nutrients: i32,
    sun: i32,
    score: f32,
    opp_sun: i32,
    opp_score: f32,
    opp_is_waiting: i32,
    number_of_trees: i32,
    forest: [Cell; 37],
}

/* ------------------------------------------------------------ */
/* - Parsing -------------------------------------------------- */
/* ------------------------------------------------------------ */

fn parse_forest(number_of_cells: usize) -> [Cell; 37] {
    let mut forest: [Cell; 37] = [Cell::new(); 37];
    for i in 0..number_of_cells as usize {
        let mut cell: Cell = Cell::new();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("P1");
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], usize); // 0 is the center cell, the next cells spiral outwards
        cell.richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        cell.neighbors = [
            parse_input!(inputs[2], i32),
            parse_input!(inputs[3], i32),
            parse_input!(inputs[4], i32),
            parse_input!(inputs[5], i32),
            parse_input!(inputs[6], i32),
            parse_input!(inputs[7], i32)
        ];
        forest[index as usize] = cell;
    }
    return forest;
}

fn get_turn_informations(number_of_cells: usize, forest: &mut [Cell; 37], turn: usize) -> Game {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P2");
    let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P3");
    let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P4");
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let sun = parse_input!(inputs[0], i32); // your sun points
    let score = parse_input!(inputs[1], f32); // your current score
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P5");
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
    let opp_score = parse_input!(inputs[1], f32); // opponent's score
    let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P6");
    let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
    for i in 0..number_of_cells { // to reset forest
        forest[i].tree = None;
    }
    for i in 0..number_of_trees {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("P7");
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let cell_index = parse_input!(inputs[0], usize);
        forest[cell_index].tree = Some(Tree {
            is_mine: parse_input!(inputs[2], u16) == 1,
            size: parse_input!(inputs[1], i32),
            asleep: parse_input!(inputs[3], i32) == 1,
        });
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("P11");
    let number_of_possible_moves = parse_input!(input_line, i32);
    for i in 0..number_of_possible_moves as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("P12");
        let possible_move = input_line.trim_matches('\n').to_string();
    }
    return Game {
        turn: turn,
        day: day,
        nutrients: nutrients,
        sun: sun,
        score: score,
        opp_sun: opp_sun,
        opp_score: opp_score,
        opp_is_waiting: opp_is_waiting,
        number_of_trees: number_of_trees,
        forest: forest.clone(),
    };
}

/* ------------------------------------------------------------ */
/* - Functions ------------------------------------------------ */
/* ------------------------------------------------------------ */

fn get_my_number_of_tree(game: &Game) -> i32 {
    return game.forest.iter().filter(|cell| match &cell.tree {
        Some(tree) => tree.is_mine,
        None => false
    }).count() as i32;
}

fn get_my_number_of_tree_of_size(size: i32, game: &Game) -> i32 {
    return game.forest.iter().filter(|cell| match &cell.tree {
        Some(tree) => tree.is_mine && tree.size == size,
        None => false
    }).count() as i32;
}

fn get_sun_rays(day: i32) -> Vec<usize> {
    return match day % 6 {
        0 => vec![25, 26, 27, 28, 29, 30, 31],
        1 => vec![28, 29, 30, 31, 32, 33, 34],
        2 => vec![31, 32, 33, 34, 35, 36, 19],
        3 => vec![34, 35, 36, 19, 20, 21, 22],
        4 => vec![19, 20, 21, 22, 23, 24, 25],
        _ => vec![22, 23, 24, 25, 26, 27, 28]
    }
}

fn is_shadowed(game: &Game, cycle: usize, size: i32, only_mine: bool, cell_index: usize) -> bool {
    let dir: usize = ((cycle + 3) % 6);
    let mut dist: i32 = 0;
    let mut next_n = game.forest[cell_index].neighbors[dir];
    while next_n != -1 && dist < 3 {
        match game.forest[next_n as usize].tree {
            Some(tree) => if (tree.is_mine || !only_mine) && tree.size > dist && tree.size >= size {
                return true;
            },
            _ => {}
        }
        next_n = game.forest[next_n as usize].neighbors[dir];
        dist += 1;
    }
    return false;
}

fn shadow_sensitiveness(game: &Game, range: std::ops::Range<usize>, size: i32, only_mine: bool, cell_index: usize) -> i32 {
    let mut shadow_day: i32 = 0;
    for day in range {
        let cycle: usize = day % 6;
        if is_shadowed(game, cycle, size, only_mine, cell_index) {
            shadow_day += 1;
        }
    }
    return shadow_day;
}

// possible amélioration en prenant en compte une éventuelle position intermédiaire de l'arbre courant entre deux autres arbres
fn nb_opp_sun_lost_by(game: &Game, range: std::ops::Range<usize>, cell_id: usize, size: i32) -> i32 {
    let mut nb: i32 = 0;
    for day in range {
        let cycle: usize = day % 6;
        let mut dist: i32 = 0;
        let mut next_n = game.forest[cell_id].neighbors[cycle];
        let mut max_tree_size: i32 = 0;
        while next_n != -1 && dist < size {
            match game.forest[next_n as usize].tree {
                Some(tree) => if tree.size <= size && tree.size > max_tree_size {
                    if !tree.is_mine {
                        nb += tree.size;
                    }
                    max_tree_size = tree.size;
                },
                _ => {}
            }
            next_n = game.forest[next_n as usize].neighbors[cycle];
            dist += 1;
        }
    }
    return nb;
}

// possible amélioration en prenant en compte une éventuelle position intermédiaire de l'arbre courant entre deux autres arbres
fn nb_own_sun_lost_by(game: &Game, range: std::ops::Range<usize>, cell_id: usize, size: i32) -> i32 {
    let mut nb: i32 = 0;
    for day in range {
        let cycle: usize = day % 6;
        let mut dist: i32 = 0;
        let mut next_n = game.forest[cell_id].neighbors[cycle];
        let mut max_tree_size: i32 = 0;
        while next_n != -1 && dist < size {
            match game.forest[next_n as usize].tree {
                Some(tree) => if tree.size <= size && tree.size > max_tree_size {
                    if tree.is_mine {
                        nb += tree.size;
                    }
                    max_tree_size = tree.size;
                },
                _ => {}
            }
            next_n = game.forest[next_n as usize].neighbors[cycle];
            dist += 1;
        }
    }
    return nb;
}

fn sun_rate(cell_id: usize, size: i32, range: std::ops::Range<usize>, game: &Game) -> f32 {
    let nb_opp_sun_lost: i32 = nb_opp_sun_lost_by(game, range.clone(), cell_id, size); // [0..(6*range.len())]
    let nb_own_sun_lost: i32 = nb_own_sun_lost_by(game, range.clone(), cell_id, size); // [0..(6*range.len())]
    let nb_sun_won: i32 = (range.len() as i32 - shadow_sensitiveness(game, range.clone(), size, false, cell_id)) * size; // [0..(size*range.len()]
    let mut sun_rate: f32 = (nb_sun_won - nb_own_sun_lost + nb_opp_sun_lost) as f32; // [(-6*range.len())..(6*range.len()+size*range.len())]
    return sun_rate;
}

/* ------------------------------------------------------------ */
/* - Neighbors ------------------------------------------------ */
/* ------------------------------------------------------------ */

fn get_available_completes(game: &Game) -> Vec<Action> {
    let mut available_completes: Vec<Action> = vec![];
    for (i, cell) in game.forest.iter().enumerate() {
        match &cell.tree {
            Some(tree) => if !tree.asleep 
                && tree.is_mine 
                && tree.size == 3 
                && game.sun >= 4
            {
                available_completes.push(Action { cell_id: i, seed_cell_id: 0, action: "COMPLETE".to_string() });
            }
            None => {}
        }
    }
    return available_completes;
}

fn get_available_grows(game: &Game) -> Vec<Action> {
    let mut available_grows: Vec<Action> = vec![];
    for (i, cell) in game.forest.iter().enumerate() {
        match &cell.tree {
            Some(tree) => if !tree.asleep
                && tree.is_mine
                && tree.size < 3
                && game.sun >= ((tree.size as i32 * 4) - 1).abs() as i32 + get_my_number_of_tree_of_size(tree.size + 1, &game)
            {
                available_grows.push(Action { cell_id: i, seed_cell_id: 0, action: "GROW".to_string() });
            }
            None => {}
        }
    }
    return available_grows;
}

fn get_available_seed_position_from(base_cell: usize, cell_index: usize, dist: i32, size: i32, positions: &mut Vec<usize>, game: &Game) -> Vec<usize> {
    if dist < size {
        for n_cell in game.forest[cell_index].neighbors.iter().filter(|&n| *n > -1) {
            if game.forest[*n_cell as usize].tree.is_none()
                && !positions.contains(&(*n_cell as usize))
                && game.forest[*n_cell as usize].richness > 0
            {
                positions.push(*n_cell as usize);
            }
            get_available_seed_position_from(base_cell, *n_cell as usize, dist + 1, size, positions, game);
        }
    }
    return positions.clone();
}

fn get_available_seeds(game: &Game) -> Vec<Action> {
    let mut available_seeds: Vec<Action> = vec![];
    for (i, cell) in game.forest.iter().enumerate()  {
        match &cell.tree {
            Some(tree) => if !tree.asleep
                && tree.size > 1 // > 1 optional, > 0 mandatory
                && tree.is_mine
                && game.sun >= get_my_number_of_tree_of_size(0, &game)
            {
                for pos in get_available_seed_position_from(i, i, 0, tree.size, &mut vec![], game).iter() {
                    available_seeds.push(Action {
                        cell_id: i,
                        seed_cell_id: *pos,
                        action: "SEED".to_string()
                    });
                }
            }
            None => {}
        }
    }
    return available_seeds;
}

fn get_neighbors(game: &Game) -> Vec<Action> {
    let mut ret: Vec<Action> = [
        &get_available_grows(&game)[..],
        &get_available_completes(&game)[..],
        &get_available_seeds(&game)[..]
    ].concat();
    ret.push(Action { cell_id: 0, seed_cell_id: 0, action: match game.sun > 10 {
        true => "WAIT STUPID".to_string(),
        _ => "WAIT".to_string()
    }.to_string()});
    return ret;
}

/* ------------------------------------------------------------ */
/* - Algo ----------------------------------------------------- */
/* ------------------------------------------------------------ */

fn find_best_action(game: &Game, start_time: std::time::Instant) -> Action {
    let actions: Vec<Action> = get_neighbors(game);
    let seed_actions: Vec<&Action> = actions.iter().filter(|a| a.action == "SEED").collect::<Vec<&Action>>();
    let grow_actions: Vec<&Action> = actions.iter().filter(|a| a.action == "GROW").collect::<Vec<&Action>>();
    let comp_actions: Vec<&Action> = actions.iter().filter(|a| a.action == "COMPLETE").collect::<Vec<&Action>>();

    let next_day_range: std::ops::Range<usize> = (game.day as usize + 1)..(game.day as usize + 2);

    if get_my_number_of_tree_of_size(3, &game) >= 20 - game.day { // 23 - game.day to be correct but works better like it
        let mut best_score: f32 = std::f32::MAX;
        let mut best_action: Option<&Action> = None;

        for action in comp_actions.iter() {
            let sun_rate_next_day: f32 = sun_rate(action.cell_id, 3, next_day_range.clone(), game); // [-6..9]
            if sun_rate_next_day < best_score || (sun_rate_next_day == best_score && sun_rate(action.cell_id, 3, 0..6, game) < sun_rate(best_action.as_ref().unwrap().cell_id, 3, 0..6, game)) {
                best_score = sun_rate_next_day;
                best_action = Some(action);
            }
            eprintln!("{}: {}", action, sun_rate_next_day);
        }
        if best_action.is_some()
            && game.nutrients + (game.forest[best_action.as_ref().unwrap().cell_id].richness * 2 - 2) > 0 // FIXME, opti maybe, to check
        {
            return best_action.unwrap().clone();
        }
    }

    if !grow_actions.is_empty() {
        let mut best_score: f32 = std::f32::MIN;
        let mut best_action: Option<&Action> = None;

        for action in grow_actions.iter() {
            let size: i32 = game.forest[action.cell_id].tree.as_ref().unwrap().size;

            let current_sun_rate_next_day: f32 = sun_rate(action.cell_id, size, next_day_range.clone(), game); // [-6..9]
            let sun_rate_next_day: f32 = sun_rate(action.cell_id, size + 1, next_day_range.clone(), game); // [-6..9]

            let diff: f32 = sun_rate_next_day - current_sun_rate_next_day;

            if (diff > best_score || (diff == best_score && sun_rate(action.cell_id, size + 1, 0..6, game) - sun_rate(best_action.as_ref().unwrap().cell_id, game.forest[best_action.as_ref().unwrap().cell_id].tree.as_ref().unwrap().size + 1, 0..6, game) > 0.0))
                && (diff > 0.0 || (sun_rate_next_day == current_sun_rate_next_day && sun_rate(action.cell_id, size + 1, 0..6, game) > sun_rate(action.cell_id, size, 0..6, game)))
            {
                best_score = sun_rate_next_day;
                best_action = Some(action);
            }
            eprintln!("{}: {}", action, sun_rate_next_day);
        }
        if best_action.is_some()
            && game.day < 23 - (3 - (game.forest[best_action.as_ref().unwrap().cell_id].tree.as_ref().unwrap().size + 1)) // FIXME, check if it work, not sure
        {
            return best_action.unwrap().clone();
        }
    }

    if !seed_actions.is_empty() && get_my_number_of_tree_of_size(0, &game) == 0 {
        let mut best_score: f32 = std::f32::MIN;
        let mut best_action: Option<&Action> = None;

        for action in seed_actions.iter() {
            let mut score: f32 = (6.0 - shadow_sensitiveness(game, 0..6, 0, true, action.seed_cell_id) as f32) / 6.0; // [0..1]
            if score > best_score || (score == best_score && (game.forest[action.seed_cell_id].richness * 2 - 2) > (game.forest[best_action.as_ref().unwrap().seed_cell_id].richness * 2 - 2)) {
                best_score = score;
                best_action = Some(action);
            }
            eprintln!("{}: {}", action, score);
        }
        if best_action.is_some()
            && game.day < 20
        {
            return best_action.unwrap().clone();
        }
    }

    return Action::new(match rand::thread_rng().gen::<usize>() % 3 {
        1 => "WAIT BOUP".to_string(),
        2 => "WAIT BIIIP".to_string(),
        _ => "WAIT BIP".to_string()
    });
}

/* ------------------------------------------------------------ */
/* - Main ----------------------------------------------------- */
/* ------------------------------------------------------------ */

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("");
    let number_of_cells = parse_input!(input_line, usize); // 37
    let mut forest: [Cell; 37] = parse_forest(number_of_cells);
    let mut base_turn: usize = 0;
    loop {
        base_turn += 1;
        let mut game: Game = get_turn_informations(number_of_cells, &mut forest, base_turn);
        let start_time = Instant::now();

        let mut best_action: Action = find_best_action(&game, start_time);
        eprintln!("search duration: {:.3?}", start_time.elapsed());

        match &best_action.action[..] {
            "GROW" => println!("GROW {}", best_action.cell_id),
            "COMPLETE" => println!("COMPLETE {}", best_action.cell_id),
            "SEED" => println!("SEED {} {}", best_action.cell_id, best_action.seed_cell_id),
            d => println!("{}", d)
        }
    }
}