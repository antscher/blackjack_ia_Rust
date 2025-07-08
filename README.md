# BlackJack IA (Rust)

## Description
This project implements an artificial intelligence (AI) agent that learns to play Blackjack using the Q-learning algorithm, written in Rust. The agent interacts with a simulated Blackjack environment and improves its strategy over time by updating a Q-table.

## Features
- **Q-learning algorithm**: The agent uses a Q-table to learn the best actions to take in each state.
- **Dynamic state discovery**: Unlike traditional implementations where all possible states are defined before training, this project dynamically adds new states to the Q-table as they are encountered during training. This makes the implementation more flexible and memory-efficient.
- **Customizable parameters**: Learning rate (alpha), discount factor (gamma), and exploration rate (epsilon) are easily adjustable in the code.
- **Serialization**: The Q-table can be saved to and loaded from a JSON file.

## Blackjack Rules
The rules used are based on standard Blackjack. For more details, see:  
https://www.pokerstars.com/fr/casino/how-to-play/blackjack/premium/

## How it works
- The agent plays simulated games of Blackjack.
- For each state-action pair, it updates the Q-table using the Q-learning update rule:
  
  `Q(s, a) ← Q(s, a) + α × [r + γ × max_a' Q(s', a') - Q(s, a)]`
- States are created on-the-fly as the agent encounters new situations.
- After training, the Q-table can be used to play optimally (according to what the agent has learned).

## Project Structure
- `src/`
  - `main.rs`: Entry point of the program.
  - `game.rs`: Game logic and state management.
  - `card.rs`: Card and deck definitions.
  - `training.rs`: Q-learning logic and Q-table management.
- `qtable.json`: Serialized Q-table (after training).
- `Cargo.toml`: Rust project configuration.

## Usage
1. **Build the project:**
   ```sh
   cargo build --release
   ```
2. **Run training:**
   ```sh
   cargo run --release
   ```
   (You can modify the number of training episodes and parameters in the code.)
3. **Q-table output:**
   After training, the Q-table is saved to `qtable.json`.

## Customization
- You can adjust the learning parameters (`ALPHA`, `GAMMA`, `EPSILON`) in `training.rs`.
- The code is modular and can be extended for more actions or rule variations.


## Results
- After training, the AI still loses most of the time. On average, it gets a score of -50 for every 1000 games played.
- At the start, when the AI was just picking random moves, it did much worse, with a score of -600 for every 1000 games. Training with Q-learning helped the AI play better and lose less often.
- This means the AI learned to make better choices, but the rules of the game still make it hard to win. If we change some rules or try different settings, the AI might do even better.

## Collaborators 

Mahel Rafes (https://github.com/moefijmqefijmejfmzcds/)

Antoine Scherpereel (https://github.com/antscher/)

## License
This project is protect by a licence but feel free to use it and to improve it ;)
>>>>>>> two_actions
