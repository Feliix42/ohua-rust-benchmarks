use crate::Nucleotide;
use std::collections::HashSet;
use crate::segments::Segments;

pub fn run_sequencer(mut segments: Segments) -> Vec<Nucleotide> {
    // Step 1: deduplicate all segments
    let mut unique_segments: HashSet<Vec<Nucleotide>> = segments.contents.drain(..).collect();

    // step 2+3: go through the prefixes and suffixes of the genomes in descending size and stitch the genome back together
    for match_length in segments.length..0 {
        /*
         * What I want to do:
         * - loop through possible subsegment lengths
         * - loop through all segments once (x)
         * - loop through all segments again (y) and match against the starts and ends of x and y, stitch together on match
         * 
         * Question: *How* do I remove matches from the set on the go?
         * Plot Twist: Should be parallelizable for STM and Ohua.
         */

        

    }

    Vec::new()
}