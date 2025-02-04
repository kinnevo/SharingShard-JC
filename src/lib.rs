use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, UnorderedMap};

/*
** Structures
*/

#[derive(BorshSerialize, BorshDeserialize)]
struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    areas: u8,
    reward: u128,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: UnorderedMap<AccountId, String>,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct User{
    name: String,
    discord: String,
    email: String,
    interests: u8,
    my_exp: Vec<u128>,
    pov_exp: Vec<u128>,
    date: i64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] 
pub struct Contract{
    users: LookupMap<AccountId, User>,
    experience: LookupMap<u128, Experience>,
    exp_by_area: LookupMap< u8, Vec<u128> >,
    n_exp: u128,
    fee: u128,
}

/*
** Functions
*/

/*
** Initialization
*/
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self{
            users: LookupMap::new(b"m"),
            experience: LookupMap::new(b"m"),
            exp_by_area: LookupMap::new(b"m"),
            n_exp: 0,
            fee: 10,
        }
    }
    pub fn new_user(&mut self, wallet: AccountId, n: String, disc: String, mail: String, interst: u8){
        //assert wallet
        self.users.insert(&wallet.clone(), &User{name: n, discord: disc, email: mail, interests: interst, my_exp: Vec::new(), pov_exp: Vec::new(), date: 0});
    }
    pub fn add_experience(&mut self, wallet: AccountId, exp_title: String, v_link: String, amount: u128, desc: String, t: u16, date: i64){
        //assert wallet
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: exp_title.clone(),
            owner: wallet.clone(),
            description: desc,
            url: v_link,
            reward: amount,
            moment: "".to_string(),
            time : t,
            pov: UnorderedMap::new(b"m"),
            areas: 0,
            exp_date: date
        });
        self.n_exp += 1;
    }
    pub fn get_reward(&mut self, wallet: AccountId, video_n: u128) -> u128{
        let exp = (self.experience.get(&video_n.clone())).unwrap();
        let reward = exp.reward;
        if exp.owner == wallet {
            reward
        }
        else{
            0
        }
    }
}

fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "pepe.near".parse().unwrap();
    contract.new_user(id.clone(), "pepe".to_string(), "pepediscord".to_string(), "pepemail".to_string(), 8);
    contract.add_experience(id.clone(), "experience 1".to_string(), "https://video.de/pepe".to_string(), 12, "descripcion video pepe".to_string(), 1200, 3000);
    let rew = contract.get_reward(id.clone(), 0);
    println!("{:?}", rew);
}