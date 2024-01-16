#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Calculator {
    pub owner: Pubkey,
    pub display: i64,
}

impl<'info, 'entrypoint> Calculator {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedCalculator<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let display = account.display;

        Mutable::new(LoadedCalculator {
            __account__: account,
            __programs__: programs_map,
            owner,
            display,
        })
    }

    pub fn store(loaded: Mutable<LoadedCalculator>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let display = loaded.display;

        loaded.__account__.display = display;
    }
}

#[derive(Debug)]
pub struct LoadedCalculator<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Calculator>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub display: i64,
}

#[account]
#[derive(Debug)]
pub struct Market {
    pub owner: Pubkey,
    pub display: i64,
}

impl<'info, 'entrypoint> Market {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedMarket<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let display = account.display;

        Mutable::new(LoadedMarket {
            __account__: account,
            __programs__: programs_map,
            owner,
            display,
        })
    }

    pub fn store(loaded: Mutable<LoadedMarket>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let display = loaded.display;

        loaded.__account__.display = display;
    }
}

#[derive(Debug)]
pub struct LoadedMarket<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Market>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub display: i64,
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, Copy)]
pub enum Operation {
    BID,
    CLAIM,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::BID
    }
}

pub fn init_calculator_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut calculator: Empty<Mutable<LoadedCalculator<'info, '_>>>,
) -> () {
    let mut calculator = calculator.account.clone();

    assign!(calculator.borrow_mut().owner, owner.key());
}
