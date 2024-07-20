fn main() {
    map_iterator();
}

fn map_iterator() {
    let v1 = vec![1, 2, 3];
    let mapped_vec: Vec<_> = v1.iter().map(|x| x * 3).collect();
    println!("{:?}", mapped_vec);
    let added_vec: i32 = v1.iter().map(|x| x * 3).sum();
    println!("{:?}", added_vec)
}

fn sum_iterator() {
    let v1 = vec![1, 2, 3];
    let added_vec: i32 = v1.iter().sum();
    println!("{:?}", added_vec)
}
    
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
