use super::*;

pub(crate) struct Pile {
  pub(crate) amount: u128,
  pub(crate) divisibility: u8,
  pub(crate) symbol: Option<char>,
}

impl Display for Pile {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let cutoff = 10u128.pow(self.divisibility.into());

    let whole = self.amount / cutoff;
    let mut fractional = self.amount % cutoff;

    if fractional == 0 {
      write!(f, "{whole}")?;
    } else {
      let mut width = usize::from(self.divisibility);
      while fractional % 10 == 0 {
        fractional /= 10;
        width -= 1;
      }

      write!(f, "{whole}.{fractional:0>width$}")?;
    }

    if let Some(symbol) = self.symbol {
      write!(f, "\u{00A0}{symbol}")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display() {
    assert_eq!(
      Pile {
        amount: 0,
        divisibility: 0,
        symbol: None,
      }
      .to_string(),
      "0"
    );
    assert_eq!(
      Pile {
        amount: 25,
        divisibility: 0,
        symbol: None,
      }
      .to_string(),
      "25"
    );
    assert_eq!(
      Pile {
        amount: 0,
        divisibility: 1,
        symbol: None,
      }
      .to_string(),
      "0"
    );
    assert_eq!(
      Pile {
        amount: 1,
        divisibility: 1,
        symbol: None,
      }
      .to_string(),
      "0.1"
    );
    assert_eq!(
      Pile {
        amount: 1,
        divisibility: 2,
        symbol: None,
      }
      .to_string(),
      "0.01"
    );
    assert_eq!(
      Pile {
        amount: 10,
        divisibility: 2,
        symbol: None,
      }
      .to_string(),
      "0.1"
    );
    assert_eq!(
      Pile {
        amount: 1100,
        divisibility: 3,
        symbol: None,
      }
      .to_string(),
      "1.1"
    );
    assert_eq!(
      Pile {
        amount: 100,
        divisibility: 2,
        symbol: None,
      }
      .to_string(),
      "1"
    );
    assert_eq!(
      Pile {
        amount: 101,
        divisibility: 2,
        symbol: None,
      }
      .to_string(),
      "1.01"
    );
    assert_eq!(
      Pile {
        amount: u128::MAX,
        divisibility: 18,
        symbol: None,
      }
      .to_string(),
      "340282366920938463463.374607431768211455"
    );
    assert_eq!(
      Pile {
        amount: u128::MAX,
        divisibility: MAX_DIVISIBILITY,
        symbol: None,
      }
      .to_string(),
      "3.40282366920938463463374607431768211455"
    );
    assert_eq!(
      Pile {
        amount: 0,
        divisibility: 0,
        symbol: Some('$'),
      }
      .to_string(),
      "0\u{00A0}$"
    );
  }
}
