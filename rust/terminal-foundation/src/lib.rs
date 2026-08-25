//! Safe, platform-neutral foundation primitives shared across migrated Terminal layers.
//!
//! F01 moves deterministic TIL/value semantics into Rust without recreating
//! Win32/COM representation details that remain native boundaries.

#![forbid(unsafe_code)]

/// A monotonically increasing state generation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Generation(u32);

impl Generation {
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    fn bump(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

/// Value wrapper whose equality intentionally tracks mutation generation, not `T` equality.
///
/// This mirrors `til::generational`: reads are cheap, while every mutable access bumps the
/// generation so downstream caches can cheaply detect any state change.
#[derive(Clone, Debug)]
pub struct Generational<T> {
    generation: Generation,
    value: T,
}

impl<T: Default> Default for Generational<T> {
    fn default() -> Self {
        Self {
            generation: Generation::default(),
            value: T::default(),
        }
    }
}

impl<T> Generational<T> {
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            generation: Generation(0),
            value,
        }
    }

    #[must_use]
    pub const fn with_generation(generation: Generation, value: T) -> Self {
        Self { generation, value }
    }

    #[must_use]
    pub const fn generation(&self) -> Generation {
        self.generation
    }

    #[must_use]
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Marks the wrapped state as changed before returning mutable access.
    pub fn write(&mut self) -> &mut T {
        self.generation.bump();
        &mut self.value
    }
}

impl<T> PartialEq for Generational<T> {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
    }
}

impl<T> Eq for Generational<T> {}

#[cfg(test)]
mod tests {
    use super::{Generation, Generational};

    #[derive(Clone, Debug, Default)]
    struct Data {
        value: i32,
    }

    #[test]
    fn microsoft_til_generational_basic_matches_source_contract() {
        let mut src = Generational::<Data>::default();
        let mut dst = Generational::<Data>::default();

        assert_eq!(0, src.get().value);

        src.write().value = 123;
        assert_ne!(dst, src);

        dst = src.clone();
        assert_eq!(dst, src);
        assert_eq!(123, dst.get().value);
    }

    #[test]
    fn generational_equality_is_generation_based_not_value_based() {
        let mut left = Generational::new(10_u32);
        let mut right = Generational::new(99_u32);
        assert_eq!(left, right);

        *left.write() = 99;
        assert_ne!(left, right);
        right.write();
        assert_eq!(left, right);
    }

    #[test]
    fn explicit_generation_is_preserved_and_wraps_like_uint32() {
        let max = Generation(u32::MAX);
        let mut value = Generational::with_generation(max, 1_u8);
        value.write();
        assert_eq!(0, value.generation().value());
    }
}
