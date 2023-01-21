#[derive(Debug)]
pub struct CustomSet<T> {
    store: Vec<T>,
}

impl<T: PartialEq + Eq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.store.len() != other.store.len() {
            return false;
        }

        self.store.iter().all(|item| other.contains(item))
    }
}

impl<T: PartialEq + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut store = vec![];
        for item in input.iter() {
            if !store.contains(item) {
                store.push(item.to_owned());
            }
        }
        Self { store }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.store.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.store.push(element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for item in &self.store {
            if !other.contains(item) {
                return false;
            }
        }

        return true;
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for item in &self.store {
            if other.contains(item) {
                return false;
            }
        }

        return true;
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut set = Self::new(&[]);
        self.store.iter().for_each(|item| {
            if other.contains(item) {
                set.add(item.to_owned())
            }
        });
        set
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut set = Self::new(&[]);
        self.store.iter().for_each(|item| {
            if !other.contains(item) {
                set.add(item.to_owned());
            }
        });
        set
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut set = Self::new(&[]);
        self.store.iter().for_each(|item| set.add(item.to_owned()));
        other.store.iter().for_each(|item| set.add(item.to_owned()));
        set
    }
}
