#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Shoe {
    size: u8,
}

pub fn shoes_in_size(shoes: &[Shoe], size: u8) -> Vec<Shoe> {
    shoes
        .iter()
        .filter(|shoe| shoe.size == size)
        .copied()
        .collect()
}

pub fn shoes_in_size_consumes(shoes: Vec<Shoe>, size: u8) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == size)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn iterator_demo() {
        let v = [1, 2, 3];
        let mut v_iter = v.iter();
        assert_eq!(Some(&1), v_iter.next());
        assert_eq!(Some(&2), v_iter.next());
        assert_eq!(Some(&3), v_iter.next());
        assert_eq!(None, v_iter.next());
    }

    #[test]
    fn sum_iterator() {
        let v = [1, 2, 3];
        let v_iter = v.iter();
        let total: i32 = v_iter.sum();
        // let total2: i32 = v_iter.sum(); // Not allowed - v_iter was moved and consumed
        assert_eq!(total, 6);
    }

    #[test]
    fn mapped_iterator() {
        let v = [1, 2, 3];
        let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
        assert_eq!(v2, vec![2, 4, 6]);
    }

    #[test]
    fn shoe_filter() {
        let shoes = vec![
            Shoe { size: 8 },
            Shoe { size: 14 },
            Shoe { size: 14 },
            Shoe { size: 8 },
        ];
        let of_size_14 = shoes_in_size(&shoes, 14);
        assert_eq!(of_size_14, vec![Shoe { size: 14 }, Shoe { size: 14 }]);
        println!("{:?}", shoes);
        println!("{:?}", of_size_14);
    }
    
    #[test]
    fn shoe_filter_consumed() {
        let shoes = vec![
            Shoe { size: 8 },
            Shoe { size: 14 },
            Shoe { size: 14 },
            Shoe { size: 8 },
        ];
        let of_size_14 = shoes_in_size_consumes(shoes, 14);
        assert_eq!(of_size_14, vec![Shoe { size: 14 }, Shoe { size: 14 }]);
        // println!("{:?}", shoes); // Moved - consumed original
        println!("{:?}", of_size_14);
    }
}
