pub mod card;
pub mod game;
pub mod training;
use crate::training::QTable;
use game::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const NB_ITERATIONS: u64 = 1_000_000; // Number of iterations for training
const NUM_THREADS: usize = 20;
const SPINNER_FRAMES: &[&str] = &["|", "/", "-", "\\"];

fn main() {
    let locker = Arc::new(Mutex::new(QTable::new()));
    let mut handles = vec![];
    let progress = Arc::new(Mutex::new(vec![(0u8, 0f32, 0usize); NUM_THREADS]));

    // Affichage initial
    for _ in 0..NUM_THREADS {
        println!("Thread progress: ");
    }

    // Thread d'affichage
    {
        let progress = Arc::clone(&progress);
        thread::spawn(move || {
            loop {
                {
                    let mut progress = progress.lock().unwrap();
                    // Remonter le curseur
                    print!("\x1B[{}A", NUM_THREADS);
                    for (i, (percent, avg_reward, spinner_idx)) in progress.iter_mut().enumerate() {
                        // Update spinner
                        *spinner_idx = (*spinner_idx + 1) % SPINNER_FRAMES.len();
                        let spinner = SPINNER_FRAMES[*spinner_idx];
                        println!(
                            "Thread {:>2}: {:>3}% {}, average reward : {}",
                            i, percent, spinner, avg_reward
                        );
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    for thread_id in 0..NUM_THREADS {
        let locker = Arc::clone(&locker);
        let progress = Arc::clone(&progress);
        let handle = thread::spawn(move || {
            training_for_thread(NB_ITERATIONS, &locker, &progress, thread_id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Save the QTable to a file
    let q = locker.lock().unwrap();
    let res = q.save("qtable.json");
    match res {
        Ok(_) => {
            print!("success");
        }
        Err(e) => {
            print!("failed, {:?}", e);
        }
    }
}

pub fn training_for_thread(
    nb_iterations: u64,
    locker: &Arc<Mutex<QTable>>,
    progress: &Arc<Mutex<Vec<(u8, f32, usize)>>>,
    thread_id: usize,
) {
    let mut avg_reward = 0.0;
    for per in 0..nb_iterations {
        let percentage = nb_iterations as f32 * 0.05;

        let epsilon = if per < percentage as u64 {
            1.0 - (per as f32 / nb_iterations as f32)
        } else {
            0.02
        };
        let mut q = locker.lock().unwrap();
        let mut game_state = GameState::new();
        let reward = q.trainnig_q(&mut game_state, 1.0, epsilon);

        // Mise à jour du pourcentage toutes les 100 itérations
        if per % 1000 == 0 || per == nb_iterations - 1 {
            let percent = ((per + 1) * 100 / nb_iterations) as u8;
            let mut p = progress.lock().unwrap();
            p[thread_id].0 = percent;

            p[thread_id].1 = avg_reward;
            avg_reward = 0.0;
        } else {
            avg_reward += reward;
        }
    }
}
