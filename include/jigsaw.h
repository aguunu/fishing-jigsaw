#ifndef JIGSAW_H
#define JIGSAW_H

#include <vector>

#include "../include/utils.h"

class Jigsaw {
   public:
    Jigsaw();
    Jigsaw(const Jigsaw& game);
    u8 get_round() const;
    bool has_finished() const;
    void set_figure_index(const u8 figure_index);
    u32 figure() const;
    void perform_action(const u8 action);
    bool is_legal(const u8 action) const;
    const std::vector<u8> legal_actions() const;
    u8 rollout_policy(u8 max_depth) const;
    void render() const;

   private:
    u32 board;
    u8 figure_index;
    u8 round;
    u8 agent;

    static const u8 ROWS;
    static const u8 COLS;
    static const u8 SKIP_ACTION;
    static const u8 INVALID_FIGURE_INDEX;
    static const std::vector<u8> ALL_FIGURES_INDEXES;
    static const u8 TOTAL_FIGURES;
    static const u8 TOTAL_ACTIONS;
    static const u32 ALL_FIGURES[];
};

#endif