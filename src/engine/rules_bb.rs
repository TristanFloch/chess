pub fn bishop_attacks_bb(sq: usize, blockers: u64) -> u64 {
    let mut attacks = 0;

    let mut fill_until_blocker =
        |r_range: Box<dyn Iterator<Item = usize>>, f_range: Box<dyn Iterator<Item = usize>>| {
            for (r, f) in std::iter::zip(r_range, f_range) {
                let bb = 1u64 << (r * 8 + f);
                attacks |= bb;
                if blockers & bb != 0 {
                    break;
                }
            }
        };

    // cast to isize so substracting 1 causes no problem
    let tr = sq / 8; // target rank
    let tf = sq % 8; // target file

    // north east
    fill_until_blocker(Box::new((tr + 1)..8), Box::new((tf + 1)..8));
    // north west
    fill_until_blocker(Box::new((tr + 1)..8), Box::new((0..tf).rev()));
    // south west
    fill_until_blocker(Box::new((0..tr).rev()), Box::new((tf + 1)..8));
    // south east
    fill_until_blocker(Box::new((0..tr).rev()), Box::new((0..tf).rev()));

    attacks
}

    attacks
}
