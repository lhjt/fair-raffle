use beacon::get_beacon;
use env_logger::Env;
use log::info;
use sha2::{
    digest::generic_array::{typenum::U32, GenericArray},
    Digest, Sha256,
};

mod beacon;

fn main() {
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "debug"));
}

fn get_hash(data: &[u8]) -> GenericArray<u8, U32> {
    return Sha256::digest(&data);
}

fn generate_tickets(tickets: Vec<String>) -> Vec<(GenericArray<u8, U32>, String)> {
    info!("Generating tickets");
    return tickets
        .into_iter()
        .enumerate()
        .map(|(i, s)| (get_hash(format!("{}{}", &s, i).as_bytes()), s))
        .collect::<Vec<(GenericArray<u8, U32>, String)>>();
}

fn generate_chain(
    tickets: Vec<(GenericArray<u8, U32>, String)>,
) -> Vec<(GenericArray<u8, U32>, String)> {
    assert!(!tickets.is_empty());
    let capacity = tickets.len();
    info!("Generating chain for `{}` tickets", capacity);
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
        let r = get_hash(format!("{:x}{:x}", acc.last().unwrap(), h).as_bytes());
        acc.push(r);
        acc
    });

    computed.into_iter().zip(names.into_iter()).collect()
}

fn draw(data: Vec<(GenericArray<u8, U32>, String)>) -> Vec<(GenericArray<u8, U32>, String)> {
    let p = get_beacon();
    let cap = data.len();
    data.into_iter()
        .fold(Vec::with_capacity(cap), |mut acc, (d, n)| {
            let d = get_hash(format!("{:x}{}", d, p).as_bytes());
            acc.push((d, n));
            acc
        })
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
        println!("Tickets (aka hashed names + index):");
        println!(
            "{:#?}",
            tickets_and_digests
                .iter()
                .map(|(d, n)| format!("{} : {:x}", n, d))
                .collect::<Vec<String>>()
        );

        let chain = generate_chain(tickets_and_digests);
        println!("Ticket chain:");
        println!(
            "{:#?}",
            chain
                .iter()
                .map(|(d, n)| format!("{} : {:x}", n, d))
                .collect::<Vec<String>>()
        );

        let mut drawed = draw(chain);
        drawed.sort_by_key(|(d, _)| format!("{:x}", d));
        println!("Drawed chain:");
        println!(
            "{:#?}",
            drawed
                .iter()
                .map(|(d, n)| format!("{} : {:x}", n, d))
                .collect::<Vec<String>>()
        );
    }
}
