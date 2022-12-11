// Attempt to setup stable Rust benchmarking

// #[macro_use]
// extern crate bencher;
// extern crate aoc_2022;
// use std::fs;

// use aoc_2022::days::day6;
// use bencher::Bencher;


// fn bench_find_first_marker(b: &mut Bencher) {
//     let input = fs::read_to_string(format!("./input/day6.txt")).unwrap();
//     b.iter(|| day6::find_first_marker(&input, 14));
// }

// fn bench_find_first_marker_optimized(b: &mut Bencher) {
//     let input = fs::read_to_string(format!("./input/day6.txt")).unwrap();
//     b.iter(|| day6::find_first_marker_optimized(&input, 14));
// }

// // fn part_one(b: &mut Bencher){
// //     let input = fs::read_to_string(format!("./input/day8.txt")).unwrap();
// //     b.iter(|| day8::part1(&input));
// // }

// // fn part_two(b: &mut Bencher) {
// //     let input = fs::read_to_string(format!("./input/day8.txt")).unwrap();
// //     b.iter(|| day8::part2(&input));
// // }

// benchmark_group!(benches, bench_find_first_marker, bench_find_first_marker_optimized);
// benchmark_main!(benches);
