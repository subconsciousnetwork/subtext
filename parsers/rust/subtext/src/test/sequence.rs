mod char {
    use crate::sequence::Sequence;

    #[test]
    fn can_be_completed() {
        let mut sequence = Sequence::new(&['H', 'e', 'l', 'l', 'o'], None);

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );

        "Hello".chars().for_each(|state| {
            sequence.go_to(&state);
        });

        assert!(sequence.is_complete(), "Sequence should be complete")
    }

    #[test]
    fn cannot_complete_without_reset() {
        let mut sequence = Sequence::new(&['H', 'e', 'l', 'l', 'o'], Some(' '));

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );

        "HelixHello".chars().for_each(|state| {
            sequence.go_to(&state);
        });

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );
    }

    #[test]
    fn can_be_completed_after_reset() {
        // let mut sequence = Sequence::new("Hello".chars().collect());
        let mut sequence = Sequence::new(&['H', 'e', 'l', 'l', 'o'], Some(' '));

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );

        "Helix".chars().for_each(|state| {
            sequence.go_to(&state);
        });

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );

        sequence.go_to(&' ');

        "Hello".chars().for_each(|state| {
            sequence.go_to(&state);
        });

        assert!(sequence.is_complete(), "Sequence should be complete");
    }

    #[test]
    fn can_detect_sequence_without_a_reset() {
        let mut sequence = Sequence::new(&['#', '#'], None);

        assert!(
            sequence.is_complete() == false,
            "Sequence should not be complete"
        );

        "Hello##".chars().for_each(|state| {
            sequence.go_to(&state);
        });

        assert!(sequence.is_complete(), "Sequence should be complete");
    }
}
