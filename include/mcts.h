#ifndef MCTS_H
#define MCTS_H

#include <vector>

#include "../include/jigsaw.h"
#include "../include/node.h"
#include "../include/utils.h"

class MCTS {
   public:
    MCTS(u32 iters, u8 max_depth, f32 c);
    u8 search(const Jigsaw& initial_state);
    Node* select(Node* node, f32 c) const;
    Node* expand(Node* node) const;
    u8 rollout(Node* node) const;

   private:
    Node* root;
    u32 iters;
    u8 max_depth;
    f32 c;
};

#endif