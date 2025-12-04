//! Practical CLI Aggregation: Real-World Usage Patterns
//!
//! This example demonstrates practical CLI aggregation scenarios that you might
//! encounter in real-world software development, including:
//! 1. DevOps tool aggregation (Docker, Kubernetes, CI/CD)
//! 2. Development tool aggregation (Git, Build, Test, Deploy)
//! 3. Database administration tool aggregation
//! 4. Cloud services management aggregation
//!
//! Each scenario shows how unilang's aggregation features solve common pain points
//! in tool management and provide unified command interfaces.

use unilang::prelude::*;

// =============================================================================
// DevOps Tools Aggregation - Container and Orchestration Management
// =============================================================================

/// Create DevOps tool commands for container and orchestration management
fn create_devops_commands() -> Vec<CommandDefinition>
{
  vec![
    // Docker commands
    CommandDefinition::former()
      .name( ".list" )
      .namespace( ".container" )
      .description( "List all containers".to_string() )
      .hint( "Docker container listing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "all" )
          .description( "Show all containers (default shows just running)".to_string() )
          .kind( Kind::Boolean )
          .hint( "Include stopped containers".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".logs" )
      .namespace( ".container" )
      .description( "Get container logs".to_string() )
      .hint( "Docker container logs".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "container" )
          .description( "Container name or ID".to_string() )
          .kind( Kind::String )
          .hint( "Container identifier".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "follow" )
          .description( "Follow log output".to_string() )
          .kind( Kind::Boolean )
          .hint( "Stream logs continuously".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "false".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    // Kubernetes commands
    CommandDefinition::former()
      .name( ".list" )
      .namespace( ".pod" )
      .description( "List Kubernetes pods".to_string() )
      .hint( "Kubernetes pod listing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "namespace" )
          .description( "Kubernetes namespace".to_string() )
          .kind( Kind::String )
          .hint( "Target namespace".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "default".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".scale" )
      .namespace( ".deployment" )
      .description( "Scale Kubernetes deployment".to_string() )
      .hint( "Kubernetes deployment scaling".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "deployment" )
          .description( "Deployment name".to_string() )
          .kind( Kind::String )
          .hint( "Deployment identifier".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "replicas" )
          .description( "Number of replicas".to_string() )
          .kind( Kind::Integer )
          .hint( "Replica count".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
  ]
}

// =============================================================================
// Development Tools Aggregation - Git, Build, Test, Deploy
// =============================================================================

/// Create development tool commands for version control and CI/CD
fn create_development_commands() -> Vec<CommandDefinition>
{
  vec![
    // Git commands
    CommandDefinition::former()
      .name( ".create" )
      .namespace( ".branch" )
      .description( "Create and switch to new branch".to_string() )
      .hint( "Git branch creation".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "name" )
          .description( "Branch name".to_string() )
          .kind( Kind::String )
          .hint( "New branch name".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "from" )
          .description( "Base branch".to_string() )
          .kind( Kind::String )
          .hint( "Source branch".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "main".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".squash" )
      .namespace( ".commit" )
      .description( "Squash last N commits".to_string() )
      .hint( "Git commit squashing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "count" )
          .description( "Number of commits to squash".to_string() )
          .kind( Kind::Integer )
          .hint( "Commit count".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "message" )
          .description( "Squash commit message".to_string() )
          .kind( Kind::String )
          .hint( "New commit message".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
    // Build commands
    CommandDefinition::former()
      .name( ".release" )
      .namespace( ".build" )
      .description( "Build release artifacts".to_string() )
      .hint( "Release build".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "target" )
          .description( "Build target".to_string() )
          .kind( Kind::String )
          .hint( "Target platform".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "x86_64-unknown-linux-gnu".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".integration" )
      .namespace( ".test" )
      .description( "Run integration tests".to_string() )
      .hint( "Integration testing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "parallel" )
          .description( "Run tests in parallel".to_string() )
          .kind( Kind::Boolean )
          .hint( "Parallel execution".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "true".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
  ]
}

// =============================================================================
// Database Administration Aggregation
// =============================================================================

/// Create database administration commands for multiple database systems
fn create_database_commands() -> Vec<CommandDefinition>
{
  vec![
    // PostgreSQL commands
    CommandDefinition::former()
      .name( ".backup" )
      .namespace( ".postgres" )
      .description( "Create PostgreSQL database backup".to_string() )
      .hint( "PostgreSQL backup".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "database" )
          .description( "Database name".to_string() )
          .kind( Kind::String )
          .hint( "Target database".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "output" )
          .description( "Backup file path".to_string() )
          .kind( Kind::Path )
          .hint( "Output file".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".restore" )
      .namespace( ".postgres" )
      .description( "Restore PostgreSQL database from backup".to_string() )
      .hint( "PostgreSQL restore".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "database" )
          .description( "Target database name".to_string() )
          .kind( Kind::String )
          .hint( "Target database".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "backup_file" )
          .description( "Backup file path".to_string() )
          .kind( Kind::File )
          .hint( "Backup file".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
    // Redis commands
    CommandDefinition::former()
      .name( ".monitor" )
      .namespace( ".redis" )
      .description( "Monitor Redis commands in real-time".to_string() )
      .hint( "Redis monitoring".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "host" )
          .description( "Redis host".to_string() )
          .kind( Kind::String )
          .hint( "Redis server".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "localhost".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
        ArgumentDefinition::former()
          .name( "port" )
          .description( "Redis port".to_string() )
          .kind( Kind::Integer )
          .hint( "Redis port".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "6379".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
  ]
}

// =============================================================================
// Cloud Services Management Aggregation
// =============================================================================

/// Create cloud management commands for AWS, Azure, and GCP
fn create_cloud_commands() -> Vec<CommandDefinition>
{
  vec![
    // AWS commands
    CommandDefinition::former()
      .name( ".list" )
      .namespace( ".ec2" )
      .description( "List EC2 instances".to_string() )
      .hint( "AWS EC2 listing".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "region" )
          .description( "AWS region".to_string() )
          .kind( Kind::String )
          .hint( "Target region".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "us-east-1".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
        ArgumentDefinition::former()
          .name( "state" )
          .description( "Instance state filter".to_string() )
          .kind( Kind::String )
          .hint( "Instance state".to_string() )
          .attributes( ArgumentAttributes {
            optional: true,
            default: Some( "running".to_string() ),
            ..ArgumentAttributes::default()
          } )
          .end(),
      ])
      .end(),
    CommandDefinition::former()
      .name( ".sync" )
      .namespace( ".s3" )
      .description( "Sync files with S3 bucket".to_string() )
      .hint( "AWS S3 sync".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "source" )
          .description( "Source path".to_string() )
          .kind( Kind::Path )
          .hint( "Local path".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "bucket" )
          .description( "S3 bucket name".to_string() )
          .kind( Kind::String )
          .hint( "Target bucket".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
    // Azure commands
    CommandDefinition::former()
      .name( ".status" )
      .namespace( ".vm" )
      .description( "Get Azure VM status".to_string() )
      .hint( "Azure VM status".to_string() )
      .arguments( vec![
        ArgumentDefinition::former()
          .name( "resource_group" )
          .description( "Resource group name".to_string() )
          .kind( Kind::String )
          .hint( "Resource group".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
        ArgumentDefinition::former()
          .name( "vm_name" )
          .description( "Virtual machine name".to_string() )
          .kind( Kind::String )
          .hint( "VM name".to_string() )
          .attributes( ArgumentAttributes::default() )
          .end(),
      ])
      .end(),
  ]
}

// =============================================================================
// Practical Aggregation Demonstration
// =============================================================================

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), unilang::Error>
{
  println!( "🏗️ Practical CLI Aggregation: Real-World Usage Patterns" );
  println!();

  // Scenario 1: DevOps Engineer's Unified CLI
  println!( "📊 Scenario 1: DevOps Engineer's Unified CLI" );
  println!( "   Problem: Managing Docker, Kubernetes, CI/CD tools separately" );
  println!( "   Solution: Unified DevOps CLI with consistent interface" );
  println!();

  let devops_cli = CliBuilder::new()
    .app_name( "devops_unified" )
    .static_module_with_prefix( "devops", "ops", create_devops_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build()?;

  println!( "   DevOps commands available:" );
  for (name, _cmd) in devops_cli.commands()
  {
    if name.starts_with( ".ops." )
    {
      println!( "     {name}" );
    }
  }
  println!();

  // Scenario 2: Developer's Workflow CLI
  println!( "💻 Scenario 2: Developer's Workflow CLI" );
  println!( "   Problem: Switching between Git, build tools, testing frameworks" );
  println!( "   Solution: Integrated development workflow CLI" );
  println!();

  let dev_cli = CliBuilder::new()
    .app_name( "dev_workflow" )
    .static_module_with_prefix( "development", "dev", create_development_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build()?;

  println!( "   Development commands available:" );
  for (name, _cmd) in dev_cli.commands()
  {
    if name.starts_with( ".dev." )
    {
      println!( "     {name}" );
    }
  }
  println!();

  // Scenario 3: Database Administrator's CLI
  println!( "🗄️ Scenario 3: Database Administrator's CLI" );
  println!( "   Problem: Managing PostgreSQL, Redis, MongoDB with different tools" );
  println!( "   Solution: Unified database administration interface" );
  println!();

  let dba_cli = CliBuilder::new()
    .app_name( "dba_tools" )
    .static_module_with_prefix( "database", "db", create_database_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build()?;

  println!( "   Database administration commands:" );
  for (name, _cmd) in dba_cli.commands()
  {
    if name.starts_with( ".svc1." )
    {
      println!( "     {name}" );
    }
  }
  println!();

  // Scenario 4: Cloud Operations CLI
  println!( "☁️ Scenario 4: Cloud Operations CLI" );
  println!( "   Problem: Managing AWS, Azure, GCP with separate CLIs" );
  println!( "   Solution: Multi-cloud unified management interface" );
  println!();

  let cloud_cli = CliBuilder::new()
    .app_name( "cloud_ops" )
    .static_module_with_prefix( "cloud", "cloud", create_cloud_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build()?;

  println!( "   Cloud management commands:" );
  for (name, _cmd) in cloud_cli.commands()
  {
    if name.starts_with( ".cloud." )
    {
      println!( "     {name}" );
    }
  }
  println!();

  // Scenario 5: Ultimate Unified CLI - All tools combined
  println!( "🚀 Scenario 5: Ultimate Unified CLI - All Tools Combined" );
  println!( "   Problem: Context switching between multiple specialized CLIs" );
  println!( "   Solution: Single CLI for all development and operations tasks" );
  println!();

  let unified_cli = CliBuilder::new()
    .app_name( "unified_platform" )
    .global_prefix( "platform" )
    .static_module_with_prefix( "devops", "ops", create_devops_commands() )
    .static_module_with_prefix( "development", "dev", create_development_commands() )
    .static_module_with_prefix( "database", "db", create_database_commands() )
    .static_module_with_prefix( "cloud", "cloud", create_cloud_commands() )
    .auto_help( true )
    .detect_conflicts( true )
    .build()?;

  println!( "   Unified platform commands (total: {}):", unified_cli.commands().len() );

  let mut categories = std::collections::HashMap::new();
  for (name, _cmd) in unified_cli.commands()
  {
    if let Some(category) = name.split('.').nth(2)
    {
      *categories.entry(category.to_string()).or_insert(0) += 1;
    }
  }

  for (category, count) in &categories
  {
    println!( "     {category}: {count} commands" );
  }
  println!();

  // Benefits Summary
  println!( "✨ Practical Benefits Demonstrated:" );
  println!( "   🎯 Single Command Interface: One CLI to rule them all" );
  println!( "   🏷️ Consistent Naming: Predictable command patterns across tools" );
  println!( "   🔄 Context Switching: No need to remember different CLI syntaxes" );
  println!( "   📚 Unified Help: Single help system for all tools" );
  println!( "   🔍 Command Discovery: Easy to explore available functionality" );
  println!( "   ⚡ Performance: Zero runtime overhead for command resolution" );
  println!( "   🛡️ Type Safety: Consistent argument validation across all tools" );
  println!( "   🏗️ Maintainability: Centralized command management and updates" );
  println!();

  println!( "🎉 Real-world problems solved:" );
  println!( "   ✅ Tool proliferation and context switching overhead" );
  println!( "   ✅ Inconsistent command interfaces across tools" );
  println!( "   ✅ Training overhead for new team members" );
  println!( "   ✅ Documentation scattered across multiple tools" );
  println!( "   ✅ Difficulty in automation due to CLI inconsistencies" );

  Ok( () )
}