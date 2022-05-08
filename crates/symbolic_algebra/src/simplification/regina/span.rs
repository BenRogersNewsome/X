use crate::algo::structures::{Expression, Identity};
use super::history::History;
use super::strand::Strand;

/// A struct to hold a shared reference to all the data needed for a simplification or expansion.
pub struct Span<'a, 'b> {
    strands: Vec<Strand<'a>>,
    algebra: &'b Vec<Identity>,
    simplify: bool,
    history: History,
}

impl<'a, 'b> Span<'a, 'b> where 'b: 'a {

    pub fn init(expression: &'a Expression, algebra: &'b Vec<Identity>, simplify: bool) -> Span<'a, 'b> {
        Span {
            strands: vec![
                Strand::init(expression, simplify),
            ],
            algebra,
            simplify,
            history: History::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.strands.len()
    }

    pub fn saturate(mut self) -> Self {
        // TODO: Should always go down the simplest route at any given time
        let mut last_simp = self.strands[0].simplicity;
        loop {
            let history_len = self.history.len();

            let mut position = 0;
            let mut min_simpl = self.strands[0].simplicity;

            for (i, strand) in self.strands.iter().enumerate() {
                if (strand.simplicity < min_simpl) == self.simplify {
                    min_simpl = strand.simplicity;
                    position = i;
                }
            }

            let simplest = self.strands.remove(position);
            self.strands.append(&mut simplest.propagate(&self.algebra, &mut self.history));
            
            if (min_simpl >= last_simp) == self.simplify {
                break
            }else{
                last_simp = min_simpl;
            }

            // if self.history.len() == history_len {
            //     break;
            // }
            println!("Strands: {}", self.strands.len());
        };
        self
    }

    fn iterate(mut self) -> Self {
        let old_strands = Self::pop_strands(&mut self.strands);
        for strand in old_strands {
            let mut propagated = strand.propagate(&self.algebra, &mut self.history);
            self.strands.append(&mut propagated);
        };
        self
    }

    /// Re-initialize strands to an empty vec and return all the old strands
    fn pop_strands(strands: &mut Vec<Strand<'a>>) -> Vec<Strand<'a>> {
        let current_number_of_strands = strands.len();
        // Todo: Optimise this capacity for average number of strands propagated
        std::mem::replace(strands, Vec::with_capacity(current_number_of_strands))
    }

    pub fn simplest(mut self) -> Self {
        let capacity = self.strands.len();
        let old_strands = std::mem::replace(&mut self.strands, Vec::with_capacity(capacity));
        
        let filtered_strands = old_strands.into_iter().fold(vec![], |mut accum: Vec<Strand>, strand| {
            if accum.len() == 0 || self.simplify == (strand.simplicity < accum[0].simplicity) {
                return vec![strand];
            }else if strand.simplicity == accum[0].simplicity {
                accum.push(strand);
                return accum;
            }else{
                return accum;
            }
        });

        self.strands = filtered_strands;
        self
    }

    pub fn trim(mut self) -> Self {
        let capacity = self.strands.len();
        let old_strands = std::mem::replace(&mut self.strands, Vec::with_capacity(capacity));
        for old_strand in old_strands {
            self.strands.push(old_strand.trim())
        }
        self
    }

}

impl<'a, 'b, Idx> std::ops::Index<Idx> for Span<'a, 'b> where Idx: std::slice::SliceIndex<[Strand<'a>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.strands.index(index)
    }
}


#[cfg(test)]
mod tests {

    use crate::expression;
    use super::super::{strand::Strand, history::History};
    use super::Span;

    #[test]
    fn test_simplest() {
        let expression = expression!(
            BinaryOperator {
                label: b'+'
            },
            BinaryOperator {
                label: b'*'
            },
            Element {
                label: b"a".to_vec()
            },
            Element {
                label: b"b".to_vec()
            },
            Element {
                label: b"c".to_vec()
            },
        );
        let simpler_expression = expression!(
            BinaryOperator {
                label: b'+'
            },
            Element {
                label: b"a".to_vec()
            },
            Element {
                label: b"b".to_vec()
            },
        );

        let span = Span {
            strands: vec![
                Strand::init(&expression, true),
                Strand::init(&simpler_expression, true)
            ],
            algebra: &vec![],
            simplify: true,
            history: History::new(),
        };

        let simplest = span.simplest();
        assert_eq!(simplest.len(), 1);
        assert_eq!(simplest[0].current, simpler_expression);
    }
}