const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

fn south_one(bb: u64) -> u64 {
    bb >> 8
}

fn north_one(bb: u64) -> u64 {
    bb << 8
}

fn east_one(bb: u64) -> u64 {
    (bb << 1) & NOT_A_FILE
}

fn west_one(bb: u64) -> u64 {
    (bb >> 1) & NOT_H_FILE
}

pub fn exclude_friends(attacks: u64, friends: u64) -> u64 {
    attacks ^ (attacks & friends)
}

pub fn king_attacks_bb(sq: usize) -> u64 {
    let mut king = 1u64 << sq;
    let mut attacks = east_one(king) | west_one(king);
    king |= attacks;
    attacks |= north_one(king) | south_one(king);

    attacks
}

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
