use super::Simplifiable;
use log::{debug, info};

pub enum Polarity {
    Simplify,
    Expand,
}

/// A depth first algorithm for propagating strands - be warned, this is a heavily recursive function.
pub fn propagate_<'a, T: 'a>(
    strand: T,
    identities: &'a [T::Identity],
    polarity: isize,
    history: &mut Vec<u64>,
    max_strand_depth: usize,
    depth_since_last_simplest: usize,
    last_simplest: usize,
) -> Vec<T> where T: Simplifiable<'a> {

    let mut final_strands: Vec<T> = vec![];

    for identity in identities {
        // info!(
        //     "Trying {}",
        //     iden_string,
        // );

        let mut new_strands_with_instructions = strand.try_manipulate(&identity).expect("Error unwrapping strands");
        new_strands_with_instructions.sort_by(
            |a, b|{
                (a.0.simplicity() as isize).cmp(&(polarity * b.0.simplicity() as isize))
            });
        
        for (new_strand, _) in new_strands_with_instructions {

            if (&history).contains(&new_strand.uuid()) {
                continue
            }


            history.push(new_strand.uuid());

            // Should we ditch the current strand?
            if new_strand.simplicity() as isize > polarity * (last_simplest as isize)
                && depth_since_last_simplest > max_strand_depth {
                info!("Simplicity depth reached");
                continue
            }

            let (new_depth_since_last_simplest, new_last_simplest) =
                match (new_strand.simplicity() as isize) < polarity * (last_simplest as isize) {
                    true => {
                        info!("Simpler");
                        (0, new_strand.simplicity())
                    },
                    false => (depth_since_last_simplest + 1, last_simplest),
                };
            
            debug!("Propagating");
            let propagated = propagate_(
                new_strand,
                identities,
                polarity,
                history,
                max_strand_depth,
                new_depth_since_last_simplest,
                new_last_simplest,
            );

            final_strands.extend(propagated);
        }
    }

    // Base case
    if final_strands.len() == 0 {
        return vec![strand]
    }else{
        return final_strands
    }
}

// fn should_propagate_strand<'a, T>(
//     strand: Strand<'a, T>,
//     polarity: isize,
//     max_strand_depth: usize,
//     depth_since_last_simplest: usize,
//     last_simplest: usize
// ) -> Option<(usize, usize)> where T: Simplifiable<'a>{

//     // Should this be less than or less than equal to??
//     if (strand.current.simplicity() as isize) <= polarity * last_simplest as isize {

//         return Some((strand.current.simplicity(), 0))
//     } else if (strand.current.simplicity() as isize) > polarity * last_simplest as isize && depth_since_last_simplest < max_strand_depth {

//         return Some((last_simplest, depth_since_last_simplest + 1))

//     } else {
//         return None
//     }

// }

