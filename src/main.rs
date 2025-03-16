use rand::prelude::*;
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Card {
    Number,
    Jack = 1,
    Queen = 2,
    King = 3,
    Ace = 4,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Card::Number => '-',
                Card::Jack => 'J',
                Card::Queen => 'Q',
                Card::King => 'K',
                Card::Ace => 'A',
            }
        )
    }
}
type Deck = arrayvec::ArrayVec<Card, 52>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct GameStateNoMem {
    // false = p1, true = p2
    player: bool,
    // 0 is how the game starts off, waiting for a face card to be drawn
    number_cards_remaining_till_loss: u8,
    p1_deck: Deck,
    p2_deck: Deck,
    played_cards: Deck,
}
impl GameStateNoMem {
    fn switch_player(&mut self) {
        self.player = !self.player
    }
}

#[derive(Debug)]
struct GameStateMem {
    result: GameResult,
    inner: GameStateNoMem,
    visited_states: std::collections::HashSet<GameStateNoMem>,
}

impl GameStateMem {
    fn from_decks(p1_deck: Deck, p2_deck: Deck) -> Self {
        Self {
            result: GameResult {
                p1_start: p1_deck.clone(),
                p2_start: p2_deck.clone(),
                tricks: 0,
                cards: 0,
                is_infinite: false,
            },
            inner: GameStateNoMem {
                player: false,
                number_cards_remaining_till_loss: 0,
                p1_deck,
                p2_deck,
                played_cards: Deck::new(),
            },
            visited_states: HashSet::new(),
        }
    }
    fn gen_rand(rng: &mut impl Rng) -> Self {
        let mut whole_deck = Deck::new();
        for _ in 0..4 {
            whole_deck.push(Card::Ace);
            whole_deck.push(Card::King);
            whole_deck.push(Card::Queen);
            whole_deck.push(Card::Jack);
        }
        for _ in 0..36 {
            whole_deck.push(Card::Number);
        }
        whole_deck.shuffle(rng);
        let mut p1_deck = Deck::new();
        let mut p2_deck = Deck::new();
        for card in whole_deck {
            if p1_deck.len() == 26 {
                p2_deck.push(card);
            } else {
                p1_deck.push(card);
            }
        }
        Self::from_decks(p1_deck, p2_deck)
    }
    fn switch_player(&mut self) {
        self.inner.switch_player();
    }
    #[allow(clippy::collapsible_else_if)]
    fn step(&mut self) -> Option<&GameResult> {
        let played_card = match if self.inner.player {
            self.inner.p2_deck.pop()
        } else {
            self.inner.p1_deck.pop()
        } {
            Some(card) => card,
            None => {
                return Some(&self.result);
            }
        };
        self.result.cards += 1;
        self.inner.played_cards.push(played_card);

        // Highest index is the top of the deck, assuming deck is face down
        if self.inner.number_cards_remaining_till_loss == 0 {
            if played_card == Card::Number {
                self.switch_player();
            } else {
                self.switch_player();
                self.inner.number_cards_remaining_till_loss = played_card as u8;
            }
        } else {
            if played_card == Card::Number {
                self.inner.number_cards_remaining_till_loss -= 1;
                if self.inner.number_cards_remaining_till_loss == 0 {
                    self.result.tricks += 1;
                    if self.inner.player {
                        // If player 2 just lost the trick
                        self.inner.played_cards.reverse();
                        self.inner.played_cards.extend(self.inner.p1_deck.drain(..));
                        std::mem::swap(&mut self.inner.played_cards, &mut self.inner.p1_deck);
                    } else {
                        // If player 1 just lost the trick
                        self.inner.played_cards.reverse();
                        self.inner.played_cards.extend(self.inner.p2_deck.drain(..));
                        std::mem::swap(&mut self.inner.played_cards, &mut self.inner.p2_deck);
                    }
                    self.switch_player();
                }
            } else {
                self.switch_player();
                self.inner.number_cards_remaining_till_loss = played_card as u8;
            }
        }
        if !self.visited_states.insert(self.inner.clone()) {
            self.result.is_infinite = true;
            Some(&self.result)
        } else {
            None
        }
    }
    fn simulate(&mut self) -> GameResult {
        loop {
            if let Some(out) = self.step().cloned() {
                return out;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct GameResult {
    p1_start: Deck,
    p2_start: Deck,
    tricks: u64,
    cards: u64,
    is_infinite: bool,
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_infinite {
            write!(
                f,
                "∞ tricks, ∞ cards: "
            )?;
        } else {
            write!(f, "{} tricks, {} cards: ", self.tricks, self.cards)?;
        }
        write!(f, "1. ")?;
        for card in self.p1_start.iter().rev() {
            write!(f, "{card}")?;
        }
        write!(f, " 2. ")?;
        for card in self.p2_start.iter().rev() {
            write!(f, "{card}")?;
        }
        Ok(())
    }
}

fn main() {
    let options = vec![
        "Check a specific game",
        "Generate a random game",
        "Generate a bunch of random games and track the best",
    ];
    let thing_to_do = inquire::Select::new("What would you like to do?", options.clone())
        .prompt()
        .unwrap();

    let thing_to_do = options.iter().position(|s| *s == thing_to_do).unwrap();
    match thing_to_do {
        0 => check_specific_game(),
        1 => generate_random_game(),
        2 => try_find_best(),
        _ => unreachable!(),
    }
}

fn parse_input_deck(s: &str) -> Result<Deck, String> {
    let mut out = Deck::new();
    for c in s.trim().chars().rev() {
        match c.to_ascii_lowercase() {
            'a' => out.push(Card::Ace),
            'k' => out.push(Card::King),
            'q' => out.push(Card::Queen),
            'j' => out.push(Card::Jack),
            '-' => out.push(Card::Number),
            _ => return Err(format!("Unexpected character: \"{c}\"")),
        }
    }
    if out.len() != 26 {
        return Err(format!("Expected 26 cards. Got {}.", out.len()));
    }
    Ok(out)
}

fn input_player_deck(message: &str) -> Deck {
    #[derive(Clone, Copy)]
    struct Validator {}
    impl inquire::validator::StringValidator for Validator {
        fn validate(
            &self,
            input: &str,
        ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
            match parse_input_deck(input) {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            }
        }
    }
    parse_input_deck(
        &inquire::Text::new(message)
            .with_validator(Validator {})
            .prompt()
            .unwrap(),
    )
    .unwrap()
}

fn check_specific_game() {
    println!(
        "Deck format is the cards in the order they will be played. A/K/Q/J represents Ace, King, Queen, and Jack respectively. - represents a number card"
    );
    let p1_deck = input_player_deck("What is Player 1's deck?");
    let p2_deck = input_player_deck("What is Player 2's deck?");
    println!("{}", GameStateMem::from_decks(p1_deck, p2_deck).simulate());
}

fn generate_random_game() {
    println!("{}", GameStateMem::gen_rand(&mut rand::rng()).simulate());
}

fn try_find_best() {
    #[derive(Clone, Copy)]
    struct Validator {}
    impl inquire::validator::StringValidator for Validator {
        fn validate(
            &self,
            input: &str,
        ) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
            match input.parse::<usize>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            }
        }
    }
    let cores = num_cpus::get_physical();
    let cores_minus_1 = cores - 1;
    let cores_to_use = inquire::Text::new(&format!("How many threads would you like to spawn? Given that you have {cores} cores and one of them will be used to coordinate the others, I reccomend using {cores_minus_1} or {cores} threads: ")).with_validator(Validator{}).prompt().unwrap().parse::<usize>().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    for _ in 0..cores_to_use {
        let tx = tx.clone();
        std::thread::spawn(move || try_find_best_agent(tx));
    }
    let mut best_finite: Option<GameResult> = None;
    for msg in rx {
        if msg.is_infinite {
            println!("\nFound an infinite solution!\n{msg}");
        } else if msg.cards > best_finite.as_ref().map(|bf| bf.cards).unwrap_or(0) {
            println!("New best found!\n{msg}");
            best_finite = Some(msg);
        }
    }
}

fn try_find_best_agent(tx: std::sync::mpsc::Sender<GameResult>) {
    let mut rng = rand::rngs::SmallRng::from_os_rng();
    let mut best_finite: Option<GameResult> = None;
    loop {
        let gr = GameStateMem::gen_rand(&mut rng).simulate();
        if gr.is_infinite {
            tx.send(gr).unwrap();
        } else if gr.cards > best_finite.as_ref().map(|bf| bf.cards).unwrap_or(0) {
            tx.send(gr.clone()).unwrap();
            best_finite = Some(gr);
        }
    }
}
