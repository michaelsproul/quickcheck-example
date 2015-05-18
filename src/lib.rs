#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod test;

pub use self::Transaction::*;

/// The minimum number of tickets assigned to any player.
pub const MIN_TICKETS: usize = 1;

/// The maximum number of tickets assigned to any player.
pub const MAX_TICKETS: usize = 50;

pub struct Lottery {
    /// The total number of tickets. Equal to sum(tickets).
    pub total_tickets: usize,
    /// The number of tickets each player/process has, such that player i's
    /// ticket count is tickets[i].
    pub tickets: Vec<usize>
}

#[derive(Clone, Debug)]
pub enum Transaction {
    /// Increase the given player's ticket count by 1.
    Reward(usize),
    /// Decrease the given player's ticket count by 1.
    Penalise(usize)
}

impl Lottery {
    pub fn apply_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Reward(p) => {
                if self.tickets[p] < MAX_TICKETS {
                    self.tickets[p] += 1;
                    self.total_tickets += 1;
                }
            }
            Penalise(p) => {
                if self.tickets[p] > MIN_TICKETS {
                    self.tickets[p] -= 1;
                    self.total_tickets -= 1;
                }
            }
        }
    }

    pub fn bad_apply_transaction(
        &mut self,
        transaction: Transaction,
        check_reward: bool,
        check_penalise: bool) {

        match transaction {
            Reward(p) => {
                if !check_reward || self.tickets[p] < MAX_TICKETS {
                    self.tickets[p] += 1;
                    self.total_tickets += 1;
                }
            }
            Penalise(p) => {
                if !check_penalise || self.tickets[p] > MIN_TICKETS {
                    self.tickets[p] -= 1;
                    self.total_tickets -= 1;
                }
            }
        }
    }
}
