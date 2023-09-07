<div align="center">
<img src="./Logo.jpg" alt="logo" width="80" height="80" />
</div>

<h3 align="center">SwipePM</h3>
  <p align="center">
    A social competition Sports Prediction App with Zeitgeist
    <br />
    <a href="https://www.figma.com/file/V1W0wKDqXxGYrEH5TFfzJL/SwipePM?type=design&node-id=0%3A1&mode=design&t=cq5XPPpWLxi543Po-1" name="Figma">Figma</a>
    ·
    <a href="https://youtu.be/pBcYVFuj9M0">Youtube</a>
    ·
    <a href="https://github.com/FrankBevr/SwipePM">Code</a>
  </p>
</div>

> :exclamation: see state @ submission deadline use `git checkout 5327dbe`

### Screenshot

<img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExOXhmcTFjY2ZhZzVvdTdheHhvZjVhOGVqcG05Zms3MXJ2a2Vvb25zOSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/QpJtL7s53HVVn11yBi/giphy.gif" alt="logo" />

### Summary

Sportsgeist, a social competition Sports Prediction App with Zeitgeist.  
SwipePM is a MVP of Sportsgeist.

### Quickstart

1. `npm install` in `./frontend` and `cargo contract build` in `./contracts/football_match`
2. `docker run -it -v ~/SwipePM/:/ink -p 9944:9944 -p 5173:5173 frankbevr/swipepm-quickstart`

### Team

| Name          | Discord      | Telegram      | E-mail                 |
| :------------ | :----------- | :------------ | :--------------------- |
| Patrik Bauer  | daredevil3x7 | @daredevil3x7 | `cryyptop@gmail.com`   |
| Frank Dierolf | frankbevr    | @frankbevr    | `frank_dierolf@web.de` |
| Tom           | -            | @morkeltry    | -                      |

<details>
<summary>More</summary>

### Description

Sportsgeist is a social competition sports betting application.  
Sportsgeist founder is Patrik.  
A minimal valuable product is SwipePM.  
SwipePM allows you to bet on Team A or on Team B.  
An admin declares the winner.  
The winner get all the funds.  
SwipePM utilizes ink!.  
Zeitgeist includes the smart contract pallet and allows custom Betting Logic via ink.  
We created a Custom Betting Logic and built a frontend on top of it.

### Track - ink! Smart Contracts

**Problem:**  
Sportbetting is huge.  
Centralised Entities are the current leaders.  
Sportsbetting has no huge social component to it.  
Fantasy football has a social component to it.  
Sportsgeist merges these Problem into a solution.

**Solution:**  
A is a social competition sports betting application called Sportsgeist.  
Start Small and check the complication and possibilties.  
Swipe PM is a MVP of Sportgeist.

### Challenge - Zeitgeist

The initial Idea was use chain extenstion to call `extrinsics::predictionMarkets::createMarket` in our contract.  
We communicated with Zeitgeist Team to make it happen.  
We encountered issues and couldn't solve it.  
Its still in the pipeline to make one successfull chainextension call.

### Future Plans

In the initial stages, we aim to introduce **Sportsgeist** as an interactive gaming application to familiarize users with its features and promote engagement.
Users will have the opportunity to win unique non-fungible tokens **(NFTs)** as part of this interactive experience, adding an exciting element of digital asset ownership.

As we continue to grow and build a **robust user base**, we will progressively **introduce sports betting features**.
We will strategically roll these out in **markets with strong user engagement** to ensure a seamless transition and capitalize on the existing momentum.
This phased approach will allow us to fine-tune our offerings in response to user feedback, ensuring a tailored and responsive betting experience.

### Tech Stack

| Backend        | Frontend                                  |
| :------------- | :---------------------------------------- |
| **Rust**, ink! | **Typescript**, Vue, UnoCSS, polkadot/api |

### What happend?

- [x] Chunk Sportsgeist Idea down to SwipePM
- [x] Create a Technical Design Sheet to chunk it in scope
- [x] Create Figma Design to scratch it out
- [x] Create ink contract.
- [x] Create Frontend
- [x] Connect Frontend with Contract.
- [x] Reevaluate
- [x] Create Figma Prototype for Sportsduell (~advanced SwipePM)
- [x] Improve ink Contract with documentation and added logic
- [x] Improve Frontend
- [x] Create DockerImage for one command setup
- [x] Make it nicey
- [x] Organise Trip to Seoul

</details>
