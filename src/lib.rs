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
        }
    }
}

#[wasm_bindgen]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct IncrementFuture {
    context: Rc<RefCell<State>>,
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
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        let mut context = self.context.borrow_mut();
        context.counter += 1;
        std::task::Poll::Ready(context.counter)
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

        assert_eq!(1, futures_executor::block_on(fut1));
        assert_eq!(2, futures_executor::block_on(fut2));
    }
}
