use std::convert::TryInto;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};

pub trait ParseGridChar<T: Copy + Debug> {
    fn parse_grid_char(c: char) -> T;
}

pub struct Grid<T: Copy + Debug> {
    pub data: Vec<Vec<T>>,
}

impl<T: Copy + Debug> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn construct(width: usize, height: usize, value: T) -> Self {
        let data = vec![vec![value; width]; height];
        Self::new(data)
    }

    pub fn _get<S: TryInto<usize>>(&self, x: S, y: S) -> Option<T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        self.data.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn get<S: TryInto<usize>>(&self, x: S, y: S) -> T {
        self._get(x, y).expect("grid contains coords")
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn parse<P: ParseGridChar<T>>(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().map(P::parse_grid_char).collect())
            .collect();
        Self::new(data)
    }

    pub fn neighbour_coords_cardinal(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < self.width() - 1 {
            neighbours.push((x + 1, y));
        }
        if y < self.height() - 1 {
            neighbours.push((x, y + 1));
        }
        neighbours
    }

    pub fn neighbours_cardinal(&self, x: usize, y: usize) -> Vec<T> {
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push(self.data[y][x - 1]);
        }
        if y > 0 {
            neighbours.push(self.data[y - 1][x]);
        }
        if x < self.width() - 1 {
            neighbours.push(self.data[y][x + 1]);
        }
        if y < self.height() - 1 {
            neighbours.push(self.data[y + 1][x]);
        }
        neighbours
    }

    pub fn iter_coords_col_major(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width()).flat_map(move |x| (0..self.height()).map(move |y| (x, y)))
    }

    pub fn iter_coords_row_major(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }
}

impl<T: Copy + Debug + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Copy + Debug> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self::new(self.data.clone())
    }
}

impl<T: Copy + Debug + PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Copy + Debug + PartialEq> Eq for Grid<T> {}
