use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};

fn main() {
    println!("Hello, world!");
}

fn generate_tickets(tickets: Vec<String>) -> Vec<(GenericArray<u8, U32>, String)> {
    return tickets
        .into_iter()
        .map(|s| (Sha256::digest(&s.as_bytes()), s))
        .collect::<Vec<(GenericArray<u8, U32>, String)>>();
}

fn get_hash(data: &[u8]) -> GenericArray<u8, U32> {
    return Sha256::digest(&data);
}

fn generate_chain(
    tickets: Vec<(GenericArray<u8, U32>, String)>,
) -> Vec<(GenericArray<u8, U32>, String)> {
    assert!(!tickets.is_empty());
    let capacity = tickets.len();
    let mut names = Vec::with_capacity(capacity);
    let mut hashes = Vec::with_capacity(capacity);

    tickets.into_iter().for_each(|(h, n)| {
        names.push(n);
        hashes.push(h);
    });

    let mut computed: Vec<GenericArray<u8, U32>> = Vec::with_capacity(capacity);
    let d = get_hash(format!("{}{:x}", names[0], hashes[0]).as_bytes());
    computed.push(d);

    let computed = hashes.into_iter().fold(computed, |mut acc, h| {
        let r = Sha256::digest(format!("{:x}{:x}", acc.last().unwrap(), h).as_bytes());
        acc.push(r);
        acc
    });

    computed.into_iter().zip(names.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tickets() {
        let tickets = vec![
            String::from("John"),
            String::from("Doe"),
            String::from("Jane"),
            String::from("Doe"),
        ];
        let tickets_and_digests = generate_tickets(tickets);
        println!("{:?}", tickets_and_digests);

        let chain = generate_chain(tickets_and_digests)
            .into_iter()
            .map(|(d, n)| format!("{} : {:x}", n, d))
            .collect::<Vec<String>>();
        println!("{:#?}", chain);
    }
}
