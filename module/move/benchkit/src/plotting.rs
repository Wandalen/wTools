//! Visualization and plotting utilities for benchmark results
//!
//! This module provides tools for generating performance charts and graphs
//! to enhance benchmark reports. Designed to work seamlessly with benchkit's
//! markdown-first reporting approach.

use crate::prelude::*;
use std::path::Path;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(feature = "visualization")]
use plotters::prelude::*;

/// Chart configuration for benchmark visualizations
#[derive(Debug, Clone)]
pub struct ChartConfig
{
  /// Chart title
  pub title: String,
  /// Chart width in pixels
  pub width: u32,
  /// Chart height in pixels  
  pub height: u32,
  /// X-axis label
  pub x_label: String,
  /// Y-axis label
  pub y_label: String,
  /// Output format
  pub format: ChartFormat,
}

impl Default for ChartConfig
{
  fn default() -> Self
  {
    Self
    {
      title: "Benchmark Results".to_string(),
      width: 800,
      height: 600,
      x_label: "Input Size".to_string(),
      y_label: "Operations/sec".to_string(),
      format: ChartFormat::SVG,
    }
  }
}

/// Supported chart output formats
#[derive(Debug, Clone)]
pub enum ChartFormat
{
  /// SVG format (best for markdown embedding)
  SVG,
  /// PNG format (best for documentation)
  PNG,
  /// HTML format (interactive charts)
  HTML,
}

impl ChartFormat
{
  /// Get file extension for this format
  pub fn extension(&self) -> &str
  {
    match self
    {
      ChartFormat::SVG => "svg",
      ChartFormat::PNG => "png", 
      ChartFormat::HTML => "html",
    }
  }
}

/// Performance scaling chart generator
#[cfg(feature = "visualization")]
#[derive(Debug)]
pub struct ScalingChart
{
  config: ChartConfig,
  data_series: Vec<(String, Vec<(f64, f64)>)>, // (name, (x, y) points)
}

#[cfg(feature = "visualization")]
impl ScalingChart
{
  /// Create new scaling chart
  pub fn new(config: ChartConfig) -> Self
  {
    Self
    {
      config,
      data_series: Vec::new(),
    }
  }
  
  /// Add data series to the chart
  pub fn add_series(&mut self, name: &str, data_points: Vec<(f64, f64)>)
  {
    self.data_series.push((name.to_string(), data_points));
  }
  
  /// Add benchmark results as a scaling series
  pub fn add_scaling_results(&mut self, name: &str, scaling_results: &[(usize, BenchmarkResult)])
  {
    let data_points: Vec<(f64, f64)> = scaling_results
      .iter()
      .map(|(scale, result)| (*scale as f64, result.operations_per_second()))
      .collect();
    
    self.add_series(name, data_points);
  }
  
  /// Generate the chart and save to file
  pub fn generate(&self, output_path: &Path) -> Result<()>
  {
    match self.config.format
    {
      ChartFormat::SVG => self.generate_svg(output_path),
      ChartFormat::PNG => self.generate_png(output_path),
      ChartFormat::HTML => self.generate_html(output_path),
    }
  }
  
  fn generate_svg(&self, output_path: &Path) -> Result<()>
  {
    let root = SVGBackend::new(output_path, (self.config.width, self.config.height))
      .into_drawing_area();
    self.draw_chart(root)
  }
  
  fn generate_png(&self, output_path: &Path) -> Result<()>
  {
    // For now, PNG support requires additional image processing library
    // Generate SVG instead and inform user
    let svg_path = output_path.with_extension("svg");
    self.generate_svg(&svg_path)?;
    println!("⚠️  PNG support requires image library - generated SVG instead: {:?}", svg_path);
    Ok(())
  }
  
  fn generate_html(&self, output_path: &Path) -> Result<()>
  {
    // For HTML, we'll generate SVG and embed it
    let svg_content = self.generate_svg_string()?;
    let html_content = format!(
      r#"<!DOCTYPE html>
<html>
<head>
  <title>{}</title>
  <style>
    body {{ font-family: Arial, sans-serif; margin: 40px; }}
    .chart-container {{ text-align: center; }}
  </style>
</head>
<body>
  <div class="chart-container">
    <h1>{}</h1>
    {}
  </div>
</body>
</html>"#,
      self.config.title, self.config.title, svg_content
    );
    
    std::fs::write(output_path, html_content)?;
    Ok(())
  }
  
  fn generate_svg_string(&self) -> Result<String>
  {
    let mut buffer = String::new();
    {
      let root = SVGBackend::with_string(&mut buffer, (self.config.width, self.config.height))
        .into_drawing_area();
      self.draw_chart(root)?;
    }
    
    Ok(buffer)
  }
  
  fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, plotters::coord::Shift>) -> Result<()>
  where
    DB::ErrorType: 'static,
  {
    root.fill(&WHITE)?;
    
    if self.data_series.is_empty()
    {
      return Ok(());
    }
    
    // Find data ranges
    let (x_min, x_max, y_min, y_max) = self.find_data_ranges();
    
    let mut chart = ChartBuilder::on(&root)
      .caption(&self.config.title, ("sans-serif", 30))
      .margin(20)
      .x_label_area_size(40)
      .y_label_area_size(50)
      .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    
    chart
      .configure_mesh()
      .x_desc(&self.config.x_label)
      .y_desc(&self.config.y_label)
      .draw()?;
    
    // Color palette for different series
    let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN];
    
    for (i, (name, data_points)) in self.data_series.iter().enumerate()
    {
      let color = colors[i % colors.len()];
      
      // Draw line
      chart
        .draw_series(LineSeries::new(data_points.iter().cloned(), color))?
        .label(name)
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
      
      // Draw points
      chart.draw_series(
        data_points.iter().map(|(x, y)| Circle::new((*x, *y), 3, color.filled()))
      )?;
    }
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    Ok(())
  }
  
  fn find_data_ranges(&self) -> (f64, f64, f64, f64)
  {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    
    for (_, data_points) in &self.data_series
    {
      for (x, y) in data_points
      {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        y_min = y_min.min(*y);
        y_max = y_max.max(*y);
      }
    }
    
    // Add some padding
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;
    
    (
      x_min - x_range * 0.05,
      x_max + x_range * 0.05,
      y_min - y_range * 0.05,
      y_max + y_range * 0.05,
    )
  }
}

/// Framework comparison bar chart generator
#[cfg(feature = "visualization")]
#[derive(Debug)]
pub struct ComparisonChart
{
  config: ChartConfig,
  data: Vec<(String, f64)>, // (framework_name, ops_per_second)
}

#[cfg(feature = "visualization")]
impl ComparisonChart
{
  /// Create new comparison chart
  pub fn new(config: ChartConfig) -> Self
  {
    Self
    {
      config,
      data: Vec::new(),
    }
  }
  
  /// Add framework performance data
  pub fn add_framework(&mut self, name: &str, ops_per_second: f64)
  {
    self.data.push((name.to_string(), ops_per_second));
  }
  
  /// Add benchmark results
  pub fn add_benchmark_results(&mut self, results: &[(String, BenchmarkResult)])
  {
    for (name, result) in results
    {
      self.add_framework(name, result.operations_per_second());
    }
  }
  
  /// Generate the chart
  pub fn generate(&self, output_path: &Path) -> Result<()>
  {
    match self.config.format
    {
      ChartFormat::SVG => self.generate_svg(output_path),
      ChartFormat::PNG => self.generate_png(output_path),
      ChartFormat::HTML => self.generate_html(output_path),
    }
  }
  
  fn generate_svg(&self, output_path: &Path) -> Result<()>
  {
    let root = SVGBackend::new(output_path, (self.config.width, self.config.height))
      .into_drawing_area();
    self.draw_chart(root)
  }
  
  fn generate_png(&self, output_path: &Path) -> Result<()>
  {
    // For now, PNG support requires additional image processing library
    // Generate SVG instead and inform user
    let svg_path = output_path.with_extension("svg");
    self.generate_svg(&svg_path)?;
    println!("⚠️  PNG support requires image library - generated SVG instead: {:?}", svg_path);
    Ok(())
  }
  
  fn generate_html(&self, output_path: &Path) -> Result<()>
  {
    let svg_content = self.generate_svg_string()?;
    let html_content = format!(
      r#"<!DOCTYPE html>
<html>
<head>
  <title>{}</title>
  <style>
    body {{ font-family: Arial, sans-serif; margin: 40px; }}
    .chart-container {{ text-align: center; }}
  </style>
</head>
<body>
  <div class="chart-container">
    <h1>{}</h1>
    {}
  </div>
</body>
</html>"#,
      self.config.title, self.config.title, svg_content
    );
    
    std::fs::write(output_path, html_content)?;
    Ok(())
  }
  
  fn generate_svg_string(&self) -> Result<String>
  {
    let mut buffer = String::new();
    {
      let root = SVGBackend::with_string(&mut buffer, (self.config.width, self.config.height))
        .into_drawing_area();
      self.draw_chart(root)?;
    }
    
    Ok(buffer)
  }
  
  fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, plotters::coord::Shift>) -> Result<()>
  where
    DB::ErrorType: 'static,
  {
    root.fill(&WHITE)?;
    
    if self.data.is_empty()
    {
      return Ok(());
    }
    
    let y_max = self.data.iter().map(|(_, ops)| *ops).fold(0.0f64, f64::max) * 1.1;
    
    let mut chart = ChartBuilder::on(&root)
      .caption(&self.config.title, ("sans-serif", 30))
      .margin(20)
      .x_label_area_size(60)
      .y_label_area_size(50)
      .build_cartesian_2d(0f32..self.data.len() as f32, 0f64..y_max)?;
    
    chart
      .configure_mesh()
      .y_desc(&self.config.y_label)
      .x_desc(&self.config.x_label)
      .x_label_formatter(&|x| {
        let index = *x as usize;
        if index < self.data.len() {
          self.data[index].0.clone()
        } else {
          String::new()
        }
      })
      .draw()?;
    
    // Draw bars
    chart.draw_series(
      self.data.iter().enumerate().map(|(i, (_name, ops))| {
        Rectangle::new([(i as f32 - 0.4, 0.0), (i as f32 + 0.4, *ops)], BLUE.filled())
      })
    )?
    .label("Operations/sec")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    Ok(())
  }
}

/// High-level plotting functions for common benchkit use cases
#[cfg(feature = "visualization")]
pub mod plots
{
  use super::*;
  
  /// Generate scaling analysis chart from benchmark suite results
  pub fn scaling_analysis_chart(
    suite_results: &[(usize, BenchmarkResult)],
    title: &str,
    output_path: &Path,
  ) -> Result<()>
  {
    let config = ChartConfig
    {
      title: title.to_string(),
      x_label: "Input Size".to_string(),
      y_label: "Operations/sec".to_string(),
      ..Default::default()
    };
    
    let mut chart = ScalingChart::new(config);
    chart.add_scaling_results("Performance", suite_results);
    chart.generate(output_path)
  }
  
  /// Generate framework comparison chart
  pub fn framework_comparison_chart(
    framework_results: &[(String, BenchmarkResult)],
    title: &str,
    output_path: &Path,
  ) -> Result<()>
  {
    let config = ChartConfig
    {
      title: title.to_string(),
      x_label: "Framework".to_string(),
      y_label: "Operations/sec".to_string(),
      ..Default::default()
    };
    
    let mut chart = ComparisonChart::new(config);
    chart.add_benchmark_results(framework_results);
    chart.generate(output_path)
  }
  
  /// Generate performance trend chart over time
  pub fn performance_trend_chart(
    historical_data: &[(String, f64)], // (date/commit, ops_per_second)
    title: &str,
    output_path: &Path,
  ) -> Result<()>
  {
    let config = ChartConfig
    {
      title: title.to_string(),
      x_label: "Time".to_string(),
      y_label: "Operations/sec".to_string(),
      ..Default::default()
    };
    
    let mut chart = ScalingChart::new(config);
    let data_points: Vec<(f64, f64)> = historical_data
      .iter()
      .enumerate()
      .map(|(i, (_, ops))| (i as f64, *ops))
      .collect();
    
    chart.add_series("Performance Trend", data_points);
    chart.generate(output_path)
  }
}

// Stubs for when visualization feature is disabled
#[cfg(not(feature = "visualization"))]
pub mod plots
{
  use super::*;
  
  /// Scaling analysis chart (disabled - enable 'visualization' feature)
  pub fn scaling_analysis_chart(
    _suite_results: &[(usize, BenchmarkResult)],
    _title: &str,
    _output_path: &Path,
  ) -> Result<()>
  {
    println!("⚠️  Visualization disabled - enable 'visualization' feature for charts");
    Ok(())
  }
  
  /// Framework comparison chart (disabled - enable 'visualization' feature)
  pub fn framework_comparison_chart(
    _framework_results: &[(String, BenchmarkResult)],
    _title: &str,
    _output_path: &Path,
  ) -> Result<()>
  {
    println!("⚠️  Visualization disabled - enable 'visualization' feature for charts");
    Ok(())
  }
  
  /// Performance trend chart (disabled - enable 'visualization' feature)
  pub fn performance_trend_chart(
    _historical_data: &[(String, f64)],
    _title: &str,
    _output_path: &Path,
  ) -> Result<()>
  {
    println!("⚠️  Visualization disabled - enable 'visualization' feature for charts");
    Ok(())
  }
}

