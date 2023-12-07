use omalley_aoc2023 as aoc_lib;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! benchmarks_internal {
  ( $($day:ident),* ) => {
    paste::paste!{
      $(fn [<$day _benchmark>](c: &mut Criterion) {
          use aoc_lib::$day;
          let input_data = aoc_lib::utils::read_inputs("input", &vec![stringify!($day)])
            .expect("can't read input");
          let input = $day::generator(&input_data[0]);
          c.bench_function(concat!(stringify!($day), " gen"), |b| {
            b.iter(|| $day::generator(&input_data[0]))
          });
          c.bench_function(concat!(stringify!($day), " part 1"), |b| {
            b.iter(|| $day::part1(&input))
          });
          c.bench_function(concat!(stringify!($day), " part 2"), |b| {
            b.iter(|| $day::part2(&input))
          });
        }
        criterion_group!($day, [<$day _benchmark>]);
      )*

      criterion_main!($($day),*);
    }
  };
}

#[macro_export]
macro_rules! benchmarks {
  ( $($day:literal),* ) => {
    paste::paste!{ benchmarks_internal!{$( [<day $day>] ),*} }
  }
}

benchmarks!(1,2,3,4,5,6);
