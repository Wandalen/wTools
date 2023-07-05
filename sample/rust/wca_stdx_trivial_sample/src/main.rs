fn main() {
  let args = std::env::args().skip( 1 ).collect::< Vec< _ > >().join( " " );

  let aggregator = wca::stdx::cli( (  ) ).build();
  aggregator.perform( args ).unwrap();
}
