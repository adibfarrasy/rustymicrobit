pub trait OurFuture {
    type Output;
    fn poll(&mut self, task_id: usize) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Pending,
    Ready(T),
}
