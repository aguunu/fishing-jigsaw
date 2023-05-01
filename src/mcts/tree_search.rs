use super::environment::Environment;
use super::node::Node;
use super::utils::Index;
use rand::Rng;

pub struct Tree<T: Environment + Clone> {
    nodes: Vec<Node<T::Action>>,
    root_state: T,
    root_index: Index,
    config: Config,
}

#[derive(Clone)]
pub struct Stats<T: Environment> {
    pub iters: u32,
    pub actions: Vec<(T::Action, u32)>,
}

impl<T: Environment> Stats<T> {
    pub fn best_action(&self) -> Option<T::Action> {
        self.actions
            .iter()
            .max_by_key(|&(_, visits)| visits)
            .and_then(|&(action, _)| Some(action))
    }
}

#[derive(Clone)]
pub struct Config {
    pub max_iters: u32,
    pub max_depth: u32,
    pub c: f32,
    pub callback_interval: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_iters: 300_000,
            max_depth: 10,
            c: f32::sqrt(2.0),
            callback_interval: 1000,
        }
    }
}

impl<T: Environment + Clone> Tree<T> {
    pub fn compute<F: FnMut(Stats<T>)>(&mut self, mut callback: F) {
        for iter in 1..=self.config.max_iters {
            self.try_callback_trigger(iter, &mut callback);

            let mut state = self.root_state.clone();

            let mut state_depth = 0;
            let mut index =
                self.select(self.root_index, self.config.c, &mut state, &mut state_depth);

            if state.has_finished() || state_depth > self.config.max_depth {
                self.backpropagate(state.eval(), 1, index);
                continue;
            }

            index = self.expand(index, &mut state);

            let rollout_depth = self.config.max_depth - state_depth;
            let score = self.simulation(state, rollout_depth);

            self.backpropagate(score, 1, index);
        }
    }

    pub fn new(state: T, config: Config) -> Self {
        let mut result = Tree {
            nodes: Vec::with_capacity(config.max_iters as usize),
            root_state: state,
            root_index: 0,
            config,
        };

        // Create root node.
        result.create_node();

        result
    }

    fn try_callback_trigger(&self, iter: u32, callback: &mut dyn FnMut(Stats<T>)) {
        if iter % self.config.callback_interval == 0 || iter == self.config.max_iters {
            let actions: Vec<(T::Action, u32)> = self.nodes[self.root_index]
                .children
                .iter()
                .map(|(action, index)| (*action, self.nodes[*index].visits))
                .collect();

            callback(Stats {
                iters: iter,
                actions,
            });
        }
    }

    fn create_node(&mut self) -> Index {
        let index = self.nodes.len();

        self.nodes.push(Node::new());

        return index;
    }

    fn select(&mut self, index: Index, c: f32, state: &mut T, state_depth: &mut u32) -> Index {
        // Check if state is terminal.
        if state.has_finished() {
            return index;
        }

        let legal_actions = state.legal_actions();

        let node = &self.nodes[index];

        let fully_expanded = legal_actions
            .iter()
            .all(|action| node.children.contains_key(action));

        // Check if state is not fully expanded.
        if !fully_expanded {
            return index;
        }

        // Select next state (unknown) by taking best action.
        let &action = legal_actions
            .iter()
            .max_by(|&x, &y| {
                let x_index = node.children[x];
                let y_index = node.children[y];
                self.nodes[x_index]
                    .ucb(c, node.visits)
                    .total_cmp(&self.nodes[y_index].ucb(c, node.visits))
            })
            .unwrap();

        // Perform selected action.
        state.perform_action(action);

        *state_depth += 1;

        return self.select(node.children[&action], c, state, state_depth);
    }

    fn expand(&mut self, index: Index, state: &mut T) -> Index {
        let legal_actions = state.legal_actions();

        let action = legal_actions
            .iter()
            .find(|&action| !self.nodes[index].children.contains_key(action))
            .expect("Action should not be None");

        let new_index = self.create_node();
        self.nodes[new_index].set_parent(index);
        self.nodes[index].set_child(*action, new_index);

        return new_index;
    }

    fn simulation(&mut self, mut state: T, max_depth: u32) -> i32 {
        let mut rng = rand::thread_rng();

        let mut wins = 0;

        let mut depth = 0;

        while !state.has_finished() && max_depth >= depth {
            let legal_actions = state.legal_actions();

            let random_index = rng.gen_range(0..legal_actions.len());
            let action = legal_actions[random_index];
            state.perform_action(action);

            depth += 1;
        }
        wins += state.eval();

        return wins;
    }

    fn backpropagate(&mut self, wins: i32, visits: u32, index: Index) {
        let mut cursor = Some(index);

        while let Some(index) = cursor {
            self.nodes[index].update(wins, visits);
            cursor = self.nodes[index].parent;
        }
    }
}
