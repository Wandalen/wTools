#![allow(missing_debug_implementations)]
#![allow(missing_docs)]

use criterion :: { criterion_group, criterion_main, Criterion };
use wca ::grammar ::Dictionary;
use wca :: { CommandsAggregator, Type };

fn init(count: usize, command: &wca ::grammar ::Command) -> CommandsAggregator 
{
  let mut dic_former = Dictionary ::former();
  for i in 0..count 
  {
  let name = format!("command_{i}");

  let mut command = command.clone();
  command.phrase.clone_from(&name);

  dic_former = dic_former.command(command);
 }
  let dictionary = dic_former.form();

  // The CommandsAggregator has changed and there are no more grammar fields and the executor no longer stores routines.
  // Accordingly, I made changes and write commands through DictionaryFormer and pass it to CommandsAggregator
  CommandsAggregator ::former().dictionary(dictionary).perform()
}

fn initialize_commands_without_args(count: usize) -> CommandsAggregator 
{
  init(
  count,
  &wca ::grammar ::Command ::former()
   .hint("hint")
   .long_hint("long_hint")
   .phrase("{placeholder}")
   .form(),
 )
}

fn initialize_commands_with_subjects(count: usize) -> CommandsAggregator 
{
  // The way commands are initialized has changed, now the ComandFormer from the grammar module is used and the subject() and property methods are called differently
  init(
  count,
  &wca ::grammar ::Command ::former()
   .hint("hint")
   .long_hint("long_hint")
   .phrase("{placeholder}")
   .subject()
   .hint("hint")
   .kind(Type ::String)
   .optional(true)
   .end()
   .subject()
   .hint("hint")
   .kind(Type ::String)
   .optional(true)
   .end()
   .form(),
 )
}

fn initialize_commands_with_properties(count: usize) -> CommandsAggregator 
{
  init(
  count,
  &wca ::grammar ::Command ::former()
   .hint("hint")
   .long_hint("long_hint")
   .phrase("{placeholder}")
   .property("prop")
   .hint("hint")
   .kind(Type ::String)
   .optional(true)
   .end()
   .property("prop2")
   .hint("hint")
   .kind(Type ::String)
   .optional(true)
   .end()
   .form(),
 )
}

fn run_commands< S: AsRef<str >>(ca: &CommandsAggregator, command: S) 
{
  ca.perform(command.as_ref()).unwrap();
}

fn benchmark_initialize_thousand_commands(c: &mut Criterion) 
{
  const COUNT: usize = 1_000;

  c.bench_function("initialize_thousand_commands_without_args", |b| {
  b.iter(|| initialize_commands_without_args(COUNT));
 });
  c.bench_function("initialize_thousand_commands_with_subjects", |b| {
  b.iter(|| initialize_commands_with_subjects(COUNT));
 });
  c.bench_function("initialize_thousand_commands_with_properties", |b| {
  b.iter(|| initialize_commands_with_properties(COUNT));
 });
}

fn benchmark_initialize_and_run_thousand_commands(c: &mut Criterion) 
{
  const COUNT: usize = 1_000;

  c.bench_function("initialize_and_run_thousand_commands_without_args", |b| {
  b.iter(|| {
   let ca = initialize_commands_without_args(COUNT);
   run_commands(&ca, ".command_999");
 });
 });
  c.bench_function("initialize_and_run_thousand_commands_with_subjects", |b| {
  b.iter(|| {
   let ca = initialize_commands_with_subjects(COUNT);
   run_commands(&ca, ".command_999");
 });
 });
  c.bench_function("initialize_and_run_thousand_commands_with_properties", |b| {
  b.iter(|| {
   let ca = initialize_commands_with_properties(COUNT);
   run_commands(&ca, ".command_999");
 });
 });
}

criterion_group!(
  benches,
  benchmark_initialize_thousand_commands,
  benchmark_initialize_and_run_thousand_commands
);
criterion_main!(benches);
