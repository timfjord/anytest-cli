use crate::ArgsList;

pub fn concat<I1, S1, I2, S2>(vec1: I1, vec2: I2) -> ArgsList
where
    I1: IntoIterator<Item = S1>,
    S1: ToString,
    I2: IntoIterator<Item = S2>,
    S2: ToString,
{
    let mut args: ArgsList = vec![];

    args.extend(vec1.into_iter().map(|s| s.to_string()));
    args.extend(vec2.into_iter().map(|s| s.to_string()));

    args
}
