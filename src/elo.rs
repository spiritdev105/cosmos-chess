//! The Elo algorithm, the most widespread rating system and the gold-standard in chess and other games.  
//! Used in the official FIDE chess ratings, many online games, and the basis of even more rating systems.
//!
//! The higher the Elo rating number, the stronger the player.
//! Compared to other rating algorithms, Elo ratings are relatively static, but very transparent and simple to calculate.
//!
//! # Quickstart
//!
//! This is the most basic example on how to use the Elo Module.  
//! Please take a look at the functions below to see more advanced use cases.
//!
//!
//! use elo::{elo, EloConfig, EloRating, Outcomes},
//!
//! // Initialise a new player rating with a rating of 1000.
//! let player_one = EloRating::new();
//!
//! // Or you can initialise it with your own values of course.
//! // Imagine these numbers being pulled from a database.
//! let some_rating = 1325;
//! let player_two = EloRating {
//!   rating: some_rating,
//! };
//!
//! // The outcome of the match is from the perspective of player one.
//! let outcome = Outcomes::WIN;
//!
//! // The config allows you to specify certain values in the Elo calculation.
//! // Here we modify the k-value to be 20.0, instead of the usual 32.0.
//! // To simplify massively: This means the ratings will not change as much.
//! let config = EloConfig { k: 20 };
//!
//! // The elo function will calculate the new ratings for both players and return them.
//! let (new_player_one, new_player_two) = elo(&player_one, &player_two, &outcome, &config);
//!
//!
//! # More Information
//!
//! - [Wikipedia Article](https://en.wikipedia.org/wiki/Elo_rating_system)
//! - [Elo Calculator](https://www.omnicalculator.com/sports/elo)
//! - [FIDE Ratings](https://ratings.fide.com/)

/// Constants
const LN10: u64 = 2358; //ln(10)
const E: u64 = 2784;    //e
const PREC: u64 = 10;   //precision

/// Calculates the exponential function e^x
fn fp_exp(x: u64) -> u64 {
  let mut result = 1 << PREC; // Start with 1 in fixed-point
  let mut term = 1 << PREC;   // The current term, starts with x^0 / 0! = 1

  for i in 1..=10 {
    // Calculate x^i / i!
    term = ((term * x) >> PREC) / (i as u64);
    // Add the term to the result
    result += term;

    // Break early if the term is too small to affect the result
    if term == 0 {
      break;
    }
  }

  result
}

/// Calculates the exponential function e^x for integer
fn fp_exp_int(x: u64) -> u64 {
  let mut s: u64 = 1 << PREC;
  for _ in 1..=x {
    s = (s * E) >> PREC;
  }
  s
}

/// Calculates 10^x using fixed-point arithmetic
fn fp_pow10(x: u64) -> u64 {
  // multiply x by ln(10) to convert to e^x form
  let exponent = (x * LN10) >> PREC;
  
  // compute e^(x * ln(10))
  let e1 = exponent >> PREC;
  let e2 = exponent - (e1 << PREC);

  (fp_exp_int(e1) * fp_exp(e2)) >> PREC
}

/// The possible outcomes for a match: Win, Draw, Loss.
///
/// Note that this is always from the perspective of player one.  
/// That means a win is a win for player one and a loss is a win for player two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcomes {
  /// A win, from player_one's perspective.
  WIN,
  /// A loss, from player_one's perspective.
  LOSS,
  /// A draw.
  DRAW,
}

impl Outcomes {
  #[must_use]
  /// Converts the outcome of the match into the points used in chess (1 = Win, 0.5 = Draw, 0 = Loss).
  ///
  /// Used internally in several rating algorithms, but some, like TrueSkill, have their own conversion.
  pub const fn to_chess_points(self) -> u64 {
    // Could set the visibility to crate level, but maybe someone has a use for it, who knows.
    match self {
      Self::WIN => 1 << PREC,         //1.0
      Self::DRAW => 1 << (PREC - 1),  //0.5
      Self::LOSS => 0,                //0.0
    }
  }
}

/// The Elo rating of a player
///
/// The default rating is 1000
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EloRating {
  /// The player's Elo rating number, by default 1000.
  pub rating: u64,
}

impl EloRating {
  /// Initialise a new `EloRating` with a rating of 1000.
  #[must_use]
  pub const fn new() -> Self {
    Self { rating: 1000 }
  }
}

impl Default for EloRating {
  fn default() -> Self {
    Self::new()
  }
}

impl From<u64> for EloRating {
  fn from(r: u64) -> Self {
    Self { rating: r }
  }
}

impl From<EloRating> for u64 {
  fn from(elo: EloRating) -> u64 {
      elo.rating
  }
}

#[derive(Clone, Copy, Debug)]
/// Constants used in the Elo calculations.
pub struct EloConfig {
  /// The k-value is the maximum amount of rating change from a single match.
  /// In chess, k-values from 40 to 10 are used, with the most common being 32, 24, 16 or 10.
  /// The higher the number, the more volatile the ranking.  
  /// Here the default is 32.
  pub k: u64,
}

impl EloConfig {
  #[must_use]
  /// Initialise a new `EloConfig` with a k value of `32.0`.
  pub const fn new() -> Self {
    Self { k: 32 }
  }
}

impl Default for EloConfig {
  fn default() -> Self {
  Self::new()
  }
}

/// Calculates the [`EloRating`]s of two players based on their old ratings and the outcome of the game.
///
/// Takes in two players as [`EloRating`]s, an [`Outcome`](Outcomes) and an [`EloConfig`].
///
/// The outcome of the match is in the perspective of `player_one`.
/// This means [`Outcomes::WIN`] is a win for `player_one` and [`Outcomes::LOSS`] is a win for `player_two`.
///
/// # Examples
///
/// use elo::{elo, EloConfig, EloRating, Outcomes};
///
/// let player_one = EloRating { rating: 600 };
/// let player_two = EloRating { rating: 711 };
///
/// let outcome = Outcomes::WIN;
///
/// let config = EloConfig::new();
///
/// let (new_one, new_two) = elo(&player_one, &player_two, &outcome, &config);
///
/// assert!(new_one == 620);
/// assert!(new_two == 690);
/// ```
#[must_use]
pub fn elo(
  player_one: &EloRating,
  player_two: &EloRating,
  outcome: &Outcomes,
  config: &EloConfig,
) -> (EloRating, EloRating) {
  let expected = expected_score(player_one, player_two);
  let outcome = outcome.to_chess_points();

  let one_new_elo = ((player_one.rating << PREC) + config.k * outcome - config.k * expected) >> PREC;
  let two_new_elo = player_one.rating + player_two.rating - one_new_elo;

  (
    EloRating {
      rating: one_new_elo,
    },
    EloRating {
      rating: two_new_elo,
    },
  )
}

/// Calculates the expected score of two players based on their elo rating.
///
/// Takes in two players as [`EloRating`]s and returns the probability of victory for each player as an [`f64`] between 1.0 and 0.0.  
/// 1.0 means a certain victory for the player, 0.0 means certain loss.
/// Values near 0.5 mean a draw is likely to occur.
///
/// # Examples
///
/// use elo::{expected_score, EloRating};
///
/// let player_one = EloRating { rating: 1320 };
/// let player_two = EloRating { rating: 1217 };
///
/// let (exp1, exp2) = expected_score(&player_one, &player_two);
///
/// assert!(exp1 == 64);
/// assert!(exp2 == 36);
///
#[must_use]
pub fn expected_score(player_one: &EloRating, player_two: &EloRating) -> u64 {
  let diff = if player_one.rating >= player_two.rating {
    player_one.rating - player_two.rating
  } else {
    player_two.rating - player_one.rating
  };
  let exp_one = (1 << (PREC + PREC)) / ((1 << PREC) + fp_pow10((diff << PREC) / 400));

  if player_two.rating >= player_one.rating {
    exp_one
  } else {
    (1 << PREC) - exp_one
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_elo() {
    let (winner_new_elo, loser_new_elo) = elo(
      &EloRating { rating: 1000 },
      &EloRating { rating: 1000 },
      &Outcomes::WIN,
      &EloConfig::new(),
    );
    assert!(winner_new_elo.rating == 1016);
    assert!(loser_new_elo.rating == 984);
  
    let (winner_new_elo, loser_new_elo) = elo(
      &EloRating { rating: 1000 },
      &EloRating { rating: 1000 },
      &Outcomes::LOSS,
      &EloConfig::new(),
    );
    assert!(winner_new_elo.rating == 984);
    assert!(loser_new_elo.rating == 1016);
  
    let (winner_new_elo, loser_new_elo) = elo(
      &EloRating { rating: 1000 },
      &EloRating { rating: 1000 },
      &Outcomes::DRAW,
      &EloConfig::new(),
    );
    assert!(winner_new_elo.rating == 1000);
    assert!(loser_new_elo.rating == 1000);
  
    let (winner_new_elo, loser_new_elo) = elo(
      &EloRating { rating: 500 },
      &EloRating { rating: 1500 },
      &Outcomes::WIN,
      &EloConfig::default(),
    );
    assert!(winner_new_elo.rating == 531);
    assert!(loser_new_elo.rating == 1469);
  }

  #[test]
  fn test_expected_score() {
    let player_one = EloRating::new();
    let player_two = EloRating::default();
  
    let winner_expected = expected_score(&player_one, &player_two);
  
    assert!(((winner_expected * 100) >> PREC) == 50);
    // loser: 50%
  
    let player_one = EloRating { rating: 2251 };
    let player_two = EloRating { rating: 1934 };
  
    let winner_expected = expected_score(&player_one, &player_two);
  
    assert!(((winner_expected * 100) >> PREC) == 86);
    // loser: 14%
  }

  #[test]
  #[allow(clippy::clone_on_copy)]
  fn test_misc_stuff() {
    let player_one = EloRating::new();
    let config = EloConfig::new();
  
    assert_eq!(player_one, player_one.clone());
    assert!(config.k == config.clone().k);
  
    assert!(!format!("{player_one:?}").is_empty());
    assert!(!format!("{config:?}").is_empty());
  
    assert_eq!(player_one, EloRating::from(1000));
  }
}