/// Returns `true` if `nr` is power of 2 (including 1)
#[inline]
pub fn is_pow2(nr: u32) -> bool {
    if nr == 0 {
        return false;
    }
    nr.count_ones() == 1
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_is_pow2() {
        let pows: HashSet<_> = (0..20).map(|i| 2u32.pow(i)).collect();
        let mut max = 0;
        for pow in &pows {
            assert!(is_pow2(*pow));
            max = *pow;
        }

        for i in (0..max).step_by(100) {
            if pows.contains(&i) {
                continue;
            }
            assert!(!is_pow2(i));
        }
    }
}
