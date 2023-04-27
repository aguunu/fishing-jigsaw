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
            max_depth: 16,
            c: f32::sqrt(2.0),
            callback_interval: 1000,
        }
    }
}

pub fn compute<T, F>(state: &T, config: Config, callback: F)
where
    T: Environment + Clone,
    F: FnMut(Stats<T>),
{
    let mut tree = Tree::new(state, config);
    tree.compute(callback);
}

impl<T: Environment + Clone> Tree<T> {
    fn compute<F>(&mut self, mut callback: F)
    where
        F: FnMut(Stats<T>),
    {
        for iter in 0..self.config.max_iters {
            self.try_callback_trigger::<T>(iter, &mut callback);

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

    pub fn new(state: &T, config: Config) -> Self {
        let mut result = Tree {
            nodes: Vec::with_capacity(config.max_iters as usize),
            root_state: state.clone(),
            root_index: 0,
            config,
        };

        result.create_node(); // create root node

        result
    }

    fn try_callback_trigger<F>(&self, current_iter: u32, callback: &mut dyn FnMut(Stats<T>)) {
        if current_iter == 0 || current_iter % self.config.callback_interval != 0 {
            return;
        }

        let actions: Vec<(T::Action, u32)> = self.nodes[self.root_index]
            .children
            .iter()
            .map(|(action, index)| (*action, self.nodes[*index].visits))
            .collect();

        callback(Stats {
            iters: current_iter,
            actions,
        });
    }

    fn create_node(&mut self) -> Index {
        let index = self.nodes.len();

        self.nodes.push(Node::new());

        return index;
    }

    fn select(&mut self, index: Index, c: f32, state: &mut T, state_depth: &mut u32) -> Index {
        /* Check if states is terminal */
        if state.has_finished() {
            return index;
        }

        let legal_actions = state.legal_actions();

        let node = &self.nodes[index];

        let fully_expanded = legal_actions
            .iter()
            .all(|action| node.children.contains_key(action));

        /* Check if state is not fully expanden or is terminal */
        if !fully_expanded {
            return index;
        }

        /* Select next state (unknown) by taking best action */
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

        /* Perform selected action */
        state.perform_action(action);

        *state_depth += 1;

        return self.select(node.children[&action], c, state, state_depth);
    }

    fn expand(&mut self, index: Index, state: &mut T) -> Index {
        let legal_actions = state.legal_actions();

        let action = legal_actions
            .iter()
            .find(|&action| !self.nodes[index].children.contains_key(action));

        match action {
            Some(action) => {
                let new_index = self.create_node();
                self.nodes[new_index].set_parent(index);
                self.nodes[index].set_child(*action, new_index);

                return new_index;
            }
            None => panic!(),
        }
    }

    fn simulation(&mut self, mut state: T, max_depth: u32) -> i32 {
        let mut rng = rand::thread_rng();

        let mut wins = 0;

        let mut depth = 0;

        while !state.has_finished() && max_depth >= depth {
            let legal_actions = state.legal_actions();
            let action_index: u8 = rng.gen::<u8>() % u8::try_from(legal_actions.len()).unwrap();
            let action = legal_actions.get(action_index as usize).unwrap();
            state.perform_action(*action);

            depth += 1;
        }
        wins += state.eval();

        return wins;
    }

    fn backpropagate(&mut self, wins: i32, visits: u32, index: Index) {
        let mut current_index = index;

        loop {
            self.nodes[current_index].update(wins, visits);

            match self.nodes[current_index].parent {
                None => break,
                Some(parent) => current_index = parent,
            }
        }
    }
}
