The `GameData` struct stores values related to a football_match.

It has the following fields:
- `winning_team`: an integer representing the winning team.
- `admin`: an integer representing the current admin.
- `particpant_chelsea`: an AccountId representing the player, who bets on chelse.
- `particpant_manchester`: an AccountId representing the player, who bets on manchester.
- `particpant_chelsea_is_set`: a boolean representing, if particpant_chelsea is already set.
- `particpant_manchester_is_set`: a boolean representing, if particpant_chelsea is already set.

# Examples

```
use football_match::impls::football_match::football_match::GameData;
use ink::primitives::AccountId;

let game = GameData {
    winning_team: 0,
    admin: AccountId::from([0xFF as u8; 32]),
    particpant_chelsea: AccountId::from([0xFF as u8; 32]),
    particpant_manchester: AccountId::from([0xFF as u8; 32]),
    particpant_chelsea_is_set: false,
    particpant_manchester_is_set: false,
};

assert!(game.winning_team == 0u8);
assert!(game.particpant_chelsea_is_set == false);
assert!(game.particpant_manchester == AccountId::from([0xFF as u8; 32]))
```
