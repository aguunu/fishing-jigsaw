#include "../include/jigsaw.h"

#include <cstdlib>
#include <iostream>

const u8 Jigsaw::ROWS = 4;
const u8 Jigsaw::COLS = 6;
const u8 Jigsaw::SKIP_ACTION = 24;
const u8 Jigsaw::INVALID_FIGURE_INDEX = TOTAL_FIGURES + 1;
const u8 Jigsaw::TOTAL_FIGURES = 6;
const u8 Jigsaw::TOTAL_ACTIONS = 24 + 1;
const u32 Jigsaw::ALL_FIGURES[] = {
    0b100000000000000000000000, 0b100000100000100000000000,
    0b100000110000000000000000, 0b110000010000000000000000,
    0b110000110000000000000000, 0b110000011000000000000000,
};
const std::vector<u8> Jigsaw::ALL_FIGURES_INDEXES = {0, 1, 2, 3, 4, 5};

enum Player { MACHINE, HUMAN };

Jigsaw::Jigsaw() {
    this->board = 0;
    this->round = 0;
    this->depth = 0;
    this->agent = MACHINE;
    this->figure_index = INVALID_FIGURE_INDEX;
}

Jigsaw::Jigsaw(const Jigsaw& game) {
    this->board = game.board;
    this->figure_index = game.figure_index;
    this->round = game.round;
    this->depth = game.depth;
    this->agent = game.agent;
}

u8 Jigsaw::get_round() const { return this->round; }

u8 Jigsaw::get_depth() const { return this->depth; }

void Jigsaw::set_figure_index(const u8 figure_index) {
    this->figure_index = figure_index;
}

u32 Jigsaw::figure() const { return ALL_FIGURES[this->figure_index]; }

bool Jigsaw::has_finished() const { return this->board == 0xFFFFFF; }

void Jigsaw::perform_action(const u8 action) {
    if (this->agent == MACHINE) {
        this->figure_index = action;
    } else {  // this->agent == HUMAN
        if (action != SKIP_ACTION) {
            this->board |= this->figure() >> action;
        }

        this->round += 1;

        this->figure_index = INVALID_FIGURE_INDEX;
    }

    this->agent = (this->agent == MACHINE) ? HUMAN : MACHINE;
    this->depth += 1;
}

bool Jigsaw::is_legal(const u8 action) const {
    if (action == SKIP_ACTION) {
        return true;
    }

    u8 x_offset = action % this->COLS;

    if (this->board & (this->figure() >> action)) {
        return false;
    }

    if (((this->figure() >> action) << action) != this->figure()) {
        return false;
    }

    u8 column_mask = (u8) ~(0xFF << this->COLS);

    for (u8 i = 0; i < this->ROWS; i++) {
        u8 figure_row = (this->figure() >> (this->COLS * i)) & column_mask;

        if (((figure_row >> x_offset) << x_offset) != figure_row) {
            return false;
        }
    }

    return true;
}

const std::vector<u8> Jigsaw::legal_actions() const {
    if (this->agent == MACHINE) {
        return ALL_FIGURES_INDEXES;
    }

    std::vector<u8> legal_actions;
    for (u8 action = 0; action < TOTAL_ACTIONS; action++) {
        if (this->is_legal(action)) {
            legal_actions.push_back(action);
        }
    }
    return legal_actions;
}

u8 Jigsaw::rollout_policy(u8 max_depth) const {
    Jigsaw game = Jigsaw(*this);

    while (!game.has_finished()) {
        std::vector<u8> legal_actions = game.legal_actions();
        u8 action_index = rand() % legal_actions.size();
        u8 action = legal_actions.at(action_index);
        game.perform_action(action);

        if (game.depth > max_depth) {
            return 0;
        }
    }

    return 1;
}

void Jigsaw::render() const {
    std::cout << "===============" << static_cast<unsigned>(this->round)
              << "===============\n";

    u32 board_mask = 1 << (this->ROWS * this->COLS - 1);
    u32 figure_mask = 1 << (this->ROWS * this->COLS - 1);

    for (u8 i = 0; i < this->ROWS; i++) {
        for (u8 j = 0; j < this->COLS; j++) {
            if (this->board & board_mask) {
                std::cout << "🟩";
            } else if (this->is_legal(i * this->COLS + j)) {
                std::cout << "🟦";
            } else {
                std::cout << "⬛";
            }
            board_mask /= 2;
        }

        std::cout << " ";

        for (u8 j = 0; j < this->COLS; j++) {
            if (this->figure() & figure_mask) {
                std::cout << "🟥";
            } else {
                std::cout << "  ";
            }
            figure_mask /= 2;
        }

        std::cout << "\n";
    }
}