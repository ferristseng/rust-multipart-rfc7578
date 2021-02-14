// Copyright 2021 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};
use std::iter::FromIterator;

/// A `BoundaryGenerator` is a policy to generate a random string to use
/// as a part boundary.
///
/// The default generator will build a random string of 6 ascii characters.
/// If you need more complexity, you can implement this, and use it with
/// [`Form::new`].
///
/// # Examples
///
/// ```
/// use common_multipart_rfc7578::client::multipart::BoundaryGenerator;
///
/// struct TestGenerator;
///
/// impl BoundaryGenerator for TestGenerator {
///     fn generate_boundary() -> String {
///         "test".to_string()
///     }
/// }
/// ```
pub trait BoundaryGenerator {
    /// Generates a String to use as a boundary.
    ///
    fn generate_boundary() -> String;
}

pub(crate) struct RandomAsciiGenerator;

impl BoundaryGenerator for RandomAsciiGenerator {
    /// Creates a boundary of 6 ascii characters.
    ///
    fn generate_boundary() -> String {
        let rng = SmallRng::from_entropy();
        let ascii = rng.sample_iter(&Alphanumeric);

        String::from_iter(ascii.map(|b| b as char).take(6))
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundaryGenerator, RandomAsciiGenerator};

    #[test]
    fn generate_random_boundary_not_empty() {
        assert!(RandomAsciiGenerator::generate_boundary().len() > 0);
    }

    #[test]
    fn generate_random_boundary_different_each_time() {
        assert!(
            RandomAsciiGenerator::generate_boundary() != RandomAsciiGenerator::generate_boundary()
        );
    }
}
