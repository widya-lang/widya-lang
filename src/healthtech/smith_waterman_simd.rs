use serde::{Deserialize, Serialize};

/// ============================================================================
/// Smith-Waterman Local Sequence Alignment (DNA / Protein Sequencing)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentResult {
    pub max_score: i32,
    pub aligned_query: String,
    pub aligned_target: String,
    pub query_start: usize,
    pub query_end: usize,
    pub target_start: usize,
    pub target_end: usize,
}

pub struct SequenceAligner {
    pub match_score: i32,
    pub mismatch_penalty: i32,
    pub gap_open_penalty: i32,
    pub gap_extend_penalty: i32,
}

impl SequenceAligner {
    pub fn new() -> Self {
        Self {
            match_score: 2,
            mismatch_penalty: -1,
            gap_open_penalty: -2,
            gap_extend_penalty: -1,
        }
    }

    /// Smith-Waterman local alignment matrix computation
    pub fn align_dna(&self, seq1: &str, seq2: &str) -> AlignmentResult {
        let n = seq1.len();
        let m = seq2.len();
        let s1: Vec<char> = seq1.chars().collect();
        let s2: Vec<char> = seq2.chars().collect();

        let mut h = vec![vec![0i32; m + 1]; n + 1];
        let mut max_score = 0;
        let mut max_pos = (0, 0);

        for i in 1..=n {
            for j in 1..=m {
                let score_diag = h[i - 1][j - 1]
                    + if s1[i - 1] == s2[j - 1] {
                        self.match_score
                    } else {
                        self.mismatch_penalty
                    };
                let score_del = h[i - 1][j] + self.gap_open_penalty;
                let score_ins = h[i][j - 1] + self.gap_open_penalty;

                let val = 0.max(score_diag).max(score_del).max(score_ins);
                h[i][j] = val;

                if val > max_score {
                    max_score = val;
                    max_pos = (i, j);
                }
            }
        }

        AlignmentResult {
            max_score,
            aligned_query: seq1.to_string(),
            aligned_target: seq2.to_string(),
            query_start: 0,
            query_end: max_pos.0,
            target_start: 0,
            target_end: max_pos.1,
        }
    }
}
