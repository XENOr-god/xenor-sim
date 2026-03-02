use std::collections::{HashMap, HashSet};
use xenor_core::core::percolation::{units, Amount, Graph, PropagationStop};

fn main() {
    // ===== Build graph =====
    // 1 -> 2 (0.7), 1 -> 3 (0.3)
    // 2 -> 4 (1.0)
    // 3 -> 4 (0.5), 3 -> 5 (0.5)
    //
    // Inflow:
    // 1: 0.0
    // 2: 0.7
    // 3: 0.3
    // 4: 1.5 (1.0 from 2 + 0.5 from 3)
    // 5: 0.5

    let mut g = Graph::default();
    g.add_edge(1, 2, 0.7).unwrap();
    g.add_edge(1, 3, 0.3).unwrap();
    g.add_edge(2, 4, 1.0).unwrap();
    g.add_edge(3, 4, 0.5).unwrap();
    g.add_edge(3, 5, 0.5).unwrap();

    // ===== Activation config =====
    // threshold 0.5 => active nodes: 2,4,5 (3 is inactive because inflow=0.3)
    let threshold = 0.5;
    let max_iters = 100;

    // initial active seed (bebas): biasanya sumber reward dimasukkan biar "starting set"
    let mut initial_active: HashSet<u64> = HashSet::new();
    initial_active.insert(1);

    let prop = g.propagate_until_stable(initial_active, threshold, max_iters);
    println!("=== XENOR-SIM: activation + gated reward propagation ===");
    println!("threshold: {threshold}");
    println!("active rounds (propagate): {}", prop.rounds);
    println!(
        "active stop: {}",
        match prop.stop {
            PropagationStop::Stable => "Stable",
            PropagationStop::ReachedMaxIters => "ReachedMaxIters",
        }
    );

    let mut active_list: Vec<u64> = prop.active.iter().copied().collect();
    active_list.sort_unstable();
    println!("Active nodes: {:?}", active_list);

    // ===== Initial rewards =====
    let mut current: HashMap<u64, Amount> = HashMap::new();
    current.insert(1, units(100.0));

    println!("\nInitial total: {}", fmt_amount(sum_map(&current)));

    // ===== Reward propagation (gated by active-set) =====
    let max_rounds = 50;
    for r in 1..=max_rounds {
        let before = sum_map(&current);

        // distribute ONLY from active nodes; others "park" their balance
        let next = distribute_rewards_gated(&g, &current, &prop.active);

        let after = sum_map(&next);

        println!("\n--- Reward Round {r} ---");
        println!("Total before: {}", fmt_amount(before));
        println!("Total after : {}", fmt_amount(after));
        println!("Conserved?  : {}", if before == after { "YES" } else { "NO" });

        print_balances(&next);

        if next == current {
            println!("\nReward stable reached at round {r} ✅");
            break;
        }

        current = next;
    }

    println!("\n=== Done ===");
}

/// Gated reward distribution:
/// - kalau node TIDAK aktif => reward tetap di node itu (tidak mengalir)
/// - kalau node aktif => reward didistribusikan pakai xenor-core (konservatif)
fn distribute_rewards_gated(
    g: &Graph,
    current: &HashMap<u64, Amount>,
    active: &HashSet<u64>,
) -> HashMap<u64, Amount> {
    let mut gated_input: HashMap<u64, Amount> = HashMap::new();
    let mut parked: HashMap<u64, Amount> = HashMap::new();

    // split: active -> boleh mengalir; inactive -> park
    for (&node, &amt) in current.iter() {
        if amt == 0 {
            continue;
        }
        if active.contains(&node) {
            gated_input.insert(node, amt);
        } else {
            *parked.entry(node).or_insert(0) += amt;
        }
    }

    // distribute from active nodes only
    let ledger = g.distribute_rewards(&gated_input);

    // merge parked + distributed
    let mut next = ledger.balances;
    for (node, amt) in parked {
        *next.entry(node).or_insert(0) += amt;
    }

    next
}

fn sum_map(m: &HashMap<u64, Amount>) -> Amount {
    m.values().copied().sum()
}

fn print_balances(balances: &HashMap<u64, Amount>) {
    let mut entries: Vec<(u64, Amount)> = balances.iter().map(|(&k, &v)| (k, v)).collect();
    entries.sort_by_key(|(k, _)| *k);

    for (node, amt) in entries {
        println!("node {node}: {}", fmt_amount(amt));
    }
}

fn fmt_amount(a: Amount) -> String {
    let scale: i128 = 1_000_000;
    let sign = if a < 0 { "-" } else { "" };
    let x = a.abs();
    let whole = x / scale;
    let frac = (x % scale) as u32;
    format!("{sign}{whole}.{frac:06}")
}