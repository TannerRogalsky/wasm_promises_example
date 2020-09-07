use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct State {
    counter: u32,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Context {
    inner: Rc<RefCell<State>>,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen]
    pub fn load(&self) -> IncrementFuture {
        IncrementFuture {
            context: self.inner.clone(),
            delta: 1,
        }
    }

    #[wasm_bindgen]
    pub fn load_with_delta(&self, delta: u32) -> IncrementFuture {
        IncrementFuture {
            context: self.inner.clone(),
            delta,
        }
    }
}

#[wasm_bindgen]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct IncrementFuture {
    context: Rc<RefCell<State>>,
    delta: u32,
}

#[wasm_bindgen]
impl IncrementFuture {
    #[wasm_bindgen(js_name = "await")]
    pub async fn run(self) -> u32 {
        self.await
    }
}

impl std::future::Future for IncrementFuture {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        self.delta = self.delta.saturating_sub(1);
        let delta = self.delta;
        let mut context = self.context.borrow_mut();
        context.counter += 1;
        if delta == 0 {
            std::task::Poll::Ready(context.counter)
        } else {
            cx.waker().clone().wake();
            std::task::Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let app = Context::new();
        let fut1 = app.load();
        let fut2 = app.load();
        let fut3 = app.load_with_delta(2);

        assert_eq!(1, futures_executor::block_on(fut1));
        assert_eq!(2, futures_executor::block_on(fut2));
        assert_eq!(4, futures_executor::block_on(fut3));
    }
}
