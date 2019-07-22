use futures_test::task::noop_context;
use std::future::Future;
use std::task::Poll;

pub fn extract_future_output<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let mut cx = noop_context();
    pin_mut!(future);
    match future.poll(&mut cx) {
        Poll::Pending => panic!("had to be ready"),
        Poll::Ready(v) => v,
    }
}
