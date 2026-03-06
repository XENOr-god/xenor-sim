use std::collections::{HashMap, HashSet};
use xenor_core::core::percolation::{Amount, Graph, units};

fn main() {
    // ===== Graph =====
    let mut g = Graph::default();
    g.add_edge(1, 2, 0.7).unwrap();
    g.add_edge(1, 3, 0.3).unwrap();
    g.add_edge(2, 4, 1.0).unwrap();
    g.add_edge(3, 4, 0.5).unwrap();
    g.add_edge(3, 5, 0.5).unwrap();

    // ===== Dynamic activation threshold (reward inflow-based) =====
    // Contoh: node akan aktif kalau menerima >= 20 token dalam 1 round.
    let threshold_reward: Amount = units(20.0);

    // ===== Initial inflow (round 0) =====
    // Anggap initial rewards itu inflow pertama.
    let mut balances: HashMap<u64, Amount> = HashMap::new();
    balances.insert(1, units(100.0));

    let mut inflow: HashMap<u64, Amount> = balances.clone(); // inflow round 0 = initial rewards
    let mut active: HashSet<u64> = active_from_inflow(&inflow, threshold_reward);

    println!("=== XENOR-SIM: dynamic activation by reward inflow ===");
    println!("threshold_reward: {}", fmt_amount(threshold_reward));
    println!("Initial total: {}", fmt_amount(sum_map(&balances)));

    let max_rounds = 50;

    for r in 1..=max_rounds {
        println!("\n--- Round {r} ---");
        println!(
            "Active (based on previous inflow): {:?}",
            sorted_set(&active)
        );

        // split balances:
        // - active nodes can send their ENTIRE balance
        // - inactive nodes keep their balance parked
        let (send_map, parked_map) = split_by_active(&balances, &active);

        // distribute ONLY from active senders
        let ledger = g.distribute_rewards(&send_map);

        // reward inflow for this round = ledger result (what nodes receive)
        inflow = ledger.balances.clone();

        // next balances = parked + inflow received
        let mut next_balances = parked_map;
        for (node, amt) in ledger.balances {
            *next_balances.entry(node).or_insert(0) += amt;
        }

        let before = sum_map(&balances);
        let after = sum_map(&next_balances);

        println!("Total before: {}", fmt_amount(before));
        println!("Total after : {}", fmt_amount(after));
        println!(
            "Conserved?  : {}",
            if before == after { "YES" } else { "NO" }
        );

        println!("Inflow this round:");
        print_balances(&inflow);

        println!("Balances after merge (parked + inflow):");
        print_balances(&next_balances);

        // compute active set for next round based on THIS round inflow
        let next_active = active_from_inflow(&inflow, threshold_reward);

        // stop when balances stop changing AND active-set also stable
        if next_balances == balances && next_active == active {
            println!("\nStable reached at round {r} ✅");
            break;
        }

        balances = next_balances;
        active = next_active;
    }

    println!("\n=== Done ===");
}

fn active_from_inflow(inflow: &HashMap<u64, Amount>, threshold: Amount) -> HashSet<u64> {
    inflow
        .iter()
        .filter_map(|(&node, &amt)| if amt >= threshold { Some(node) } else { None })
        .collect()
}

fn split_by_active(
    balances: &HashMap<u64, Amount>,
    active: &HashSet<u64>,
) -> (HashMap<u64, Amount>, HashMap<u64, Amount>) {
    let mut send = HashMap::new();
    let mut parked = HashMap::new();

    for (&node, &amt) in balances {
        if amt == 0 {
            continue;
        }
        if active.contains(&node) {
            send.insert(node, amt);
        } else {
            parked.insert(node, amt);
        }
    }

    (send, parked)
}

fn sum_map(m: &HashMap<u64, Amount>) -> Amount {
    m.values().copied().sum()
}

fn sorted_set(s: &HashSet<u64>) -> Vec<u64> {
    let mut v: Vec<u64> = s.iter().copied().collect();
    v.sort_unstable();
    v
}

fn print_balances(balances: &HashMap<u64, Amount>) {
    let mut entries: Vec<(u64, Amount)> = balances.iter().map(|(&k, &v)| (k, v)).collect();
    entries.sort_by_key(|(k, _)| *k);

    if entries.is_empty() {
        println!("(empty)");
        return;
    }

    for (node, amt) in entries {
        println!("  node {node}: {}", fmt_amount(amt));
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
