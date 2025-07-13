use itertools::Itertools;
use threadpool::ThreadPool;
use threadpool_scope::scope_with;

use std::{
    cell::RefCell,
    cmp,
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc_core::{end_measure, read, spatial::Point, start_measure};

const SYMBOLS: &[u8] = b"0123456789A<>^V";
const SYMBOL_SIZE: usize = SYMBOLS.len();

type Cache = HashMap<char, HashMap<char, HashSet<String>>>;
type OptimizedCache<'a> = [[&'a str; SYMBOL_SIZE]; SYMBOL_SIZE];

type OptionWithScore = (String, usize, Vec<String>);

const KEYMAP: [u8; 256] = {
    let mut lut = [0xFFu8; 256];
    let mut i = 0;
    while i < SYMBOLS.len() {
        lut[SYMBOLS[i] as usize] = i as u8;
        i += 1;
    }
    lut
};

#[inline(always)]
fn idx(c: char) -> usize {
    KEYMAP[c as usize] as usize
}

#[derive(Debug)]
struct KeyPad {
    keys: HashMap<char, Point>,
    pointer: char,
    cache0: RefCell<Cache>,
}

impl KeyPad {
    pub fn arrow_pad() -> Self {
        KeyPad::new(HashMap::from([
            ('^', Point { x: 1, y: 0 }),
            ('A', Point { x: 2, y: 0 }),
            ('<', Point { x: 0, y: 1 }),
            ('V', Point { x: 1, y: 1 }),
            ('>', Point { x: 2, y: 1 }),
        ]))
    }

    pub fn num_pad() -> Self {
        KeyPad::new(HashMap::from([
            ('7', Point { x: 0, y: 0 }),
            ('8', Point { x: 1, y: 0 }),
            ('9', Point { x: 2, y: 0 }),
            ('4', Point { x: 0, y: 1 }),
            ('5', Point { x: 1, y: 1 }),
            ('6', Point { x: 2, y: 1 }),
            ('1', Point { x: 0, y: 2 }),
            ('2', Point { x: 1, y: 2 }),
            ('3', Point { x: 2, y: 2 }),
            ('0', Point { x: 1, y: 3 }),
            ('A', Point { x: 2, y: 3 }),
        ]))
    }

    fn new(keys: HashMap<char, Point>) -> Self {
        KeyPad {
            keys,
            cache0: RefCell::new(HashMap::new()),
            pointer: 'A',
        }
    }

    pub fn set(&mut self, pointer: char) {
        self.pointer = pointer;
    }

    pub fn press(&mut self, sequence: &str, cache: &Cache) -> Vec<String> {
        self.set('A');
        let mut branches: Vec<String> = Vec::new();

        for key in sequence.chars() {
            let parts = cache
                .get(&self.pointer)
                .unwrap()
                .get(&key)
                .unwrap();

            if branches.len() == 0 {
                branches = parts.to_owned().into_iter().collect();
            } else {
                branches = KeyPad::get_branches(branches, parts);
            }

            self.pointer = key;
        }

        branches
    }

    pub fn press_optimized(
        &mut self,
        sequence: &str,
        cache: &OptimizedCache,
    ) -> String {
        self.set('A');
        let mut res =
            String::with_capacity((sequence.len() as f32 * 2.6) as usize); // emperical factor

        for key in sequence.chars() {
            let parts = cache[idx(self.pointer)][idx(key)];
            res.push_str(parts);

            self.pointer = key;
        }

        res
    }

    fn get_branches(
        branches: Vec<String>,
        parts: &HashSet<String>,
    ) -> Vec<String> {
        let mut new_branches = Vec::with_capacity(branches.len() * parts.len());

        for branch in branches {
            for part in parts {
                let mut s = String::with_capacity(branch.len() + part.len());
                s.push_str(&branch);
                s.push_str(part);

                new_branches.push(s);
            }
        }

        new_branches
    }

    pub fn fill_cache(&mut self) {
        for (from_key, from_point) in self.keys.clone() {
            let other_keys = self.keys.iter().filter(|k| *k.0 != from_key);
            let mut inner_cache = HashMap::new();
            inner_cache.insert(from_key, HashSet::from(["A".to_string()]));

            for (to_key, to_point) in other_keys {
                let dx = to_point.x - from_point.x;
                let dy = to_point.y - from_point.y;

                let x_char = if dx > 0 { ">" } else { "<" };
                let y_char = if dy > 0 { "V" } else { "^" };

                let input: Vec<char> = x_char
                    .repeat(dx.abs() as usize)
                    .chars()
                    .chain(y_char.repeat(dy.abs() as usize).chars())
                    .collect();

                let unique_perms: HashSet<String> = input
                    .iter()
                    .permutations(input.len())
                    .map(|p| p.into_iter().collect())
                    .map(|mut s: String| {
                        s.push('A');
                        s
                    })
                    .filter(|s| KeyPad::filter_blocked_paths(s, from_key))
                    .collect();

                inner_cache.insert(*to_key, unique_perms);
            }

            self.cache0
                .borrow_mut()
                .insert(from_key, inner_cache);
        }
    }

    fn filter_blocked_paths(sequence: &str, from_key: char) -> bool {
        match from_key {
            '7' => !sequence.starts_with("VVV"),
            '4' => !sequence.starts_with("VV"),
            '1' => !sequence.starts_with("V"),
            '0' => !sequence.starts_with("<"),
            'A' => !sequence.starts_with("<<"),
            '^' => !sequence.starts_with("<"),
            '<' => !sequence.starts_with("^"),
            _ => true,
        }
    }

    fn optimize_cache<'a>(&self, cache: &'a Cache) -> OptimizedCache<'a> {
        let mut new_cache = [[""; SYMBOL_SIZE]; SYMBOL_SIZE];

        for (from_key, cache) in cache {
            for (to_key, options) in cache {
                // hack: while reducing cache there is one we cannot calculate
                // V to A in arrow_cache ... there are 2 options
                // and the last one is right -> using .last() instead of .next()
                let seq = options.iter().last().unwrap();

                new_cache[idx(*from_key)][idx(*to_key)] = seq;
            }
        }

        new_cache
    }

    fn reduce_cache(
        &mut self,
        initial_cache: &Cache,
        external_cache: &Cache,
        levels: usize,
    ) -> Cache {
        let mut new_cache = HashMap::new();

        for (from_key, cache) in initial_cache {
            let mut inner_cache = HashMap::new();

            for (to_key, options) in cache {
                let mapped_options: Vec<_> = options
                    .into_iter()
                    .map(|s| (s.clone(), s.clone()))
                    .collect();

                let best_options = self.get_best_options(
                    mapped_options,
                    external_cache,
                    0,
                    levels,
                );

                let set: HashSet<_> =
                    best_options.iter().map(|s| s.0.clone()).collect();

                inner_cache.insert(*to_key, set);
            }

            new_cache.insert(*from_key, inner_cache);
        }

        new_cache
    }

    fn get_best_options(
        &mut self,
        options: Vec<(String, String)>,
        external_cache: &Cache,
        level: usize,
        max_level: usize,
    ) -> Vec<OptionWithScore> {
        if level == max_level {
            return options
                .iter()
                .map(|o| (o.0.clone(), o.1.len(), vec![]))
                .collect();
        }

        let scores: Vec<_> = options
            .into_iter()
            .map(|s| {
                let new_options = self.press(s.1.as_str(), external_cache);
                (s.0.clone(), new_options[0].len(), new_options)
            })
            .collect();

        let top_scores = KeyPad::filter_best(scores);

        if top_scores.len() == 1 {
            top_scores
                .into_iter()
                .map(|t| t.clone())
                .collect()
        } else {
            let new_canidates: Vec<_> = top_scores
                .iter()
                .map(|ts| ts.2.iter().map(|s| (ts.0.clone(), s.clone())))
                .flatten()
                .collect();

            let new_scores: Vec<_> = self.get_best_options(
                new_canidates,
                external_cache,
                level + 1,
                max_level,
            );

            KeyPad::filter_best(new_scores)
        }
    }

    fn filter_best(scores: Vec<OptionWithScore>) -> Vec<OptionWithScore> {
        let best_score = scores
            .iter()
            .map(|s| s.clone())
            .min_by_key(|(_, s, _)| s.clone())
            .unwrap()
            .1;

        scores
            .into_iter()
            .filter(|(_, s, _)| *s == best_score)
            .collect()
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    println!("{}", input);

    let mut num_pad = KeyPad::num_pad();
    let mut arrow_pad = KeyPad::arrow_pad();

    arrow_pad.fill_cache();
    let arrow_cache0_c = arrow_pad.cache0.borrow().clone();

    let arrow_cache2 =
        arrow_pad.reduce_cache(&arrow_cache0_c, &arrow_cache0_c, 2);
    let arrow_cache = arrow_pad.reduce_cache(&arrow_cache2, &arrow_cache2, 4);

    let arrow_cache_o = arrow_pad.optimize_cache(&arrow_cache);

    num_pad.fill_cache();
    let num_cache0_c = num_pad.cache0.borrow().clone();
    let num_cache2 = num_pad.reduce_cache(&num_cache0_c, &arrow_cache, 2);
    let num_cache6 = num_pad.reduce_cache(&num_cache2, &arrow_cache, 4);

    let num_cache = num_pad.reduce_cache(&num_cache6, &arrow_cache, 25);
    let num_cache_o = num_pad.optimize_cache(&num_cache);

    let mut sum = 0;

    for sequence in input.lines() {
        sum += process_sequence(
            sequence,
            &mut num_pad,
            num_cache_o,
            arrow_cache_o,
        );
    }

    println!();
    println!("sum: {}", sum);
    println!();

    end_measure(mes);
}

fn process_sequence(
    sequence: &str,
    num_pad: &mut KeyPad,
    num_cache: OptimizedCache,
    arrow_cache: OptimizedCache,
) -> usize {
    let mut res = num_pad.press_optimized(sequence.trim(), &num_cache);

    println!("------");
    let cycles = 25;

    let chunks = 12;
    let mut sum = 0;

    let pool = ThreadPool::new(2);

    for i in 0..cycles {
        let mut new_res = String::new();

        scope_with(&pool, |s| {
            let (tx, rx) = crossbeam::channel::bounded(2);
            let mes_cycle = Instant::now();

            let len = res.len();
            let chunk_size = len / chunks;

            for _ in 0..chunks {
                let mut end = cmp::min(chunk_size, res.len());

                let after_end = &res[end..];
                let next_a_offset = after_end
                    .find('A')
                    .map(|off| off + 'A'.len_utf8())
                    .unwrap_or(after_end.len());

                end = end + next_a_offset;
                let chunk: String = res.drain(0..end).collect();
                res.shrink_to_fit();

                let arrow_cache_clone = arrow_cache.clone();
                let tx = tx.clone();

                s.execute(move || {
                    let chunk_res = press_optimized(&chunk, &arrow_cache_clone);

                    tx.send(chunk_res).unwrap();
                });
            }

            drop(tx);

            for thread_res in rx {
                if i == cycles - 1 {
                    sum = sum + thread_res.len();
                } else {
                    new_res.push_str(&thread_res);
                }
            }

            println!(
                "finish cycle {} for {} [{:?}]",
                i + 1,
                sequence,
                mes_cycle.elapsed()
            );
        });

        res = new_res;
    }

    let numeric_part: usize = sequence[..3].parse().unwrap();
    println!("------");
    println!("seq: {}", sequence);
    println!("{} * {}", sum, numeric_part);
    println!();

    sum * numeric_part
}

pub fn press_optimized(sequence: &str, cache: &OptimizedCache) -> String {
    let mut pointer = 'A';
    let mut res = String::with_capacity((sequence.len() as f32 * 2.6) as usize); // emperical factor

    for key in sequence.chars() {
        let parts = cache[idx(pointer)][idx(key)];
        res.push_str(parts);

        pointer = key;
    }

    res
}
