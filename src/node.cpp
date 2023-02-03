#include "../include/node.h"

#include <assert.h>
#include <math.h>

Node::Node(Jigsaw* state, Node* parent) {
    this->state = state;
    this->parent = parent;
    this->score = 0;
    this->visits = 0;
    this->unexplored_actions = state->legal_actions();
}

Node::~Node() {
    delete this->state;

    for (auto pair : this->children) {
        delete pair.second;
    }

    this->children.clear();
}

f64 Node::ucb(const f64 c) const {
    f64 exploitation = (f64)this->score / (f64)this->visits;

    if (!this->parent) {
        return exploitation;
    }

    f64 exploration = c * sqrtf64(2 * log(this->parent->visits) / this->visits);
    return exploitation + exploration;
}

bool Node::is_fully_expanded() const {
    return this->unexplored_actions.empty();
}

u8 Node::best_action(f64 c) const {
    assert(!this->children.empty());

    u8 best_action = this->children.begin()->first;
    f64 best_ucb = this->children.at(best_action)->ucb(c);

    for (auto const& it : this->children) {
        f64 ucb = it.second->ucb(c);
        if (best_ucb < ucb) {
            best_action = it.first;
            best_ucb = ucb;
        }
    }

    return best_action;
}

void Node::backpropagation(const u8 score) {
    this->visits += 1;
    this->score += (u32)score;
    if (this->parent) {
        this->parent->backpropagation(score);
    }
}