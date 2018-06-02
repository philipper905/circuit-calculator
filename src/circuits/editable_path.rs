use super::Error;
use super::Path;
use super::Resistance;
use super::rayon::prelude::*;

#[derive(Debug, PartialEq)]
pub enum EditablePath {
    Resistor(Resistance),
    Series(Vec<EditablePath>),
    Parallel(Vec<EditablePath>),
}

impl EditablePath {
    pub fn into_immutable_path(self) -> Path {
        match self {
            EditablePath::Resistor(r) => Path::Resistor(r),
            EditablePath::Series(v) => Path::Series(
                v.into_par_iter()
                    .map(Self::into_immutable_path)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            EditablePath::Parallel(v) => Path::Parallel(
                v.into_par_iter()
                    .map(Self::into_immutable_path)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        }
    }

    pub fn parse(s: &str) -> Result<Self, Error> {
        let c = s.chars().next().unwrap();
        let (a, _) = match c {
            '[' => Self::parse_helper(EditablePath::Series(Vec::new()), &s[1..], 1),
            '<' => Self::parse_helper(EditablePath::Parallel(Vec::new()), &s[1..], 1),
            _ => Err(Error::UnopenedCircuit),
        }?;

        Ok(a)
    }

    fn parse_helper(current_stack: Self, s: &str, i: usize) -> Result<(Self, usize), Error> {
        let mut iter = s.chars();
        let c = iter.next();

        //    if i == 0 {
        //        println!("Reset to 0");
        //    } else {
        //        println!("N val is: {}", i);
        //    }

        match c {
            Some(ch) => match (ch, current_stack) {
                ('[', EditablePath::Series(mut p)) => {
                    let (a, b) = Self::parse_helper(EditablePath::Series(vec![]), &s[1..], 1)?;
                    p.push(a);
                    let c = EditablePath::Series(p);

                    Self::parse_helper(c, &s[b..], b + i)
                }

                ('<', EditablePath::Series(mut p)) => {
                    let (a, b) = Self::parse_helper(EditablePath::Parallel(vec![]), &s[1..], 1)?;
                    p.push(a);
                    let c = EditablePath::Series(p);

                    Self::parse_helper(c, &s[b..], b + i)
                }

                ('[', EditablePath::Parallel(mut p)) => {
                    let (a, b) = Self::parse_helper(EditablePath::Series(vec![]), &s[1..], 1)?;
                    p.push(a);
                    let c = EditablePath::Parallel(p);

                    Self::parse_helper(c, &s[b..], b + i)
                }

                ('<', EditablePath::Parallel(mut p)) => {
                    let (a, b) = Self::parse_helper(EditablePath::Parallel(vec![]), &s[1..], 1)?;
                    p.push(a);
                    let c = EditablePath::Parallel(p);

                    Self::parse_helper(c, &s[b..], b + 1)
                }

                (']', stack @ EditablePath::Series(_)) => Ok((stack, i + 1)),
                (']', _) => Err(Error::UnclosedParallel),
                ('>', stack @ EditablePath::Parallel(_)) => Ok((stack, i + 1)),
                ('>', _) => Err(Error::UnclosedSeries),
                ('0', c)
                | ('1', c)
                | ('2', c)
                | ('3', c)
                | ('4', c)
                | ('5', c)
                | ('6', c)
                | ('7', c)
                | ('8', c)
                | ('9', c) => match c {
                    EditablePath::Series(mut p) => {
                        let (num_str, skip) = Self::parse_number(&s, 0, String::new(), false);
                        p.push(EditablePath::Resistor(Resistance(num_str.parse().unwrap())));

                        Self::parse_helper(EditablePath::Series(p), &s[skip..], skip + i)
                    }
                    EditablePath::Parallel(mut p) => {
                        let (num_str, skip) = Self::parse_number(&s, 0, String::new(), false);
                        p.push(EditablePath::Resistor(Resistance(num_str.parse().unwrap())));

                        Self::parse_helper(EditablePath::Parallel(p), &s[skip..], skip + i)
                    }
                    EditablePath::Resistor(_) => panic!(),
                },
                (_, c) => Self::parse_helper(c, &s[1..], i + 1),
            },
            None => Err(Error::UnclosedUnknown),
        }
    }

    fn parse_number(
        s: &str,
        i: usize,
        mut current_num_str: String,
        mut first_period: bool,
    ) -> (String, usize) {
        let c = s.chars().next();

        match c {
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6')
            | Some('7') | Some('8') | Some('9') => {}
            Some('.') if first_period == false => {
                first_period = true;
            }
            _ => return (current_num_str, i),
        }

        current_num_str.push(c.unwrap());
        Self::parse_number(&s[1..], i + 1, current_num_str, first_period)
    }


}

#[test]
fn does_regular_parsing_correctly() {
    assert_eq!(
        EditablePath::parse("[1]"),
        Ok(EditablePath::Series(vec![
            EditablePath::Resistor(Resistance(1.0)),
        ]))
    );
    assert_eq!(
        EditablePath::parse("[1 2 3 4]"),
        Ok(EditablePath::Series(vec![
            EditablePath::Resistor(Resistance(1.0)),
            EditablePath::Resistor(Resistance(2.0)),
            EditablePath::Resistor(Resistance(3.0)),
            EditablePath::Resistor(Resistance(4.0))
        ])
        )
    );
//    assert_eq!(EditablePath::parse())
}

#[test]
fn parse_errors_correctly() {
    assert_eq!(
        EditablePath::parse("["),
        Err(Error::UnclosedUnknown)
    );

    assert_eq!(
        EditablePath::parse("[>"),
        Err(Error::UnclosedSeries)
    );

    assert_eq!(
        EditablePath::parse("<]"),
        Err(Error::UnclosedParallel)
    );

    assert_eq!(
        EditablePath::parse("1.0"),
        Err(Error::UnopenedCircuit)
    );
}
