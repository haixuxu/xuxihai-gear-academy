use super::*;

#[test]
fn program_gen_count() {
    for _ in 0..1000 {
        let count = program_turn_gen(DifficultyLevel::Easy, 100, 5);
        assert!(count <= 5);
        let count2 = program_turn_gen(DifficultyLevel::Easy, 5, 10);
        assert!(count2 == 5);
        let count3 = program_turn_gen(DifficultyLevel::Hard, 100, 5);
        assert_eq!(count3, 4); // 96=5n+1
        let count4 = program_turn_gen(DifficultyLevel::Hard, 5, 10);
        assert_eq!(count4, 5);

        let count4 = program_turn_gen(DifficultyLevel::Hard, 5, 1);
        assert_eq!(count4, 1);

        let count4 = program_turn_gen(DifficultyLevel::Easy, 5, 1);
        assert_eq!(count4, 1);

        let count4 = program_turn_gen(DifficultyLevel::Easy, 0, 3);
        assert_eq!(count4, 0);
    }
}
