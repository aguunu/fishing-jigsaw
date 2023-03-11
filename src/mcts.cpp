#include "../include/mcts.h"

#include <assert.h>

#include <iostream>

MCTS::MCTS(u32 iters, u8 max_depth, f32 c) {
    srand((unsigned)time(nullptr));

    this->iters = iters;
    this->max_depth = max_depth;
    this->c = c;
}

u8 MCTS::search(const Jigsaw& initial_state) {
    /* Create root state from initial state */
    this->root = new Node(new Jigsaw(initial_state));

    for (u32 _i = 0; _i < this->iters; _i++) {
        /* Selection phase */
        Node* node = this->select(this->root, this->c);

        if (node->is_terminal()) {
            u8 score = node->ucb(0.0);
            node->backpropagation(score);
            continue;
        }

        /* Expansion phase */
        node = this->expand(node);

        /* Simulation phase */
        u8 score = this->rollout(node);

        /* Backpropagation phase */
        node->backpropagation(score);
    }

    u8 best_action = this->root->best_action(0.0);

    delete this->root;

    return best_action;
}

Node* MCTS::select(Node* node, f32 c) const {
    if (!node->is_fully_expanded() || node->is_terminal()) {
        return node;
    }

    Node* best_child = node->children.at(node->best_action(c));

    assert(best_child != nullptr);

    return this->select(best_child, c);
}

Node* MCTS::expand(Node* node) const {
    assert(node->unexplored_actions.size() > 0);

    u8 action = node->unexplored_actions.back();
    node->unexplored_actions.pop_back();

    assert(node->children.count(action) == 0);

    Jigsaw* new_state = new Jigsaw(*node->state);
    new_state->perform_action(action);

    Node* new_child = new Node(new_state, node);
    node->children.insert({action, new_child});

    return new_child;
}

u8 MCTS::rollout(Node* node) const {
    u8 max_depth = this->max_depth - node->get_depth();
    return node->state->rollout_policy(max_depth);
}