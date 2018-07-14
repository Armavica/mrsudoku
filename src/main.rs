use std::str::FromStr;
use std::io;
use std::io::BufRead;
use std::fmt;

/// Contains the state of a grid cell.  The digits are binary encoded.
#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Fixed(u16),
    Possible(u16),
}

impl Cell {
    /// Changes Cell::Possible into Cell::Fixed when possible and
    /// returns whether the cell is non-empty.
    fn check(&mut self) -> bool {
        *self = match self {
            Cell::Possible(d) if d.is_power_of_two() => Cell::Fixed(*d),
            _ => *self,
        };
        match self {
            Cell::Possible(0) => false,
            Cell::Fixed(_) | Cell::Possible(_) => true,
        }
    }
    /// Returns `true` iff the cell is fixed.
    fn is_fixed(&self) -> bool {
        match self {
            Cell::Fixed(_) => true,
            Cell::Possible(_) => false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Fixed(c) => write!(f, "{}", c.trailing_zeros()+1),
            Cell::Possible(_) => write!(f, "."),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Fixed(_) => write!(f, "    ({})    ", self),
            Cell::Possible(y) => {
                write!(f, "[")?;
                for i in 0..9 {
                    if (1 << i) & y != 0 {
                        write!(f, "{}", i+1)?;
                    } else {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Grid {
    grid: [[Cell; 9]; 9],
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[Cell::Possible((1<<9) - 1); 9]; 9];
        for (i, c) in s.chars().enumerate() {
            grid[i/9][i%9] = match c.to_digit(10) {
                Some(0) => panic!("why 0???"),
                Some(d) => Cell::Fixed(1 << (d - 1)),
                None if c == '.' => Cell::Possible((1<<9) - 1),
                None => panic!(format!("why {}???", c)),
            };
        }
        Ok(Grid { grid })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for (i, c) in row.iter().enumerate() {
                if i == 8 {
                    writeln!(f, "{}", c)?;
                } else {
                    write!(f, "{} ", c)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for c in row.iter() {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    /// Returns a transposed grid (lines swapped with columns).
    fn transpose(&self) -> Grid {
        Grid {
            grid:
                [[self.grid[0][0], self.grid[1][0], self.grid[2][0],
                  self.grid[3][0], self.grid[4][0], self.grid[5][0],
                  self.grid[6][0], self.grid[7][0], self.grid[8][0]],
                 [self.grid[0][1], self.grid[1][1], self.grid[2][1],
                  self.grid[3][1], self.grid[4][1], self.grid[5][1],
                  self.grid[6][1], self.grid[7][1], self.grid[8][1]],
                 [self.grid[0][2], self.grid[1][2], self.grid[2][2],
                  self.grid[3][2], self.grid[4][2], self.grid[5][2],
                  self.grid[6][2], self.grid[7][2], self.grid[8][2]],
                 [self.grid[0][3], self.grid[1][3], self.grid[2][3],
                  self.grid[3][3], self.grid[4][3], self.grid[5][3],
                  self.grid[6][3], self.grid[7][3], self.grid[8][3]],
                 [self.grid[0][4], self.grid[1][4], self.grid[2][4],
                  self.grid[3][4], self.grid[4][4], self.grid[5][4],
                  self.grid[6][4], self.grid[7][4], self.grid[8][4]],
                 [self.grid[0][5], self.grid[1][5], self.grid[2][5],
                  self.grid[3][5], self.grid[4][5], self.grid[5][5],
                  self.grid[6][5], self.grid[7][5], self.grid[8][5]],
                 [self.grid[0][6], self.grid[1][6], self.grid[2][6],
                  self.grid[3][6], self.grid[4][6], self.grid[5][6],
                  self.grid[6][6], self.grid[7][6], self.grid[8][6]],
                 [self.grid[0][7], self.grid[1][7], self.grid[2][7],
                  self.grid[3][7], self.grid[4][7], self.grid[5][7],
                  self.grid[6][7], self.grid[7][7], self.grid[8][7]],
                 [self.grid[0][8], self.grid[1][8], self.grid[2][8],
                  self.grid[3][8], self.grid[4][8], self.grid[5][8],
                  self.grid[6][8], self.grid[7][8], self.grid[8][8]]]
        }
    }

    /// Returns a grid with blocks swapped with lines.
    fn blocks(&self) -> Grid {
        Grid {
            grid:
                [[self.grid[0][0], self.grid[0][1], self.grid[0][2],
                  self.grid[1][0], self.grid[1][1], self.grid[1][2],
                  self.grid[2][0], self.grid[2][1], self.grid[2][2]],
                 [self.grid[0][3], self.grid[0][4], self.grid[0][5],
                  self.grid[1][3], self.grid[1][4], self.grid[1][5],
                  self.grid[2][3], self.grid[2][4], self.grid[2][5]],
                 [self.grid[0][6], self.grid[0][7], self.grid[0][8],
                  self.grid[1][6], self.grid[1][7], self.grid[1][8],
                  self.grid[2][6], self.grid[2][7], self.grid[2][8]],
                 [self.grid[3][0], self.grid[3][1], self.grid[3][2],
                  self.grid[4][0], self.grid[4][1], self.grid[4][2],
                  self.grid[5][0], self.grid[5][1], self.grid[5][2]],
                 [self.grid[3][3], self.grid[3][4], self.grid[3][5],
                  self.grid[4][3], self.grid[4][4], self.grid[4][5],
                  self.grid[5][3], self.grid[5][4], self.grid[5][5]],
                 [self.grid[3][6], self.grid[3][7], self.grid[3][8],
                  self.grid[4][6], self.grid[4][7], self.grid[4][8],
                  self.grid[5][6], self.grid[5][7], self.grid[5][8]],
                 [self.grid[6][0], self.grid[6][1], self.grid[6][2],
                  self.grid[7][0], self.grid[7][1], self.grid[7][2],
                  self.grid[8][0], self.grid[8][1], self.grid[8][2]],
                 [self.grid[6][3], self.grid[6][4], self.grid[6][5],
                  self.grid[7][3], self.grid[7][4], self.grid[7][5],
                  self.grid[8][3], self.grid[8][4], self.grid[8][5]],
                 [self.grid[6][6], self.grid[6][7], self.grid[6][8],
                  self.grid[7][6], self.grid[7][7], self.grid[7][8],
                  self.grid[8][6], self.grid[8][7], self.grid[8][8]]]
        }
    }

    /// Returns a list of exclusive groups of digits.
    /// 
    /// # Example
    /// 
    ///     [    5 7  ][123  67  ][    5 7  ] ...
    /// 
    /// The group of digits [57] appears only twice together in the line,
    /// they can then be pruned from the other cells.
    /// 
    ///     [    5 7  ][123  6   ][    5 7  ] ...
    /// 
    /// In this case, the result of this function will contain the group [57].
    #[inline(never)]
    fn exclusive_possibilities(row: &[Cell; 9]) -> [u16; 9] {
        let mut cell_groups = [0u16; 9];
        row.iter().enumerate().for_each(|(i, c)| {
            if let Cell::Possible(y) = c {
                for (j, x) in cell_groups.iter_mut().enumerate() {
                    if y & (1 << j) != 0 {
                        *x |= 1 << i;
                    }
                }
            }
        });
        let mut possibilities = [0; 9];
        possibilities.iter_mut().enumerate().for_each(|(i, pos)| {
            let mut digits: u16 = 0;
            let cells = cell_groups[i];
            for (j, x) in cell_groups.iter_mut().enumerate() {
                if cells == *x {
                    digits |= 1 << j;
                    *x = 0;
                }
            }
            if digits.count_ones() == cells.count_ones() {
                *pos = digits;
            }
        });
        possibilities
    }

    /// Prunes a row according to fixed numbers.  Returns the row if it
    /// still valid, and whether or not the row changed.
    #[inline(never)]
    fn prune_cells_by_fixed(mut row: [Cell; 9]) -> (bool, Option<[Cell; 9]>) {
        let mut changed = false;
        let fixeds = row.iter()
            .filter_map(|c| match c {
                Cell::Fixed(x) => Some(*x),
                Cell::Possible(_) => None
            })
            .fold(0, |acc, x| acc | x);
        for c in &mut row {
            if let Cell::Possible(x) = c {
                if *x & fixeds != 0 {
                    *x &= !fixeds;
                    changed = true;
                }
            }
            if !c.check() {
                return (changed, None)
            }
        }
        (changed, Some(row))
    }

    /// Prunes a row according to exclusive groups.  This operation
    /// cannot produce an invalid row.  It returns the pruned row, and
    /// whether or not the row changed.
    #[inline(never)]
    fn prune_cells_by_exclusives(mut row: [Cell; 9]) -> (bool, [Cell; 9]) {
        let mut changed = false;
        for exclusives in Grid::exclusive_possibilities(&row).iter().filter(|p| **p != 0) {
            for c in &mut row {
                if let Cell::Possible(x) = c {
                    if *x & exclusives != 0 && x != exclusives {
                        *x &= exclusives;
                        changed = true;
                    }
                }
                assert!(c.check());
            }
        }
        (changed, row)
    }

    /// Applies the two pruning operations to the grid, along rows.
    /// First fixed pruning until fixed point, then exclusive pruning
    /// until fixed point.
    #[inline(never)]
    fn prune_grid(mut self) -> Option<Grid> {
        for row in &mut self.grid {
            let mut new = (true, Some(*row));
            while new.0 && new.1.is_some() {
                new = Grid::prune_cells_by_fixed(new.1.unwrap());
            }
            if let (_, Some(new)) = new {
                let mut cn = Grid::prune_cells_by_exclusives(new);
                while cn.0 {
                    cn = Grid::prune_cells_by_exclusives(cn.1);
                }
                *row = cn.1;
            } else {
                return None
            }
        }
        Some(self)
    }

    /// Prunes the grid along rows, columns, and blocks, until fixed point.
    #[inline(never)]
    fn prune_grid_all(self) -> Option<Grid> {
        let mut old = None;
        let mut new = Some(self);
        while old != new {
            old = new.clone();
            new = new
                .and_then(Grid::prune_grid)
                .and_then(|g| g.transpose().prune_grid().map(|g| g.transpose()))
                .and_then(|g| g.blocks().prune_grid().map(|g| g.blocks()));
        }
        new
    }

    /// Divides the search space into two complementary subspaces.
    #[inline(never)]
    fn next_grids(mut self) -> (Grid, Grid) {
        let mut other = self.clone();
        'out: for (row, row1) in self.grid.iter_mut().zip(other.grid.iter_mut()) {
            for (c, c1) in row.iter_mut().zip(row1.iter_mut()) {
                let mut out = false;
                if let Cell::Possible(x) = c {
                    let y = 1 << x.trailing_zeros();
                    *c1 = Cell::Fixed(y);
                    out = true;
                    *x ^= y;
                };
                assert!(c.check());
                if out {
                    break 'out;
                }
            }
        }
        (self, other)
    }

    /// Returns `true` iff the grid is completely filled.
    #[inline(never)]
    fn is_filled(&self) -> bool {
        self.grid.iter()
            .all(|row| row.iter()
                 .all(Cell::is_fixed))
    }

    /// Returns `true` iff the grid has an invalid row.
    #[inline(never)]
    fn is_invalid_row(&self) -> bool {
        for row in &self.grid {
            let mut fixeds = 0;
            let mut n = 0;
            for c in row.iter() {
                if let Cell::Fixed(x) = c {
                    fixeds |= *x;
                    n += 1;
                }
            }
            if n != fixeds.count_ones() {
                return true;
            }
        }
        false
    }

    /// Returns `true` iff the grid is invalid.
    #[inline(never)]
    fn is_invalid(&self) -> bool {
        self.is_invalid_row()
            || self.transpose().is_invalid_row()
            || self.blocks().is_invalid_row()
    }

    /// Returns the solved grid if it exists.
    #[inline(never)]
    fn solve(self) -> Option<Grid> {
        self.prune_grid_all().and_then(|grid| {
            if grid.is_invalid() {
                None
            } else if grid.is_filled() {
                Some(grid)
            } else {
                let (grid1, grid2) = grid.next_grids();
                grid1.solve().or_else(|| grid2.solve())
            }
        })
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let grid = Grid::from_str(&line.unwrap()).ok().and_then(Grid::solve).unwrap();
        println!("{}", grid);
    }
}
