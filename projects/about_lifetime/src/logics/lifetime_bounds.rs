use std::fmt::Debug;

trait BufferBehavior<'a, T: 'a> {
    fn buffer(&self)
    where
        T: Debug,
    {
    }
    fn get_value(&self) -> &'a T;
}

#[derive(Debug)]
struct Buffer<'a, T> {
    value: &'a T,
}

impl<'buffer_life, T> BufferBehavior<'buffer_life, T> for Buffer<'buffer_life, T> {
    fn buffer(&self)
    where
        T: Debug,
    {
        println!("BufferBehavior: {:?}", self.value);
    }

    fn get_value(&self) -> &'buffer_life T {
        self.value
    }
}

pub fn run_lifetime_bound() {
    let value = 100;
    let buffer = Buffer { value: &value };
    println!("{:?}", buffer);
    buffer.buffer();
    println!("buffer value: {:?}", buffer.get_value());
}
