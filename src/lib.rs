// Copyright 2022 Jeremy Wall (Jeremy@marzhilsltudios.com)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::marker::PhantomData;

use sycamore::prelude::*;

pub trait Dispatcher<Msg, Out> {
    fn apply(&self, msg: Msg, original: &ReadSignal<Out>) -> Out;
}

pub struct Reducer<'ctx, D, T, Msg>
where
    D: Dispatcher<Msg, T>,
{
    signal: &'ctx Signal<T>,
    dispatcher: &'ctx D,
    _phantom: PhantomData<Msg>,
}

impl<'ctx, D, T, Msg> Reducer<'ctx, D, T, Msg>
where
    D: Dispatcher<Msg, T>,
{
    pub fn new(cx: Scope<'ctx>, initial: T, dispatcher: D) -> &'ctx Self {
        let signal = create_signal(cx, initial);
        let dispatcher = create_ref(cx, dispatcher);
        create_ref(
            cx,
            Self {
                signal,
                dispatcher,
                _phantom: PhantomData,
            },
        )
    }

    pub fn dispatch(&self, msg: Msg) {
        self.signal.set(self.dispatcher.apply(msg, self.signal))
    }

    pub fn signal(&'ctx self) -> &'ctx ReadSignal<T> {
        self.signal
    }

    pub fn bind<F>(&'ctx self, cx: Scope<'ctx>, f: F)
    where
        F: Fn() -> Msg + 'ctx,
    {
        create_effect(cx, move || self.dispatch(f()));
    }
}

#[cfg(test)]
mod tests;
