//! Simple benchkit-based specialized algorithm benchmarks
//!
//! This benchmark uses benchkit exclusively to measure specialized algorithm performance

use benchkit ::prelude :: *;
use strs_tools ::string ::specialized :: { 
  smart_split, SingleCharSplitIterator, BoyerMooreSplitIterator
};
use strs_tools ::string;

fn main() 
{
  println!("🚀 Specialized Algorithms Benchkit Analysis");
  println!("==========================================");

  // Generate test data
  let single_char_data = "word1,word2,word3,word4,word5,word6,word7,word8,word9,word10".repeat(100);
  let multi_char_data = "field1 ::field2 ::field3 ::field4 ::field5 ::field6 ::field7 ::field8".repeat(100);

  // Single character splitting comparison
  println!("\n📊 Single Character Splitting Comparison");
  println!("----------------------------------------");
  
  let (_generic_count, generic_time) = time_block(|| {
  string ::split()
   .src(&single_char_data)
   .delimeter(",")
   .perform()
   .count()
 });
  
  let (_specialized_count, specialized_time) = time_block(|| {
  SingleCharSplitIterator ::new(&single_char_data, ',', false)
   .count()
 });
  
  let (_smart_count, smart_time) = time_block(|| {
  smart_split(&single_char_data, &[ ","])
   .count()
 });

  println!("Generic split: {generic_time:?}");
  println!("Specialized split: {specialized_time:?}");
  println!("Smart split: {smart_time:?}");

  // Multi character splitting comparison
  println!("\n📊 Multi Character Splitting Comparison"); 
  println!("----------------------------------------");
  
  let (_generic_multi_count, generic_multi_time) = time_block(|| {
  string ::split()
   .src(&multi_char_data)
   .delimeter(" :: ")
   .perform()
   .count()
 });
  
  let (_boyer_moore_count, boyer_moore_time) = time_block(|| {
  BoyerMooreSplitIterator ::new(&multi_char_data, " :: ")
   .count()
 });
  
  let (_smart_multi_count, smart_multi_time) = time_block(|| {
  smart_split(&multi_char_data, &[ " :: "])
   .count()
 });

  println!("Generic split: {generic_multi_time:?}");
  println!("Boyer-Moore split: {boyer_moore_time:?}");
  println!("Smart split: {smart_multi_time:?}");

  println!("\n✅ Benchmarks completed successfully!");
}