use std::any::Any;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::{cell::RefCell, marker::PhantomData};

#[derive(Default)]
pub struct Runtime {
    signal_values: RefCell<Vec<Box<RefCell<dyn Any>>>>,
    running_effect: Cell<Option<EffectId>>,
    signal_subscribers: RefCell<HashMap<SignalId, HashSet<EffectId>>>,
    effects: RefCell<Vec<Box<dyn Fn()>>>,
}

impl Runtime {
    pub fn create_signal<T>(&'static self, value: T) -> Signal<T>
    where
        T: Clone + 'static,
    {
        self.signal_values
            .borrow_mut()
            .push(Box::new(RefCell::new(value)));

        let id = SignalId(self.signal_values.borrow().len() - 1);

        Signal {
            cx: self,
            id,
            ty: PhantomData,
        }
    }

    pub fn create_effect(&'static self, f: impl Fn() + 'static) {
        self.effects.borrow_mut().push(Box::new(f));
        let id = EffectId(self.effects.borrow().len() - 1);

        self.run_effect(id);
    }

    fn run_effect(&self, effect_id: EffectId) {
        let prev_running_effect = self.running_effect.take();
        self.running_effect.set(Some(effect_id));

        // run effect
        let effect = &self.effects.borrow()[effect_id.0];
        effect();

        self.running_effect.set(prev_running_effect);
    }
}

#[derive(Copy, Clone)]
pub struct Signal<T> {
    cx: &'static Runtime,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Signal<T>
where
    T: Clone + 'static,
{
    pub fn get(&self) -> T {
        let value = &self.cx.signal_values.borrow()[self.id.0];
        let value = value.borrow();
        let value = value.downcast_ref::<T>().unwrap();

        if let Some(running_effect) = self.cx.running_effect.get() {
            let mut subs = self.cx.signal_subscribers.borrow_mut();
            let subs = subs.entry(self.id).or_default();
            subs.insert(running_effect);
        }

        // return value
        value.clone()
    }

    pub fn set(&self, value: T) {
        // set value
        {
            let wrapper = &self.cx.signal_values.borrow()[self.id.0];
            let mut wrapper = wrapper.borrow_mut();
            let wrapper = wrapper.downcast_mut::<T>().unwrap();
            *wrapper = value;
        }

        // notify subscribers
        let subs = {
            let subs = self.cx.signal_subscribers.borrow();
            subs.get(&self.id).cloned()
        };

        if let Some(subs) = subs {
            for sub in subs {
                self.cx.run_effect(sub);
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct SignalId(usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct EffectId(usize);
