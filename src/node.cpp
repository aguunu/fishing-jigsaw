#include "../include/node.h"

#include <assert.h>
#include <math.h>

Node::Node(Jigsaw* state, Node* parent) {
    this->state = state;
    this->parent = parent;
    this->score = 0;
    this->visits = 0;
    this->unexplored_actions = state->legal_actions();
    this->is_terminal_state = state->has_finished();
    this->depth = parent ? parent->get_depth() + 1 : 0;
}

Node::~Node() {
    delete this->state;

    for (auto pair : this->children) {
        delete pair.second;
    }

    this->children.clear();
}

f32 Node::ucb(const f32 c) const {
    f32 exploitation = (f32)this->score / (f32)this->visits;

    if (!this->parent) {
        return exploitation;
    }

    f32 exploration = c * sqrtf32(2 * log(this->parent->visits) / this->visits);
    return exploitation + exploration;
}

bool Node::is_fully_expanded() const {
    return this->unexplored_actions.empty();
}

bool Node::is_terminal() const { return this->is_terminal_state; }

u8 Node::best_action(f32 c) const {
    assert(!this->children.empty());

    u8 best_action = this->children.begin()->first;
    f32 best_ucb = this->children.at(best_action)->ucb(c);

    for (auto const& it : this->children) {
        f32 ucb = it.second->ucb(c);
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

u8 Node::get_depth() const { return this->depth; }