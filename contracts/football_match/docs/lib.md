# football_match

`football_match` is a small smart contract.\
football_match contains the logic for [SwipePM Dapp](https://github.com/FrankBevr/SwipePM).\
SwipePM is a betting dapp.\
It allows to bet on Manchester or Chelsea.\

## Overview (Userflow & Logic)
1. **Alice** instantiates the contract.
2. **Alice** becomes the admin.
3. **Bob** uses the frontend and bets on Manchester.
4. **Charlie** uses the frontend and isn't allowed to bet on Manchester.
5. **Charlie** bets on Chelsea.
6. The amount of the bet is controlled by the frontend.
7. **Alice**, the admin, calls `set_winner` with the winner value.\
    If Manchester won the value has to be 1\
    If Chelsea won the value has to be 2\
    The Default value is 0\
8. **Alice** sends value of 1 via `set_winner`.
9. Manchester won.
10. **Bob**, the manchester better, recieves all funds.
11. **Alice** calls `restart_match`, if a new manchester vs chelsea game is happening.\
     `restart_match` resets all values, expect the current admin.
12. **Alice** will be busy in the upcomming match.
13. **Alice** calls `change_admin` and sets **Django** as the new admin.
14. **Djange** is excited.
