// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<E: Environment> Scalar<E> {
    /// Casts a scalar from a base field, with lossy truncation.
    ///
    /// This method is commonly-used by hash-to-scalar algorithms,
    /// where the hash output does not need to preserve the full base field.
    pub fn from_field_lossy(field: &Field<E>) -> Result<Self> {
        // Note: We are reconstituting the base field into a scalar field.
        // This is safe as the scalar field modulus is less than the base field modulus,
        // and thus will always fit within a single base field element.
        debug_assert!(Scalar::<E>::size_in_bits() < Field::<E>::size_in_bits());

        // Truncate the field to the size in data bits (1 bit less than the MODULUS) of the scalar.
        // Slicing here is safe as the base field is larger than the scalar field.
        Self::from_bits_le(&field.to_bits_le()[..Scalar::<E>::size_in_data_bits()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network_environment::Console;

    type CurrentEnvironment = Console;

    const ITERATIONS: u64 = 10_000;

    #[test]
    fn test_from_field() -> Result<()> {
        let mut rng = TestRng::default();

        for _ in 0..ITERATIONS {
            // Sample a random scalar.
            let size_in_data_bits = Scalar::<CurrentEnvironment>::size_in_data_bits();
            let prepare = &Scalar::<CurrentEnvironment>::rand(&mut rng).to_bits_le()[0..size_in_data_bits];
            let expected = Scalar::<CurrentEnvironment>::from_bits_le(prepare)?;
            // Perform the operation.
            let candidate = Scalar::from_field_lossy(&expected.to_field()?)?;
            assert_eq!(expected, candidate);

            // Sample a random field.
            let expected = Field::<CurrentEnvironment>::rand(&mut rng);
            // Perform the operation.
            assert!(Scalar::from_field_lossy(&expected).is_ok());
        }
        Ok(())
    }
}
