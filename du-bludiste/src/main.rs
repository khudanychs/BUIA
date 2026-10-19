use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env;
use std::time::Instant;

// ============================================================================
// 1. DATOVÉ STRUKTURY PRO A* (Podpora až do 10^18 bez přetečení)
// ============================================================================

/// Uzel v prioritní frontě pro A* s 64-bitovými souřadnicemi.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AStarNode {
    pub f_score: u64,
    pub h_score: u64, // pro tie-breaking: preferujeme uzel blíže k cíli
    pub pos: (u64, u64),
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // MIN-HEAP: menší f_score má přednost
        match other.f_score.cmp(&self.f_score) {
            Ordering::Equal => other.h_score.cmp(&self.h_score), // tie-breaking
            ord => ord,
        }
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// 2. ULTRA-ÚSPORNÝ EXPLICITNÍ BITSET (Bonus: n = 100 až n = 10 000 v RAM)
// ============================================================================

const DIR_UNVISITED: u8 = 0;
const DIR_START: u8 = 1;
const DIR_FROM_UP: u8 = 2;
const DIR_FROM_DOWN: u8 = 3;
const DIR_FROM_LEFT: u8 = 4;
const DIR_FROM_RIGHT: u8 = 5;

pub struct CompactGrid {
    pub size: usize,
    bitset: Vec<u64>,
}

impl CompactGrid {
    pub fn new_random(size: usize, obstacle_ratio: f64) -> Self {
        let total_cells = size * size;
        let num_words = (total_cells + 63) / 64;
        let mut bitset = vec![0u64; num_words];

        let mut rng = thread_rng();
        let target_obstacles = (total_cells as f64 * obstacle_ratio).ceil() as usize;
        let mut placed_obstacles = 0;

        for idx in 0..total_cells {
            if idx == 0 || idx == total_cells - 1 {
                continue;
            }
            if rng.gen_bool(obstacle_ratio) {
                let word = idx / 64;
                let bit = idx % 64;
                bitset[word] |= 1u64 << bit;
                placed_obstacles += 1;
            }
        }

        if placed_obstacles < target_obstacles {
            for idx in 1..(total_cells - 1) {
                if placed_obstacles >= target_obstacles {
                    break;
                }
                let word = idx / 64;
                let bit = idx % 64;
                if (bitset[word] & (1u64 << bit)) == 0 {
                    bitset[word] |= 1u64 << bit;
                    placed_obstacles += 1;
                }
            }
        }

        Self { size, bitset }
    }

    #[inline(always)]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    #[inline(always)]
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        let idx = self.idx(x, y);
        let word = idx / 64;
        let bit = idx % 64;
        (self.bitset[word] & (1u64 << bit)) != 0
    }

    #[inline(always)]
    fn heuristic(&self, from: (usize, usize), to: (usize, usize)) -> u64 {
        let dx = (from.0 as isize - to.0 as isize).unsigned_abs() as u64;
        let dy = (from.1 as isize - to.1 as isize).unsigned_abs() as u64;
        dx + dy
    }

    /// Ultra-úsporné BFS s minimální paměťovou stopou (~1.125 bajtu na buňku).
    pub fn find_path_bfs(&self) -> (Option<Vec<(usize, usize)>>, usize) {
        let total = self.size * self.size;
        let goal = (self.size - 1, self.size - 1);
        let start = (0, 0);

        let mut cell_state = vec![DIR_UNVISITED; total];
        let mut queue = VecDeque::new();

        let start_idx = self.idx(start.0, start.1);
        cell_state[start_idx] = DIR_START;
        queue.push_back(start_idx);

        let mut found = false;
        let mut nodes_visited = 0;
        let size = self.size;

        while let Some(curr_idx) = queue.pop_front() {
            nodes_visited += 1;
            let cx = curr_idx % size;
            let cy = curr_idx / size;

            if (cx, cy) == goal {
                found = true;
                break;
            }

            if cx > 0 && !self.is_wall(cx - 1, cy) {
                let next_idx = curr_idx - 1;
                if cell_state[next_idx] == DIR_UNVISITED {
                    cell_state[next_idx] = DIR_FROM_RIGHT;
                    queue.push_back(next_idx);
                }
            }
            if cx + 1 < size && !self.is_wall(cx + 1, cy) {
                let next_idx = curr_idx + 1;
                if cell_state[next_idx] == DIR_UNVISITED {
                    cell_state[next_idx] = DIR_FROM_LEFT;
                    queue.push_back(next_idx);
                }
            }
            if cy > 0 && !self.is_wall(cx, cy - 1) {
                let next_idx = curr_idx - size;
                if cell_state[next_idx] == DIR_UNVISITED {
                    cell_state[next_idx] = DIR_FROM_DOWN;
                    queue.push_back(next_idx);
                }
            }
            if cy + 1 < size && !self.is_wall(cx, cy + 1) {
                let next_idx = curr_idx + size;
                if cell_state[next_idx] == DIR_UNVISITED {
                    cell_state[next_idx] = DIR_FROM_UP;
                    queue.push_back(next_idx);
                }
            }
        }

        if !found {
            return (None, nodes_visited);
        }

        let mut path = Vec::new();
        let mut cx = goal.0;
        let mut cy = goal.1;
        path.push((cx, cy));

        loop {
            let idx = self.idx(cx, cy);
            match cell_state[idx] {
                DIR_FROM_UP => cy -= 1,
                DIR_FROM_DOWN => cy += 1,
                DIR_FROM_LEFT => cx -= 1,
                DIR_FROM_RIGHT => cx += 1,
                DIR_START => break,
                _ => unreachable!(),
            }
            path.push((cx, cy));
        }

        path.reverse();
        (Some(path), nodes_visited)
    }

    /// Implementace A* pro srovnání
    pub fn find_path_astar(&self) -> (Option<Vec<(usize, usize)>>, usize) {
        let total = self.size * self.size;
        let start = (0, 0);
        let goal = (self.size - 1, self.size - 1);

        let mut g_score = vec![u64::MAX; total];
        let mut cell_state = vec![DIR_UNVISITED; total];
        let mut open_set = BinaryHeap::new();

        let start_idx = self.idx(start.0, start.1);
        g_score[start_idx] = 0;
        cell_state[start_idx] = DIR_START;

        let h_start = self.heuristic(start, goal);
        open_set.push(AStarNode {
            f_score: h_start,
            h_score: h_start,
            pos: (start.0 as u64, start.1 as u64),
        });

        let mut found = false;
        let mut nodes_visited = 0;
        let size = self.size;

        while let Some(AStarNode { pos: (cx_u, cy_u), .. }) = open_set.pop() {
            let (cx, cy) = (cx_u as usize, cy_u as usize);
            nodes_visited += 1;

            if (cx, cy) == goal {
                found = true;
                break;
            }

            let curr_idx = self.idx(cx, cy);
            let current_g = g_score[curr_idx];

            let neighbors = [
                (cx.wrapping_sub(1), cy, cx > 0, DIR_FROM_RIGHT),
                (cx + 1, cy, cx + 1 < size, DIR_FROM_LEFT),
                (cx, cy.wrapping_sub(1), cy > 0, DIR_FROM_DOWN),
                (cx, cy + 1, cy + 1 < size, DIR_FROM_UP),
            ];

            for (nx, ny, valid, dir) in neighbors {
                if !valid || self.is_wall(nx, ny) {
                    continue;
                }

                let n_idx = self.idx(nx, ny);
                let tentative_g = current_g + 1;

                if tentative_g < g_score[n_idx] {
                    g_score[n_idx] = tentative_g;
                    cell_state[n_idx] = dir;
                    let h = self.heuristic((nx, ny), goal);
                    let f = tentative_g + h;
                    open_set.push(AStarNode {
                        f_score: f,
                        h_score: h,
                        pos: (nx as u64, ny as u64),
                    });
                }
            }
        }

        if !found {
            return (None, nodes_visited);
        }

        let mut path = Vec::new();
        let mut cx = goal.0;
        let mut cy = goal.1;
        path.push((cx, cy));

        loop {
            let idx = self.idx(cx, cy);
            match cell_state[idx] {
                DIR_FROM_UP => cy -= 1,
                DIR_FROM_DOWN => cy += 1,
                DIR_FROM_LEFT => cx -= 1,
                DIR_FROM_RIGHT => cx += 1,
                DIR_START => break,
                _ => unreachable!(),
            }
            path.push((cx, cy));
        }

        path.reverse();
        (Some(path), nodes_visited)
    }
}

// ============================================================================
// 3. PROCEDURÁLNÍ KOSMICKÁ MŘÍŽKA (n = 10^2 až 10^18, 0 bajtů RAM pro zdi)
// ============================================================================

pub struct CosmicGrid {
    pub size: u64,
    pub obstacle_ratio: f64,
    pub seed: u64,
}

impl CosmicGrid {
    pub fn new(size: u64, obstacle_ratio: f64, seed: u64) -> Self {
        Self { size, obstacle_ratio, seed }
    }

    /// Rychlý SplitMix64 hash souřadnic (2 ns, žádná RAM).
    #[inline(always)]
    pub fn is_wall(&self, x: u64, y: u64) -> bool {
        if (x == 0 && y == 0) || (x == self.size - 1 && y == self.size - 1) {
            return false;
        }
        let mut z = x.wrapping_mul(0x9E3779B97F4A7C15)
            ^ y.wrapping_mul(0xBF58476D1CE4E5B9)
            ^ self.seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        let h = z ^ (z >> 31);
        (h % 10_000) < (self.obstacle_ratio * 10_000.0) as u64
    }

    #[inline(always)]
    fn heuristic(&self, from: (u64, u64), to: (u64, u64)) -> u64 {
        let dx = if from.0 > to.0 { from.0 - to.0 } else { to.0 - from.0 };
        let dy = if from.1 > to.1 { from.1 - to.1 } else { to.1 - from.1 };
        dx + dy
    }

    /// Sparse A* prohledávání s 64-bitovými souřadnicemi a tie-breakingem.
    pub fn find_path_sparse_astar(&self, max_nodes: usize) -> (Option<u64>, usize, bool, u64) {
        let goal = (self.size - 1, self.size - 1);
        let start = (0u64, 0u64);

        let mut g_score = HashMap::new();
        let mut closed_set = HashSet::new();
        let mut open_set = BinaryHeap::new();

        g_score.insert(start, 0u64);
        let h_start = self.heuristic(start, goal);
        open_set.push(AStarNode {
            f_score: h_start,
            h_score: h_start,
            pos: start,
        });

        let mut nodes_visited = 0;
        let mut found_dist = None;
        let mut hit_limit = false;
        let mut min_h = h_start;
        let size = self.size;

        while let Some(AStarNode { pos: (cx, cy), .. }) = open_set.pop() {
            if !closed_set.insert((cx, cy)) {
                continue;
            }

            nodes_visited += 1;
            let current_h = self.heuristic((cx, cy), goal);
            if current_h < min_h {
                min_h = current_h;
            }

            if (cx, cy) == goal {
                found_dist = Some(g_score[&(cx, cy)]);
                break;
            }

            if nodes_visited >= max_nodes {
                hit_limit = true;
                break;
            }

            let current_g = g_score[&(cx, cy)];

            let neighbors = [
                (cx + 1, cy, cx + 1 < size),
                (cx, cy + 1, cy + 1 < size),
                (cx.wrapping_sub(1), cy, cx > 0),
                (cx, cy.wrapping_sub(1), cy > 0),
            ];

            for (nx, ny, valid) in neighbors {
                if valid && !closed_set.contains(&(nx, ny)) && !self.is_wall(nx, ny) {
                    let tentative_g = current_g + 1;
                    let is_better = match g_score.get(&(nx, ny)) {
                        Some(&old_g) => tentative_g < old_g,
                        None => true,
                    };
                    if is_better {
                        g_score.insert((nx, ny), tentative_g);
                        let h = self.heuristic((nx, ny), goal);
                        let f = tentative_g + h;
                        open_set.push(AStarNode {
                            f_score: f,
                            h_score: h,
                            pos: (nx, ny),
                        });
                    }
                }
            }
        }

        (found_dist, nodes_visited, hit_limit, min_h)
    }

    /// Skutečně obousměrný důkaz perkolace:
    /// Střídá kroky ze startu a z cíle. Jakmile se KTERÁKOLIV strana uzavře, okamžitě končí s důkazem.
    pub fn prove_nonexistence_bidirectional(&self, max_search: usize) -> Result<String, String> {
        let start = (0u64, 0u64);
        let goal = (self.size - 1, self.size - 1);

        let mut visited_start = HashSet::new();
        let mut q_start = VecDeque::new();
        visited_start.insert(start);
        q_start.push_back(start);

        let mut visited_goal = HashSet::new();
        let mut q_goal = VecDeque::new();
        visited_goal.insert(goal);
        q_goal.push_back(goal);

        while !q_start.is_empty() || !q_goal.is_empty() {
            // 1. Krok ze Startu
            if let Some((cx, cy)) = q_start.pop_front() {
                if visited_start.len() < max_search {
                    let neighbors = [
                        (cx.wrapping_sub(1), cy, cx > 0),
                        (cx + 1, cy, cx + 1 < self.size),
                        (cx, cy.wrapping_sub(1), cy > 0),
                        (cx, cy + 1, cy + 1 < self.size),
                    ];
                    for (nx, ny, valid) in neighbors {
                        if valid && !self.is_wall(nx, ny) && visited_start.insert((nx, ny)) {
                            q_start.push_back((nx, ny));
                        }
                    }
                }
            } else {
                return Ok(format!(
                    "DOKÁZÁNO: Cesta NEEXISTUJE! Komponenta STARTU je uzavřena překážkami (velikost uzavřené komponenty: {} buněk).",
                    visited_start.len()
                ));
            }

            // 2. Krok z Cíle
            if let Some((cx, cy)) = q_goal.pop_front() {
                if visited_goal.len() < max_search {
                    let neighbors = [
                        (cx.wrapping_sub(1), cy, cx > 0),
                        (cx + 1, cy, cx + 1 < self.size),
                        (cx, cy.wrapping_sub(1), cy > 0),
                        (cx, cy + 1, cy + 1 < self.size),
                    ];
                    for (nx, ny, valid) in neighbors {
                        if valid && !self.is_wall(nx, ny) && visited_goal.insert((nx, ny)) {
                            q_goal.push_back((nx, ny));
                        }
                    }
                }
            } else {
                return Ok(format!(
                    "DOKÁZÁNO: Cesta NEEXISTUJE! Komponenta CÍLE je uzavřena překážkami (velikost uzavřené komponenty: {} buněk).",
                    visited_goal.len()
                ));
            }

            if visited_start.len() >= max_search && visited_goal.len() >= max_search {
                break;
            }
        }

        Err(format!(
            "Otevřený koridor: start prozkoumal {} uzlů, cíl prozkoumal {} uzlů (přesažen limit {})",
            visited_start.len(), visited_goal.len(), max_search
        ))
    }
}

// ============================================================================
// 4. POMOCNÉ FORMÁTOVÁNÍ VELKÝCH ČÍSEL
// ============================================================================

/// Naformátuje číslo s mezerami po tisících (např. 1 000 000 000)
fn format_num(mut n: u128) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut s = String::new();
    let mut count = 0;
    while n > 0 {
        if count > 0 && count % 3 == 0 {
            s.push(' ');
        }
        s.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
        count += 1;
    }
    s.chars().rev().collect()
}

// ============================================================================
// 5. HLAVNÍ PROGRAM S PODPOROU MOCNIN
// ============================================================================

fn main() {
    println!("======================================================================");
    println!(" PROJEKT: ŠKÁLOVATELNÉ PROHLEDÁVÁNÍ BLUDIŠTĚ V RUSTU                  ");
    println!(" Podpora mocnin 10^k od n = 10^2 až po n = 10^15                     ");
    println!("======================================================================\n");

    let args: Vec<String> = env::args().collect();

    // Pokud uživatel zadá z příkazové řádky např. `cargo run --release -- 15 0.4`:
    if args.len() >= 2 {
        let k: u32 = args[1].parse().unwrap_or_else(|_| {
            eprintln!("Chyba: argument musí být mocnina (např. 15 pro 10^15)");
            std::process::exit(1);
        });

        let obstacle_ratio: f64 = if args.len() >= 3 {
            args[2].parse().unwrap_or(0.40)
        } else {
            0.40
        };

        run_single_power_test(k, obstacle_ratio);
        return;
    }

    // Standardní běh bez argumentů: otestuje kompletní škálu mocnin
    println!("--- ČÁST 1: Porovnání BFS vs A* na řešitelné mřížce (50x50, 40 % překážek) ---");
    let mut grid_comp;
    let mut attempts = 0;
    loop {
        attempts += 1;
        grid_comp = CompactGrid::new_random(50, 0.40);
        if let (Some(_), _) = grid_comp.find_path_bfs() {
            break;
        }
    }
    println!("Nalezena řešitelná instance po {} pokusech.", attempts);

    let t0 = Instant::now();
    let (path_bfs, nodes_bfs) = grid_comp.find_path_bfs();
    let dur_bfs = t0.elapsed();

    let t1 = Instant::now();
    let (path_astar, nodes_astar) = grid_comp.find_path_astar();
    let dur_astar = t1.elapsed();

    let len_bfs = path_bfs.as_ref().unwrap().len();
    let len_astar = path_astar.as_ref().unwrap().len();

    println!("METRIKA             | BFS              | A*");
    println!("--------------------+------------------+-----------------");
    println!("Délka cesty         | {:<16} | {:<16}", len_bfs, len_astar);
    println!("Prozkoumáno uzlů    | {:<16} | {:<16}", nodes_bfs, nodes_astar);
    println!("Čas výpočtu         | {:<16?} | {:<16?}", dur_bfs, dur_astar);
    println!("Optimalita ověřena  | {}", if len_bfs == len_astar { "✓ SHODNÁ (optimální)" } else { "✗ CHYBA" });
    println!();

    println!("--- ČÁST 2: Škálování přes mocniny desítky (n = 10^k, 40 % překážek) ---");
    println!("Využívá CosmicGrid s procedurálním hashem souřadnic (0.00 MB RAM pro zdi).\n");

    let powers = [2, 3, 4, 5, 6, 9, 12, 15];

    for &k in &powers {
        let n: u64 = 10u64.pow(k);
        let total_cells = (n as u128) * (n as u128);

        println!("--------------------------------------------------");
        println!("Test pro n = 10^{} (strana: {} buněk)", k, format_num(n as u128));
        println!("Celková plocha mřížky: 10^{} ({} buněk)", k * 2, format_num(total_cells));
        println!("Spotřeba RAM pro mřížku: 0.00 MB (generováno za běhu)");

        let grid = CosmicGrid::new(n, 0.40, 42 + k as u64);
        let t_start = Instant::now();
        let res = grid.prove_nonexistence_bidirectional(100_000);
        let duration = t_start.elapsed();

        match res {
            Ok(msg) => println!("✓ {}", msg),
            Err(e) => println!("! {}", e),
        }
        println!("Čas výpočtu: {:?}", duration);
    }

    println!("\n--- ČÁST 3: Škálování pro 10 % překážek (kde cesta existuje) ---");
    println!("Test A* s tie-breakingem pro n = 10^4 (10 000 buněk na stranu, 100 milionů polí):");
    let n_solvable = 10_000u64;
    let grid_solvable = CosmicGrid::new(n_solvable, 0.10, 12345);
    let t_a = Instant::now();
    let (res_path, visited, hit_lim, _) = grid_solvable.find_path_sparse_astar(5_000_000);
    let dur_a = t_a.elapsed();

    match res_path {
        Some(dist) => println!("✓ Nalezena nejkratší cesta! Délka: {} kroků (prozkoumáno {} uzlů)", dist, visited),
        None => {
            if hit_lim {
                println!("! Dosažen limit uzlů ({})", visited);
            } else {
                println!("✗ Cesta neexistuje.");
            }
        }
    }
    println!("Čas výpočtu cesty pro 100 000 000 buněk: {:?}", dur_a);

    println!("\n======================================================================");
    println!(" Tip: Můžeš spustit libovolnou mocninu z příkazové řádky:");
    println!("      cargo run --release -- <mocnina_k> [pomer_prekazek]");
    println!("      Příklad pro n = 10^15 s 40 %: cargo run --release -- 15 0.40");
    println!("      Příklad pro n = 10^4  s 10 %: cargo run --release -- 4  0.10");
    println!("======================================================================");
}

fn run_single_power_test(k: u32, obstacle_ratio: f64) {
    if k > 18 {
        eprintln!("Chyba: maximální mocnina pro 64-bit je k = 18 (10^18).");
        return;
    }
    let n: u64 = 10u64.pow(k);
    let total_cells = (n as u128) * (n as u128);

    println!("======================================================================");
    println!(" TEST PRO MOCNINU: n = 10^{} (Poměr překážek: {:.0} %)", k, obstacle_ratio * 100.0);
    println!("======================================================================");
    println!("Rozměr mřížky:        {} x {}", format_num(n as u128), format_num(n as u128));
    println!("Celkový počet buněk:  10^{} ({})", k * 2, format_num(total_cells));
    println!("Spotřeba RAM mřížky:  0.00 MB (procedurální hash SplitMix64)");
    println!("Start:                (0, 0)");
    println!("Cíl:                  ({}, {})", format_num((n - 1) as u128), format_num((n - 1) as u128));
    let seed: u64 = thread_rng().gen_range(1..=u64::MAX);
    println!("Náhodný seed:         {}", seed);
    println!("----------------------------------------------------------------------");

    let grid = CosmicGrid::new(n, obstacle_ratio, seed);

    if obstacle_ratio >= 0.35 {
        // Pro vyšší hustotu překážek provádíme obousměrný důkaz uzavřené komponenty
        let t_start = Instant::now();
        let res = grid.prove_nonexistence_bidirectional(100_000);
        let duration = t_start.elapsed();

        match res {
            Ok(msg) => println!("Výsledek: {}", msg),
            Err(e) => println!("Výsledek: {}", e),
        }
        println!("Doba výpočtu: {:?}", duration);
    } else {
        // Pro nízkou hustotu (např. 0.10) hledáme cestu přes A*
        let t_start = Instant::now();
        let (res_path, visited, hit_lim, min_h) = grid.find_path_sparse_astar(5_000_000);
        let duration = t_start.elapsed();

        match res_path {
            Some(dist) => println!("Výsledek: Nalezena nejkratší cesta! Délka: {} kroků (prozkoumáno {} uzlů)", dist, visited),
            None => {
                if hit_lim {
                    println!("Výsledek: Dosažen limit prozkoumaných uzlů ({}), zbývalo {} kroků k cíli", visited, min_h);
                } else {
                    println!("Výsledek: DOKÁZÁNO: Cesta NEEXISTUJE (komponenta vyčerpána: {} uzlů)", visited);
                }
            }
        }
        println!("Doba výpočtu: {:?}", duration);
    }
}
