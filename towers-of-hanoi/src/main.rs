use std::fmt::Debug;

fn main() {
    const NUM_RINGS: u32 = 33;
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        solve_game(NUM_RINGS)
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn solve_game(num_rings: u32) {
    let full_tower = get_starting_stack(num_rings);

    let mut game = TowerOfHanoi::<u32> {
        towers: vec![
            full_tower.clone(),
            Tower { rings: Vec::new() },
            Tower { rings: Vec::new() },
        ],
        starting_tower: 0,
        ending_tower: 2,
        num_moves: 0,
    };

    game.move_tower()
        .expect("Unable to solve this configuration.");

    assert_eq!(
        game,
        TowerOfHanoi::<u32> {
            towers: vec![
                Tower { rings: Vec::new() },
                Tower { rings: Vec::new() },
                full_tower.clone(),
            ],
            starting_tower: 0,
            ending_tower: 2,
            num_moves: 2_u64.pow(num_rings - 1) - 1,
        }
    );
}

fn get_starting_stack(num_rings: u32) -> Tower<u32> {
    Tower {
        rings: (1..num_rings).rev().collect::<Vec<u32>>(),
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Tower<T> {
    rings: Vec<T>,
}

impl<T: Ord> Tower<T> {
    fn push(&mut self, val: T) -> () {
        self.rings.push(val)
    }
    fn pop(&mut self) -> T {
        let val = self.rings.pop();
        val.expect("Tried to move rings from an empty peg.")
    }
}

#[derive(PartialEq, Clone, Debug)]
struct TowerOfHanoi<T> {
    towers: Vec<Tower<T>>,
    starting_tower: usize,
    ending_tower: usize,
    num_moves: u64,
}

impl<T: Ord + Debug> TowerOfHanoi<T> {
    fn move_top_peg(&mut self, staring_tower: usize, ending_tower: usize) -> () {
        let ring = self.towers[staring_tower].pop();
        self.towers[ending_tower].push(ring);
        self.num_moves += 1;
    }
    fn move_tower(&mut self) -> Result<String, String> {
        let auxiliary = self.find_auxiliary()?;

        self.solve(
            self.towers[self.starting_tower].rings.len(),
            self.starting_tower,
            self.ending_tower,
            auxiliary,
        );

        Ok("Solved.".into())
    }
    fn find_auxiliary(&self) -> Result<usize, String> {
        if self.towers.len() < 3 {
            return Err("This configuration is impossible to solve.".into());
        }
        for i in 1..self.towers.len() {
            if i != self.starting_tower && i != self.ending_tower {
                return Ok(i);
            }
        }
        Err("Auxiliary tower couldn't be found.".into())
    }

    fn solve(
        &mut self,
        n: usize,
        staring_tower: usize,
        ending_tower: usize,
        auxiliary_tower: usize,
    ) -> () {
        if n == 0 {
            return;
        }
        self.solve(n - 1, staring_tower, auxiliary_tower, ending_tower);
        // println!(
        //     "Move disk {:?} from rod {} to rod {}",
        //     self.towers[staring_tower].rings.last(),
        //     staring_tower,
        //     ending_tower
        // );
        self.move_top_peg(staring_tower, ending_tower);
        self.solve(n - 1, auxiliary_tower, ending_tower, staring_tower);
    }
}
