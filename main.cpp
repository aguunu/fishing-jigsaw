#include <iostream>

#include "./include/jigsaw.h"
#include "./include/mcts.h"

int main() {
    /* Provide a random seed value to take random actions */
    srand((unsigned)time(nullptr));

    MCTS mcts = MCTS(100'000, 15, 1.0);

    /* Create Environment instance */
    Jigsaw game = Jigsaw();

    while (!game.has_finished()) {
        /* Perform a machine action. (choose new game figure) */
        std::vector<u8> legal_action = game.legal_actions();
        u8 random_index = rand() % legal_action.size();
        game.perform_action(legal_action.at(random_index));

        /* Render current game board */
        game.render();

        /* Search for the best action using MCTS algorithm */
        u8 action = mcts.search(game);

        /* Perform the best action found */
        game.perform_action(action);
    }

    return 0;
}