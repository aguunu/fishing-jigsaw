#ifndef NODE_H
#define NODE_H

#include <unordered_map>
#include <vector>

#include "../include/jigsaw.h"
#include "../include/utils.h"

class Node {
   public:
    Node(Jigsaw* state, Node* parent = nullptr);
    ~Node();
    f64 ucb(const f64 c) const;
    bool is_fully_expanded() const;
    void backpropagation(const u8 score);
    std::unordered_map<u8, Node*> children;
    Jigsaw* state;
    std::vector<u8> unexplored_actions;
    u8 best_action(f64 c) const;

   private:
    u32 score;
    u32 visits;
    Node* parent;
};

#endif