use std::collections::HashSet;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Life {
    cells: HashSet<(i32, i32)>,
}

impl Life {
    pub fn tick(&mut self) {
        let cells_to_check = self
            .cells
            .iter()
            .flat_map(|pt| impacts(*pt))
            .collect::<HashSet<_>>();

        let previous = std::mem::take(&mut self.cells);
        for pt in cells_to_check {
            let neighbors = neighbors(pt)
                .into_iter()
                .filter(|p| previous.contains(p))
                .count();
            match (previous.contains(&pt), neighbors) {
                (true, 2..4) => _ = self.cells.insert(pt),
                (false, 3) => _ = self.cells.insert(pt),
                _ => (),
            };
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn insert(&mut self, pt: (i32, i32)) -> bool {
        self.cells.insert(pt)
    }

    pub fn contains(&self, pt: &(i32, i32)) -> bool {
        self.cells.contains(pt)
    }
}

impl FromIterator<(i32, i32)> for Life {
    fn from_iter<T: IntoIterator<Item = (i32, i32)>>(iter: T) -> Self {
        Life {
            cells: HashSet::from_iter(iter),
        }
    }
}

impl Extend<(i32, i32)> for Life {
    fn extend<T: IntoIterator<Item = (i32, i32)>>(&mut self, iter: T) {
        self.cells.extend(iter)
    }
}

// cells to check
fn impacts(pt: (i32, i32)) -> Vec<(i32, i32)> {
    let mut v = neighbors(pt);
    v.push(pt);
    v
}
fn neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    let x_plus = x.checked_add(1);
    let x_minus = x.checked_sub(1);
    let y_plus = y.checked_add(1);
    let y_minus = y.checked_sub(1);
    if let Some(x) = x_plus {
        v.push((x, y));
    }
    if let Some(y) = y_plus {
        v.push((x, y));
    }
    if let Some(pt) = x_plus.zip(y_plus) {
        v.push(pt);
    }
    if let Some(pt) = x_plus.zip(y_minus) {
        v.push(pt);
    }
    if let Some(x) = x_minus {
        v.push((x, y));
    }
    if let Some(y) = y_minus {
        v.push((x, y));
    }
    if let Some(pt) = x_minus.zip(y_plus) {
        v.push(pt);
    }
    if let Some(pt) = x_minus.zip(y_minus) {
        v.push(pt);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impacts() {
        assert_eq!(
            impacts((0, 0)),
            &[
                (1, 0),
                (0, 1),
                (1, 1),
                (1, -1),
                (-1, 0),
                (0, -1),
                (-1, 1),
                (-1, -1),
                (0, 0),
            ]
        )
    }

    #[test]
    fn test_block() {
        let mut game = Life::default();
        game.insert((1, 1));
        game.insert((2, 1));
        game.insert((2, 2));
        game.insert((1, 2));
        let start = game.clone();
        game.tick();
        assert_eq!(start, game);
        game.tick();
        assert_eq!(start, game);
    }

    #[test]
    fn test_blinker() {
        let mut game = Life::default();
        game.insert((1, 1));
        game.insert((2, 1));
        game.insert((3, 1));
        let start = game.clone();
        game.tick();
        game.tick();
        assert_eq!(start, game);
    }

    #[test]
    fn test_diehard() {
        let mut game = Life::default();
        game.insert((1, 0));
        game.insert((1, 1));
        game.insert((0, 1));
        game.insert((5, 0));
        game.insert((6, 0));
        game.insert((7, 0));
        game.insert((6, 2));

        for _ in 0..130 {
            assert!(!game.is_empty());
            game.tick();
        }
        assert_eq!(game.len(), 0);
    }
}
