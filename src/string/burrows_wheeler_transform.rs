pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
    let len = input.len();

    let mut table = Vec::<String>::with_capacity(len);
    for i in 0..len {
        table.push(input[i..].to_owned() + &input[..i]);
    }
    table.sort_by_key(|a| a.to_lowercase());

    let mut encoded = String::new();
    let mut index: usize = 0;
    for (i, item) in table.iter().enumerate().take(len) {
        encoded.push(item.chars().last().unwrap());
        if item.eq(&input) {
            index = i;
        }
    }

    (encoded, index)
}

pub fn inv_burrows_wheeler_transform(input: (&str, usize)) -> String {
    let len = input.0.len();
    let mut table = Vec::<(usize, char)>::with_capacity(len);
    for i in 0..len {
        table.push((i, input.0.chars().nth(i).unwrap()));
    }

    table.sort_by(|a, b| a.1.cmp(&b.1));

    let mut decoded = String::new();
    let mut idx = input.1;
    for _ in 0..len {
        decoded.push(table[idx].1);
        idx = table[idx].0;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn burrows_wheeler_transform_to_inverse_helper(s: &str) -> bool {
        let transformed = burrows_wheeler_transform(s);
        let transformed = (transformed.0.as_ref(), transformed.1);
        let res = inv_burrows_wheeler_transform(transformed);

        s == &*res
    }

    #[test]
    //Ensure function stand-alone legitimacy
    fn stand_alone_function() {
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_string(), 1usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("CTRRAO", 1usize)),
            ("CARROT".to_string())
        );
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_string(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI", 11usize)),
            ("THEALGORITHMS".to_string())
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_string(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!", 0usize)),
            "!.!.!??.=::"
        );
    }
    #[test]
    fn basic_characters() {
        assert!(burrows_wheeler_transform_to_inverse_helper("CARROT"));
        assert!(burrows_wheeler_transform_to_inverse_helper("TOMATO"));
        assert!(burrows_wheeler_transform_to_inverse_helper("THISISATEST"));
        assert!(burrows_wheeler_transform_to_inverse_helper("THEALGORITHMS"));
        assert!(burrows_wheeler_transform_to_inverse_helper("RUST"));
    }

    #[test]
    fn special_characters() {
        assert!(burrows_wheeler_transform_to_inverse_helper("!.!.!??.=::"));
        assert!(burrows_wheeler_transform_to_inverse_helper(
            "!{}{}(((&&%%!??.=::"
        ));
        assert!(burrows_wheeler_transform_to_inverse_helper("//&$[]"));
    }

    #[test]
    fn empty() {
        assert!(burrows_wheeler_transform_to_inverse_helper(""));
    }
}
