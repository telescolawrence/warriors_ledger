# Warriors Ledger ->Game Service 

Welcome to your new warriors_ledger project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

The Warriors Ledger(Game Service), made in Rust, is the main system that manages important parts in a game. It works like the essential parts of famous games like Call of Duty, handling various functions in the game world.


To learn more before you start working with warriors_ledger, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)


## Usage
The service offers functions for creating, updating, retrieving, and deleting data related to player profiles, weapons, matches, and leaderboards. It uses `StableBTreeMap` for storage and manages memory using `MemoryManager`
If you want to start working on your project right away, you might want to try the following commands:

### Player Profile
  - `create_player_profile`: Creates a new player profile with necessary details such as name, score, level, and rank.
  - `update_player_profile`: Updates an existing player's details based on the provided ID.
  - `get_player_profile`: Retrieves a player's profile by ID.
  - `get_all_players_profile`: Retrieves all player profiles stored in the service.
  - `delete_player_profile`: Deletes a player profile by ID.

### Weeapon 

- `create_weapon`: Creates a new weapon profile with specifications like name, damage, ammo, etc.
- `update_weapon_profile`: Updates an existing weapon's details based on the provided ID.
- ``: Retrieves a weapon profile by ID.
- `get_all_weapons`: Retrieves all weapon profiles stored in the service.
- `rank_weapons_by_damage`: Ranks weapons by damage in descending order.
- `delete_weapon`: Deletes a weapon by ID.

### Match

  - `create_match:` Records a match's details including score, level, time taken, etc.
  - `update_match:` Updates match details based on the provided ID.
  - `get_match:` Retrieves match details by ID.
  - `get_all_matches:` Retrieves all recorded matches.
  - `delete_match:` Deletes a match by ID.


### Leaderboard

- `create_leaderboard:` Records leaderboard details including player ID, score, level, and rank.
- `update_leaderboard:` Updates leaderboard details based on the provided ID.
- `get_leaderboard:` Retrieves leaderboard details by ID.
- `get_all_leaderboards:` Retrieves all leaderboards stored in the service.
- `sort_leaderboard_by_score:` Sorts leaderboards by score in descending order.
- `delete_leaderboard:` Deletes a leaderboard by ID.

### Adding Weapons to Player Profile 
- `add_weapon_to_player_profile:` Adds a weapon to a player's profile by associating the weapon ID with the player's ID.

### Addding  Match to player Profile 

- `add_match_to_player_profile:` Associates a match with a player's profile by linking the match ID with the player's ID.
```bash
cd warriors_ledger/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
