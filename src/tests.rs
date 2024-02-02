#!(allow(dead_code, unused))

#[cfg(test)]
mod tests {
    use crate::{E, X, O, BD0, BD1, BD2, BD3, BD4, next_boards, solved, minimax};
    use super::*;

    #[test]
    fn test_next_boards() {
        let result0 = next_boards(BD0, X);
        let result1 = next_boards(BD1, O);
        let result2 = next_boards(BD2, O);

        assert_eq!(result0.len(), 9);
        assert_eq!(result1.len(), 0);
        assert_eq!(result2.len(), 2);
    }

    #[test]
    fn test_solved() {
        let result0 = solved(BD0, X);
        let result1 = solved(BD1, X);
        let result2 = solved(BD3, O);
        let result3 = solved(BD4, X);
        let result4 = solved(BD4, O);
        let result5 = solved(BD3, X);

        assert_eq!(result0, false);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
        assert_eq!(result4, false);
        assert_eq!(result5, false);
    }

    #[test]
    fn test_minimax() {
        let result0 = minimax([
            E, E, X,
            E, X, E,
            X, E, E,
        ], X);
        let result1 = minimax([
            E, O, X,
            E, O, E,
            X, O, E,
        ], X);
        assert_eq!(result1, -1.0);
        let result2 = minimax([
            O, X, O,
            X, X, O,
            O, O, X,
        ], X);
        assert_eq!(result2, 0.0);
        let result3 = minimax([
            O, X, X,
            X, X, O,
            O, E, O,
        ], X);
        assert_eq!(result3, 1.0);
    }
}
