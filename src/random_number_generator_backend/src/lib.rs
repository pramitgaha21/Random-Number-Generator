use std::{cell::RefCell, time::Duration, ops::RangeFull};

use candid::Principal;
use getrandom::register_custom_getrandom;
use ic_cdk::update;
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use ic_cdk::init;

thread_local! {
    pub static RNG: RefCell<Option<StdRng>> = RefCell::new(None);
}

async fn set_rand() {
    let (seed,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
        .await
        .unwrap();
    RNG.with(|rng| {
        *rng.borrow_mut() = Some(StdRng::from_seed(seed));
        ic_cdk::println!("rng: {:?}", *rng.borrow());
    });
}

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(buf));
    Ok(())
}

pub fn init_ic_rand(){
    ic_cdk_timers::set_timer(Duration::from_secs(0), || ic_cdk::spawn(set_rand()));
    register_custom_getrandom!(custom_getrandom);
}

#[init]
fn init(){
  init_ic_rand();
}

#[update]
pub fn generate_random_number() -> i32{
    RNG.with_borrow_mut(|r| r.as_mut().unwrap().gen())
}

#[update]
pub fn generate_random_number_between_ranges(from: i32, to: i32) -> i32{
    if to < from{
        ic_cdk::trap("To is smaller than From")
    }
    RNG.with_borrow_mut(|r| r.as_mut().unwrap().gen_range(from..to))
}