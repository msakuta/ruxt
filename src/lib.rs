use std::{cell::RefCell, rc::Rc};

pub trait Observable<U> {
    // fn subscribers(&self) -> &[Rc<dyn Fn()>];
    fn subscribe(&self, f: Rc<dyn Fn(U)>);
}

pub struct IterObservableInt<T, U> {
    iter: RefCell<T>,
    subscribers: RefCell<Vec<Rc<dyn Fn(U)>>>,
}

#[derive(Clone)]
pub struct IterObservable<T, U>(Rc<IterObservableInt<T, U>>);

impl<U: Clone + 'static, T: Iterator<Item = U> + 'static> IterObservable<T, U> {
    pub fn from_iter(iter: T) -> Self {
        Self(Rc::new(IterObservableInt {
            iter: RefCell::new(iter),
            subscribers: RefCell::new(vec![]),
        }))
    }

    pub fn filter(self, f: impl Fn(&U) -> bool + 'static) -> Rc<FilterObservable<U>> {
        let ret = Rc::new(FilterObservable::new(self.0.clone(), Box::new(f)));
        let sub = ret.clone();
        self.0
            .subscribers
            .borrow_mut()
            .push(Rc::new(move |val| sub.call(val)));
        ret
    }

    pub fn run(&self) {
        for val in &mut *self.0.iter.borrow_mut() {
            for sub in self.0.subscribers.borrow().iter() {
                sub(val.clone());
            }
        }
    }
}

impl<T, U> Observable<U> for IterObservableInt<T, U> {
    // fn subscribers(&self) -> &[Rc<dyn Fn()>] {
    //     &self.subscribers
    // }

    fn subscribe(&self, f: Rc<dyn Fn(U)>) {
        self.subscribers.borrow_mut().push(f);
    }
}

pub struct FilterObservable<U> {
    // observable: Rc<dyn Observable<U>>,
    filter: Box<dyn Fn(&U) -> bool>,
    subscribers: RefCell<Vec<Rc<dyn Fn(U)>>>,
}

impl<U: Clone> FilterObservable<U> {
    fn new(_observable: Rc<dyn Observable<U>>, filter: Box<dyn Fn(&U) -> bool>) -> Self {
        Self {
            // observable,
            filter,
            subscribers: RefCell::new(vec![]),
        }
    }

    fn call(&self, val: U) {
        if (self.filter)(&val) {
            for sub in self.subscribers.borrow().iter() {
                sub(val.clone());
            }
        }
    }
}

impl<U> Observable<U> for FilterObservable<U> {
    // fn subscribers(&self) -> &[Rc<dyn Fn()>] {
    //     static dummy: Vec<Rc<dyn Fn()>> = vec![];
    //     &dummy
    // }
    fn subscribe(&self, f: Rc<dyn Fn(U)>) {
        self.subscribers.borrow_mut().push(f);
    }
}
