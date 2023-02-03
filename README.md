# 🦈 Metin2 Fishing Jigsaw
## About The Project
This project aims to create a software capable of compute a *promising* solution in an acceptable amount of time in order to maximize the rewards for a given state of the *Fishing Jigsaw* game.

Fishing Jigsaw is a deterministic problem which is difficult/impossible to compute its best solution since in order to find it, the software would have to compute all possible states of the current board *which is very expensive*. <br>

##  Fishing Jigsaw
- The aim of the game is to fill all the spaces of the board.

<div align="center">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216485215-8c295180-79cb-40d2-9acb-e6f6a95eefa0.png">
</div>

- You will receive a reward depending on the numbers of attempts you make in order to fill the board, *the less amount of attempts you make the better reward you will get*.

<div align="center">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482405-c386403a-5bfc-429f-b9df-4cff9620fe79.png">
</div>

- There are 6 types of figures you can get while playing the game, you will get one of them randomly each round.

<div align="center">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482399-91f0cb97-8adc-464f-9c4b-f3640e1d18d3.png">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482401-839f1e99-791f-46fa-9c58-6b84e1d087a6.png">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482402-68251aa7-7a5e-488d-97e8-c088c86ae211.png">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482403-a1aa4bc3-04e4-473e-8376-8d039d847fbd.png">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482408-c9092625-5b6d-4b20-a86d-70450abb719c.png">
    <image style="padding: 10px" src="https://user-images.githubusercontent.com/85197622/216482411-224666be-cfbb-4b96-8863-ce0bc0046117.png">
</div>

- Every round you will be able to put the current figure in a valid position or skip it.

## Monte Carlo Tree Search
[Monte Carlo tree search]("https://en.wikipedia.org/wiki/Hobbit#Lifestyle") is a heuristic search algorithm that can be useful in board games to solve their game tree. This algorithm expands the tree based on random samplings of the search space. Its application is based on *rollouts*. In each rollout, the game is played until a *terminal state* by selecting random actions. The final score of each rollout is then used to weight the nodes in the tree so that better nodes are more likely to be chosen in future rollouts.

## Contributing
If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks!

1. Fork the Project
2. Create your Feature Branch `git checkout -b feature/AmazingFeature`
3. Commit your Changes `git commit -m 'Add some AmazingFeature`
4. Push to the Branch `git push origin feature/AmazingFeature`
5. Open a Pull Request

## Contact
Discord - `Agus#1882` 💖
