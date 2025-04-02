
pub trait CreateFrom<T> {

    type Metadata: Clone + Copy;


    fn create_from(from: T, meta: Self::Metadata) -> Self;
}
pub struct DummyCreateFrom {}

impl<'data, T> CreateFrom<&'data [T]> for DummyCreateFrom {
    type Metadata = ();

    #[inline]
    fn create_from(_: &'data [T], _: Self::Metadata) -> Self {
        Self {}
    }
}

impl<'data, T> CreateFrom<&'data mut [T]> for DummyCreateFrom {
    type Metadata = ();

    #[inline]
    fn create_from(_: &'data mut [T], _: Self::Metadata) -> Self {
        Self {}
    }
}
