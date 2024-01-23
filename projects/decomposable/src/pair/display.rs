use super::*;

impl Display for DecomposablePairItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_config(f.alternate(), false, f)
    }
}

impl DecomposablePairItem {
    /// - align: `&`
    /// - gray: `\color{gray}`
    fn write_config(&self, align: bool, gray: bool, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.result)?;
        for (index, (lhs, rhs)) in self.factors.iter().enumerate() {
            if index == 0 {
                if align {
                    f.write_str("&")?;
                }
                f.write_str("=")?;
                if gray {
                    f.write_str("\\color{gray}{")?;
                }
                write!(f, "{}×{}", lhs, rhs)?;
            }
            else {
                f.write_str("=")?;
                write!(f, "{}×{}", lhs, rhs)?;
            }
        }
        if gray {
            f.write_str("}")?;
        }
        Ok(())
    }
}

impl Display for DecomposablePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for pair in self.clone().into_iter() {
            pair.write_config(f.alternate(), pair.symmetrical, f)?;
            f.write_str("\\\\\n")?;
        }
        Ok(())
    }
}
