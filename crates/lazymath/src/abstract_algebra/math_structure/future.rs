use std::{rc::Rc, fmt::Debug, cell::RefCell};

struct _RawFutureValue<T>{
    pub(self) last_value: RefCell<Option<T>>,
    pub(self) constructor: Box<dyn Fn() -> T>,
}

impl<T: Debug> Debug for _RawFutureValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Future: {:?}", self.last_value))?;
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for _RawFutureValue<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.last_value == other.last_value
    }

    fn ne(&self, other: &Self) -> bool {
        return self.last_value != other.last_value
    }
}

impl<T: PartialEq> Eq for _RawFutureValue<T> { }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureValue<T>{
    _raw: Rc<_RawFutureValue<T>>,
}

impl<T: Clone> FutureValue<T> {

    pub fn new(constructor: Box<dyn Fn() -> T>) -> Self {
        Self {
            _raw: Rc::new(_RawFutureValue {
                last_value: RefCell::new(None),
                constructor,
            }),
        }
    }

    pub fn reify(&self) -> T {
        let new_value: T = (*self._raw.constructor)();
        self._raw.last_value.replace(Some(new_value));
        self._raw.last_value.borrow().as_ref().unwrap().clone()
    }

    pub fn take(&self) -> Option<T> {
        let old_val: Option<T> = self._raw.last_value.replace(None);
        old_val
    }

    pub fn get<'a>(&'a self) -> Option<T> {
        match self._raw.last_value.borrow().as_ref() {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::FutureValue;
    use zsft::Set;


    #[test]
    fn test_memory_address_preserved() {
        let future_val: FutureValue<Set> = FutureValue::new(Box::new(|| {
            Set::anonymous()
        }));

        let fv_clone = future_val.clone();

        let inner_future: FutureValue<Set> = FutureValue::new(Box::new(move || {
            fv_clone.clone().get().unwrap()
        }));

        let val = future_val.reify();

        assert_eq!(
            val,
            inner_future.clone().reify(),
        );

        assert_eq!(
            val,
            inner_future.clone().get().unwrap(),
        );
    }
}