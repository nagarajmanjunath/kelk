

pub trait Storage {
    fn sread<T: Sized>(&self, offset: u32, value: &mut T);
    fn swrite(&self);
}