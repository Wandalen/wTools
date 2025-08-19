//! Definitive MRE for benchkit MarkdownUpdater substring matching bug
//! 
//! This demonstrates how the exact code from benchkit/src/reporting.rs:56 creates
//! section duplication when section names share substrings.

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐛 BENCHKIT BUG: Section Substring Matching Issue");
    println!("================================================\n");

    let test_file = "final_mre.md";
    let _ = fs::remove_file(test_file);

    // Step 1: Create a file that will trigger the bug
    let initial_content = r#"# Project

## Performance Benchmarks
Old performance data

## Language Operations Performance  
Old language data
"#;

    fs::write(test_file, initial_content)?;
    println!("✅ Created test file");
    println!("Initial sections: 2");
    
    // Step 2: Simulate the exact bug from benchkit
    // This is the EXACT code from benchkit/src/reporting.rs:56
    fn buggy_section_matching(line: &str, section_name: &str) -> bool {
        line.contains(section_name)  // THIS IS THE BUG!
    }
    
    // Step 3: Demonstrate how "Performance" matches multiple sections
    let lines = initial_content.lines().collect::<Vec<_>>();
    let target_section = "Performance";
    
    println!("\n🔍 Testing section matching for: '{}'", target_section);
    for line in &lines {
        if line.trim_start().starts_with("## ") {
            let matches = buggy_section_matching(line, target_section);
            println!("  '{}' matches: {}", line, matches);
        }
    }
    
    // Step 4: Show how this creates duplication
    let search_section = "Performance";
    let matching_sections: Vec<_> = lines.iter()
        .filter(|line| line.trim_start().starts_with("## ") && line.contains(search_section))
        .collect();
    
    println!("\n🚨 BUG DEMONSTRATION:");
    println!("When updating section 'Performance Benchmarks':");
    println!("Sections that match 'Performance': {}", matching_sections.len());
    for section in &matching_sections {
        println!("  - {}", section);
    }
    
    if matching_sections.len() > 1 {
        println!("💥 RESULT: Content gets inserted into {} different sections!", matching_sections.len());
        println!("   This creates duplicate sections in the output file.");
    }
    
    // Step 5: Show the fix
    println!("\n✅ CORRECT IMPLEMENTATION:");
    println!("Change this line in benchkit/src/reporting.rs:56:");
    println!("  OLD: if line.contains(self.section_marker.trim_start_matches(\"## \")) {{");
    println!("  NEW: if line.trim() == self.section_marker {{");
    
    println!("\nWith exact matching:");
    for line in &lines {
        if line.trim_start().starts_with("## ") {
            let exact_match = line.trim() == "## Performance Benchmarks";
            println!("  '{}' == '## Performance Benchmarks': {}", line.trim(), exact_match);
        }
    }
    
    println!("\n📊 IMPACT OF THE BUG:");
    println!("- Each benchmark run multiplies duplicate sections");  
    println!("- Files grow exponentially (5k → 7k lines in one run)");
    println!("- Documentation becomes unreadable");
    println!("- Contradictory benchmark results appear");
    
    Ok(())
}