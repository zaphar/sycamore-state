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
use super::*;

pub enum Msg {
    UpdateOne(String),
    UpdateTwo(i32),
}

#[derive(Clone, PartialEq)]
pub struct FakeState {
    value_one: String,
    value_two: i32,
}

pub struct StateMachine();

impl Dispatcher<Msg, FakeState> for StateMachine {
    fn apply(&self, msg: Msg, original: &ReadSignal<FakeState>) -> FakeState {
        match msg {
            Msg::UpdateOne(val) => {
                let mut new_state = original.get().as_ref().clone();
                new_state.value_one = val;
                new_state
            }
            Msg::UpdateTwo(val) => {
                let mut new_state = original.get().as_ref().clone();
                new_state.value_two = val;
                new_state
            }
        }
    }
}

macro_rules! with_scope {
    ($cx:ident, $( $body:tt )* ) => {{
        use sycamore::prelude::create_scope_immediate;
        create_scope_immediate(|$cx| {
            $( $body )*
        });
    }};
}

#[test]
fn test_state_effect_flow() {
    with_scope! {cx,
        let state = FakeState {
            value_one: "foo".to_owned(),
            value_two: 0,
        };

        let reducer = Reducer::new(cx, state, StateMachine());

        create_child_scope(cx, |cx| {
            let form_val = create_signal(cx, reducer.signal().get_untracked().value_one.clone());

            reducer.bind(cx, || Msg::UpdateOne((*form_val.get()).clone()));

            form_val.set("bar".to_owned());

            assert_eq!(reducer.signal().get_untracked().value_one, "bar".to_owned());

            create_child_scope(cx, |cx| {
                let form_val = create_signal(cx, 0);

                reducer.bind(cx, || Msg::UpdateTwo(*form_val.get()));
                form_val.set(1);
                assert_eq!(reducer.signal().get_untracked().value_two, 1);
            });
        });
    };
}
