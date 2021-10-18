
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.inner.is_empty() {
            let group_value = &self.inner[0];
            let mut count = 0;
            while count < self.inner.len() && &self.inner[count] == group_value {
                count += 1;
            }
            let group = self.inner.drain(0..count).collect();
            Some(group)
        } else {
            None
        }
        // let mut group = Vec::new();
        // let mut i = 0;
        // let mut j = 1;
        // group.push(&self.inner[0]);
        // while j < &self.inner.len() && self.inner[i] == self.inner[j] {
        //     group.push(&self.inner[j]);
        //     j += 1;
        // }
        // self.inner.remove();
        // Some(group)
    }


}

pub fn execute() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5]; // len = 2     i = 0 vec(0) = 5   j = 1  vec(1) = 5    j = 1 + 1 = 2 vec(2) = Segmentation fault
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}