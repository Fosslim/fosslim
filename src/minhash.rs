const DEFAULT_PRIME: u32 = 4294967311;

type HashCoefficient = (u32, u32);

const DEFAULT_COEFFICIENTS: [HashCoefficient; 10] = [
    (1491601136u32, 3582045169u32),
    (4074286997, 3261015846),
    (2290128410, 3874633498),
    (518531567, 3364004087),
    (2180254080, 2685825028),
    (3787194716, 3524620623),
    (1618943887, 1968091459),
    (405469774, 4075310512),
    (2744940098, 969019993),
    (2442394881, 2058362891),
];

#[derive(Clone, Debug)]
pub struct MinHashModel {
    hash_coefficients: Vec<HashCoefficient>,
    prime: u32,
}

impl MinHashModel {
    pub fn default() -> MinHashModel {
        MinHashModel {
            hash_coefficients: DEFAULT_COEFFICIENTS.to_vec(),
            prime: DEFAULT_PRIME,
        }
    }
}
