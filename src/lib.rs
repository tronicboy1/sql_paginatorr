pub struct LimitOffsetPair {
    pub limit: usize,
    pub offset: usize,
}

/**
 * Creates a list of limit-offset pairs for use in an SQL query that requires
 * pagination.
 */
pub fn get_limit_offset_pairs(chunk_size: usize, absolute_limit: usize) -> Vec<LimitOffsetPair> {
    if !validate_chunk_size(chunk_size, absolute_limit) {
        panic!("Chunk size must leave no remainder!")
    }

    let rng = 0..absolute_limit / chunk_size;

    rng.map(|i| {
        let offset = i * chunk_size;
        let limit = offset + chunk_size;
        LimitOffsetPair { limit, offset }
    })
    .collect()
}

fn validate_chunk_size(chunk_size: usize, total: usize) -> bool {
    total % chunk_size == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_chunking() {
        let chnk_sz = 250;
        let limit = 1000;
        let chunks = get_limit_offset_pairs(chnk_sz, limit);
        assert_eq!(chunks.len(), limit / chnk_sz);
        assert_eq!(chunks[1].offset, chunks[0].limit);
        let last = chunks.last().expect("Last");
        assert_eq!(last.limit, limit);
    }

    #[test]
    fn finds_invalid_chunk_size() {
        let chnk_sz = 250;
        let limit = 1023;
        assert!(!validate_chunk_size(chnk_sz, limit))
    }

    #[test]
    #[should_panic(expected = "Chunk size must leave no remainder!")]
    fn invalid_chunk_size_panics() {
        let chnk_sz = 250;
        let limit = 1001;
        get_limit_offset_pairs(chnk_sz, limit);
    }
}