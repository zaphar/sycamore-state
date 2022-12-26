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
use std::{marker::PhantomData, rc::Rc};

use sycamore::prelude::*;

pub trait MessageMapper<Msg, Out> {
    fn map(&self, msg: Msg, original: &ReadSignal<Out>) -> Out;
}

pub struct Handler<'ctx, D, T, Msg>
where
    D: MessageMapper<Msg, T>,
{
    signal: &'ctx Signal<T>,
    dispatcher: &'ctx D,
    _phantom: PhantomData<Msg>,
}

impl<'ctx, D, T, Msg> Handler<'ctx, D, T, Msg>
where
    D: MessageMapper<Msg, T>,
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

    fn dispatch(&self, msg: Msg) {
        self.signal.set(self.dispatcher.map(msg, self.signal))
    }

    pub fn read_signal(&'ctx self) -> &'ctx ReadSignal<T> {
        self.signal
    }

    pub fn bind_trigger<F, Val>(
        &'ctx self,
        cx: Scope<'ctx>,
        trigger: &'ctx ReadSignal<Val>,
        message_fn: F,
    ) where
        F: Fn(Rc<Val>) -> Msg + 'ctx,
    {
        create_effect(cx, move || self.dispatch(message_fn(trigger.get())));
    }

    pub fn get_selector<F, Val>(
        &'ctx self,
        cx: Scope<'ctx>,
        selector_factory: F,
    ) -> &'ctx ReadSignal<Val>
    where
        F: Fn(&'ctx ReadSignal<T>) -> Val + 'ctx,
        Val: PartialEq,
    {
        create_selector(cx, move || selector_factory(self.signal))
    }
}

#[cfg(test)]
mod tests;
