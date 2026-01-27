#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! implement_generic_index {
    ($index:ident, $optional_index:ident) => {
        struct $index<IndexType>(IndexType);

        struct $optional_index<IndexType>(IndexType);

        implement_generic_index!($index, $optional_index, __inner__);
    };

    (pub $index:ident, pub $optional_index:ident) => {
        pub struct $index<IndexType>(IndexType);

        pub struct $optional_index<IndexType>(IndexType);

        implement_generic_index!($index, $optional_index, __inner__);
    };

    (pub(crate) $index:ident, pub(crate) $optional_index:ident) => {
        pub(crate) struct $index<IndexType>(IndexType);

        pub(crate) struct $optional_index<IndexType>(IndexType);

        implement_generic_index!($index, $optional_index, __inner__);
    };

    (pub(super) $index:ident, pub(super) $optional_index:ident) => {
        pub(super) struct $index<IndexType>(IndexType);

        pub(super) struct $optional_index<IndexType>(IndexType);

        implement_generic_index!($index, $optional_index, __inner__);
    };

    (pub(in $index_visibility:path) $index:ident, pub(in $optional_index_visibility:path) $optional_index:ident) => {
        pub(in $index_visibility) struct $index<IndexType>(IndexType);

        pub(in $optional_index_visibility) struct $optional_index<IndexType>(IndexType);

        implement_generic_index!($index, $optional_index, __inner__);
    };

    ($index:ident, $optional_index:ident, __inner__) => {
        impl<IndexType> $index<IndexType> {
            #[allow(dead_code)]
            pub fn new(value: IndexType) -> Self
            where
                IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug,
            {
                assert_ne!(value, IndexType::max_value());
                Self(value)
            }

            #[allow(dead_code)]
            pub fn from_usize(value: usize) -> Self
            where
                IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug + TryFrom<usize>,
            {
                Self::new(
                    value
                        .try_into()
                        .ok()
                        .expect("index conversion from usize failed"),
                )
            }

            #[allow(dead_code)]
            pub fn from_raw(value: IndexType) -> Self
            where
                IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug,
            {
                Self::new(value)
            }

            #[allow(dead_code)]
            pub fn into_raw(self) -> IndexType {
                self.0
            }
        }

        impl<IndexType> $optional_index<IndexType> {
            #[allow(dead_code)]
            pub fn new_some(value: IndexType) -> Self
            where
                IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug,
            {
                assert_ne!(value, IndexType::max_value());
                Self(value)
            }

            #[allow(dead_code)]
            pub fn new_none() -> Self
            where
                IndexType: num_traits::bounds::UpperBounded,
            {
                Self(IndexType::max_value())
            }

            #[allow(dead_code)]
            pub fn from_raw(value: Option<IndexType>) -> Self
            where
                IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug,
            {
                if let Some(value) = value {
                    Self::new_some(value)
                } else {
                    Self::new_none()
                }
            }

            #[allow(dead_code)]
            pub fn into_raw(self) -> Option<IndexType>
            where
                IndexType: num_traits::bounds::UpperBounded + Eq,
            {
                if self.is_some() { Some(self.0) } else { None }
            }

            #[allow(dead_code)]
            pub fn is_some(&self) -> bool
            where
                IndexType: num_traits::bounds::UpperBounded + Eq,
            {
                self.0 != IndexType::max_value()
            }

            #[allow(dead_code)]
            pub fn is_none(&self) -> bool
            where
                IndexType: num_traits::bounds::UpperBounded + Eq,
            {
                self.0 == IndexType::max_value()
            }
        }

        /////////////////////////
        ////// Conversions //////
        /////////////////////////

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug>
            From<Option<$index<IndexType>>> for $optional_index<IndexType>
        {
            fn from(index: Option<$index<IndexType>>) -> Self {
                if let Some(index) = index {
                    Self::new_some(index.0)
                } else {
                    Self::new_none()
                }
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug>
            From<$index<IndexType>> for $optional_index<IndexType>
        {
            fn from(index: $index<IndexType>) -> Self {
                Self::new_some(index.0)
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug>
            From<$optional_index<IndexType>> for Option<$index<IndexType>>
        {
            fn from(optional_index: $optional_index<IndexType>) -> Self {
                if optional_index.is_some() {
                    Some($index::new(optional_index.0))
                } else {
                    None
                }
            }
        }

        ////////////////////////////////////
        ////// Conversions with usize //////
        ////////////////////////////////////

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug + TryFrom<usize>>
            From<usize> for $index<IndexType>
        {
            fn from(value: usize) -> Self {
                Self::new(
                    value
                        .try_into()
                        .ok()
                        .expect("index conversion from usize failed"),
                )
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug + TryFrom<usize>>
            From<usize> for $optional_index<IndexType>
        {
            fn from(value: usize) -> Self {
                Self::new_some(
                    value
                        .try_into()
                        .ok()
                        .expect("index conversion from usize failed"),
                )
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug + TryFrom<usize>>
            From<Option<usize>> for $optional_index<IndexType>
        {
            fn from(value: Option<usize>) -> Self {
                if let Some(value) = value {
                    Self::new_some(
                        value
                            .try_into()
                            .ok()
                            .expect("index conversion from usize failed"),
                    )
                } else {
                    Self::new_none()
                }
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug + TryInto<usize>>
            From<$index<IndexType>> for usize
        {
            fn from(value: $index<IndexType>) -> Self {
                value
                    .0
                    .try_into()
                    .ok()
                    .expect("index conversion from usize failed")
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + TryInto<usize>>
            From<$optional_index<IndexType>> for Option<usize>
        {
            fn from(index: $optional_index<IndexType>) -> Self {
                if index.is_some() {
                    Some(
                        index
                            .0
                            .try_into()
                            .ok()
                            .expect("index conversion to usize failed"),
                    )
                } else {
                    None
                }
            }
        }

        ////////////////////////
        ////// Formatting //////
        ////////////////////////

        impl<IndexType: std::fmt::Debug> std::fmt::Debug for $index<IndexType> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($index), self.0)
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Debug> std::fmt::Debug
            for $optional_index<IndexType>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.is_some() {
                    write!(f, "{}({:?})", stringify!($optional_index), self.0)
                } else {
                    write!(f, "{}(None)", stringify!($optional_index))
                }
            }
        }

        impl<IndexType: std::fmt::Display> std::fmt::Display for $index<IndexType> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<IndexType: num_traits::bounds::UpperBounded + Eq + std::fmt::Display> std::fmt::Display
            for $optional_index<IndexType>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.is_some() {
                    write!(f, "{}", self.0)
                } else {
                    write!(f, "None")
                }
            }
        }

        //////////////////////////
        ////// Clone + Copy //////
        //////////////////////////

        impl<IndexType: Clone> Clone for $index<IndexType> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<IndexType: Clone> Clone for $optional_index<IndexType> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<IndexType: Copy> Copy for $index<IndexType> {}

        impl<IndexType: Copy> Copy for $optional_index<IndexType> {}

        //////////////////////
        ////// Equality //////
        //////////////////////

        impl<IndexType: PartialEq> PartialEq for $index<IndexType> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<IndexType: PartialEq> PartialEq for $optional_index<IndexType> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<IndexType: Eq> Eq for $index<IndexType> {}

        impl<IndexType: Eq> Eq for $optional_index<IndexType> {}

        //////////////////////
        ////// Ordering //////
        //////////////////////

        impl<IndexType: PartialOrd> PartialOrd for $index<IndexType> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<IndexType: PartialOrd> PartialOrd for $optional_index<IndexType> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<IndexType: Ord> Ord for $index<IndexType> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl<IndexType: Ord> Ord for $optional_index<IndexType> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        /////////////////////
        ////// Hashing //////
        /////////////////////

        impl<IndexType: std::hash::Hash> std::hash::Hash for $index<IndexType> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl<IndexType: std::hash::Hash> std::hash::Hash for $optional_index<IndexType> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}

#[macro_export]
macro_rules! implement_fixed_index {
    ($index:ident, $optional_index:ident, $index_type:ty) => {
        struct $index($index_type);

        struct $optional_index($index_type);

        implement_fixed_index!($index, $optional_index, $index_type, __inner__);
    };

    (pub $index:ident, pub $optional_index:ident, $index_type:ty) => {
        pub struct $index($index_type);

        pub struct $optional_index($index_type);

        implement_fixed_index!($index, $optional_index, $index_type, __inner__);
    };

    (pub(crate) $index:ident, pub(crate) $optional_index:ident, $index_type:ty) => {
        pub(crate) struct $index($index_type);

        pub(crate) struct $optional_index($index_type);

        implement_fixed_index!($index, $optional_index, $index_type, __inner__);
    };

    (pub(super) $index:ident, pub(super) $optional_index:ident, $index_type:ty) => {
        pub(super) struct $index($index_type);

        pub(super) struct $optional_index($index_type);

        implement_fixed_index!($index, $optional_index, $index_type, __inner__);
    };

    (pub(in $index_visibility:path) $index:ident, pub(in $optional_index_visibility:path) $optional_index:ident, $index_type:ty) => {
        pub(in $index_visibility) struct $index($index_type);

        pub(in $optional_index_visibility) struct $optional_index($index_type);

        implement_fixed_index!($index, $optional_index, $index_type, __inner__);
    };

    ($index:ident, $optional_index:ident, $index_type:ty, __inner__) => {
        impl $index {
            #[allow(dead_code)]
            pub fn new(value: $index_type) -> Self {
                assert_ne!(value, num_traits::bounds::UpperBounded::max_value());
                Self(value)
            }

            #[allow(dead_code)]
            pub fn from_raw(value: $index_type) -> Self {
                Self::new(value)
            }

            #[allow(dead_code)]
            pub fn into_raw(self) -> $index_type {
                self.0
            }
        }

        impl $optional_index {
            #[allow(dead_code)]
            pub fn new_some(value: $index_type) -> Self {
                assert_ne!(value, num_traits::bounds::UpperBounded::max_value());
                Self(value)
            }

            #[allow(dead_code)]
            pub fn new_none() -> Self {
                Self(num_traits::bounds::UpperBounded::max_value())
            }

            #[allow(dead_code)]
            pub fn from_raw(value: Option<$index_type>) -> Self {
                if let Some(value) = value {
                    Self::new_some(value)
                } else {
                    Self::new_none()
                }
            }

            #[allow(dead_code)]
            pub fn into_raw(self) -> Option<$index_type> {
                if self.is_some() { Some(self.0) } else { None }
            }

            #[allow(dead_code)]
            pub fn is_some(&self) -> bool {
                self.0 != num_traits::bounds::UpperBounded::max_value()
            }

            #[allow(dead_code)]
            pub fn is_none(&self) -> bool {
                self.0 == num_traits::bounds::UpperBounded::max_value()
            }
        }

        /////////////////////////
        ////// Conversions //////
        /////////////////////////

        impl From<Option<$index>> for $optional_index {
            fn from(index: Option<$index>) -> Self {
                if let Some(index) = index {
                    Self::new_some(index.0)
                } else {
                    Self::new_none()
                }
            }
        }

        impl From<$index> for $optional_index {
            fn from(index: $index) -> Self {
                Self::new_some(index.0)
            }
        }

        impl From<$optional_index> for Option<$index> {
            fn from(optional_index: $optional_index) -> Self {
                if optional_index.is_some() {
                    Some($index::new(optional_index.0))
                } else {
                    None
                }
            }
        }

        ////////////////////////////////////
        ////// Conversions with usize //////
        ////////////////////////////////////

        impl From<usize> for $index {
            fn from(value: usize) -> Self {
                Self::new(
                    value
                        .try_into()
                        .ok()
                        .expect("index conversion from usize failed"),
                )
            }
        }

        impl From<usize> for $optional_index {
            fn from(value: usize) -> Self {
                Self::new_some(
                    value
                        .try_into()
                        .ok()
                        .expect("index conversion from usize failed"),
                )
            }
        }

        impl From<Option<usize>> for $optional_index {
            fn from(value: Option<usize>) -> Self {
                if let Some(value) = value {
                    Self::new_some(
                        value
                            .try_into()
                            .ok()
                            .expect("index conversion from usize failed"),
                    )
                } else {
                    Self::new_none()
                }
            }
        }

        impl From<$index> for usize {
            fn from(index: $index) -> Self {
                index
                    .0
                    .try_into()
                    .ok()
                    .expect("index conversion to usize failed")
            }
        }

        impl From<$optional_index> for Option<usize> {
            fn from(index: $optional_index) -> Self {
                if index.is_some() {
                    Some(
                        index
                            .0
                            .try_into()
                            .ok()
                            .expect("index conversion to usize failed"),
                    )
                } else {
                    None
                }
            }
        }

        ////////////////////////
        ////// Formatting //////
        ////////////////////////

        impl std::fmt::Debug for $index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($index), self.0)
            }
        }

        impl std::fmt::Debug for $optional_index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.is_some() {
                    write!(f, "{}({:?})", stringify!($optional_index), self.0)
                } else {
                    write!(f, "{}(None)", stringify!($optional_index))
                }
            }
        }

        impl std::fmt::Display for $index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::fmt::Display for $optional_index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.is_some() {
                    write!(f, "{}", self.0)
                } else {
                    write!(f, "None")
                }
            }
        }

        //////////////////////////
        ////// Clone + Copy //////
        //////////////////////////

        impl Clone for $index {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Clone for $optional_index {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Copy for $index {}

        impl Copy for $optional_index {}

        //////////////////////
        ////// Equality //////
        //////////////////////

        impl PartialEq for $index {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl PartialEq for $optional_index {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $index {}

        impl Eq for $optional_index {}

        //////////////////////
        ////// Ordering //////
        //////////////////////

        impl PartialOrd for $index {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialOrd for $optional_index {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $index {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl Ord for $optional_index {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        /////////////////////
        ////// Hashing //////
        /////////////////////

        impl std::hash::Hash for $index {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl std::hash::Hash for $optional_index {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}
