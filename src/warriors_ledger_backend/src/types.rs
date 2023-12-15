
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

//struct to  store player profile
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub name: String,
    pub id: u64,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
    pub weapons: Vec<Weapon>,      //List of all weapons owned by player
    pub match_history: Vec<Match>, //List of all matches played by player
}

//struct to store weapon profile
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub id: u64,
    pub damage: u64,
    pub ammo: u64,
    pub range: u64,       //range of weapon in meters
    pub fire_rate: u64,   //number of bullets fired per second
    pub reload_time: u64, //time taken to reload in seconds
    pub accuracy: u64,    //accuracy of weapon in percentage
    pub price: u64,
    pub level: u64,
    pub rank: u64,
}

//struct to store match profile
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: u64,
    pub player_id: u64,
    pub weapon_id: u64,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
    pub time: u64, //time taken to complete match in minutes
    pub result: bool,
}

//struct to store leaderboard
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub id: u64,
    pub player_id: u64,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
}

//Implement Storable and BoundedStorable for PlayerProfile
impl Storable for PlayerProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PlayerProfile {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

//Implement Storable and BoundedStorable for Weapon
impl Storable for Weapon {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Weapon {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

//Implement Storable and BoundedStorable for Match

impl Storable for Match {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Match {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

//Implement Storable and BoundedStorable for Leaderboard
impl Storable for Leaderboard {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Leaderboard {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

//weapon profile payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct WeaponProfilePayload {
    pub name: String,
    pub damage: u64,
    pub ammo: u64,
    pub range: u64,       //range of weapon in meters
    pub fire_rate: u64,   //number of bullets fired per second
    pub reload_time: u64, //time taken to reload in seconds
    pub accuracy: u64,    //accuracy of weapon in percentage
    pub price: u64,
    pub level: u64,
    pub rank: u64,
}
//player profile payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct PlayerProfilePayload {
    pub name: String,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
}
//match profile payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct MatchProfilePayload {
    pub player_id: u64,
    pub weapon_id: u64,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
    pub time: u64, //time taken to complete match in minutes
    pub result: bool, // 
}

//leaderboard payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct LeaderboardPayload {
    pub player_id: u64,
    pub score: u64,
    pub level: u64,
    pub rank: u64,
}

// Error type for the service
#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum  Error {
    NotFound { msg: String },
    InvalidPlayerPayload{msg: String, payload: PlayerProfilePayload},
    InvalidWeaponPayload{msg: String, payload: WeaponProfilePayload},
    InvalidMatchPayload{msg: String, payload: MatchProfilePayload},
    InvalidLeaderboardPayload{msg: String, payload: LeaderboardPayload}
}