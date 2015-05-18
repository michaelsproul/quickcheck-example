use rand;
use quickcheck::{Arbitrary, Gen, QuickCheck, StdGen};

use super::*;

const TEST_NUM_PLAYERS: usize = 10;
const TEST_SIZE: usize = 10_000;

impl Arbitrary for Transaction {
    fn arbitrary<G: Gen>(g: &mut G) -> Transaction {
        let reward = g.gen();
        let player = g.gen::<usize>() % TEST_NUM_PLAYERS;

        if reward {
            Reward(player)
        } else {
            Penalise(player)
        }
    }
}

impl Lottery {
    fn test_lottery(tickets_per_player: usize) -> Lottery {
        Lottery {
            total_tickets: tickets_per_player * TEST_NUM_PLAYERS,
            tickets: vec![tickets_per_player; TEST_NUM_PLAYERS]
        }
    }

    fn is_valid(&self) -> bool {
        // Check total.
        let sum = self.tickets.iter().fold(0, |acc, &item| acc + item);
        if sum != self.total_tickets {
            return false;
        }

        // Check bounds.
        for &num_tickets in self.tickets.iter() {
            if num_tickets < MIN_TICKETS || num_tickets > MAX_TICKETS {
                return false;
            }
        }

        true
    }
}

macro_rules! transaction_test {
    ($transaction_f:expr) => ({
        fn prop(transactions: Vec<Transaction>) -> bool {
            let mut lottery = Lottery::test_lottery(5);

            for t in transactions {
                $transaction_f(&mut lottery, t);

                if !lottery.is_valid() {
                    return false;
                }
            }

            true
        }

        let gen = StdGen::new(rand::thread_rng(), TEST_SIZE);
        let mut qc = QuickCheck::new().gen(gen);
        qc.quickcheck(prop as fn(Vec<Transaction>) -> bool);
    })
}

#[test]
fn good_transactions() {
    transaction_test!(Lottery::apply_transaction);
}

#[test]
fn bad_transaction_reward() {
    transaction_test!(|lottery: &mut Lottery, t| { lottery.bad_apply_transaction(t, false, true) });
}

#[test]
fn bad_transaction_penalise() {
    transaction_test!(|lottery: &mut Lottery, t| { lottery.bad_apply_transaction(t, true, false) });
}
