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
    f32 ucb(const f32 c) const;
    bool is_fully_expanded() const;
    bool is_terminal() const;
    void backpropagation(const u8 score);
    u8 best_action(f32 c) const;
    u8 get_depth() const;
    std::unordered_map<u8, Node*> children;
    Jigsaw* state;
    std::vector<u8> unexplored_actions;
    
   private:
    u32 score;
    u32 visits;
    Node* parent;
    bool is_terminal_state;
    u8 depth;
};

#endif