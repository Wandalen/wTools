#[ derive( Debug, Clone ) ] 
pub struct NelderMeadOptimizer< F >
where F : Fn( Vec< f64 > ) -> f64
{
  pub f : F,
  pub x0 : Vec< f64 >,
  pub step : f64,
  pub improvement_threshold : f64,
  pub max_iterations : usize,
  pub max_no_improvment_steps : usize,
  pub alpha : f64,
  pub gamma : f64,
  pub rho : f64,
  pub sigma : f64,
}

impl< F > NelderMeadOptimizer< F >
where F : Fn( Vec< f64 > ) -> f64
{
  pub fn optimize( &self ) -> ( Vec< f64 >, f64 )
  {
    let dimensions = self.x0.len();
    let mut prev_best = ( self.f )( self.x0.clone() );
    let mut steps_with_no_improv = 0;
    let mut res = vec![ ( self.x0.clone(), prev_best ) ];

    for i in 0..dimensions
    {
      let mut x = self.x0.clone();
      x[ i ] += self.step;
      let score = ( self.f )( x.clone() );
      res.push( ( x, score ) );
    }

    let mut iterations = 0;
    loop
    {
      res.sort_by( | ( _, a ), ( _, b ) | a.total_cmp( b ) );
      // println!("array : {:?}", res );
      let best = res.first().clone().unwrap();

      if self.max_iterations <= iterations
      {
        return res[ 0 ].clone();
      }

      iterations += 1;

      if best.1 < prev_best - self.improvement_threshold
      {
        steps_with_no_improv = 0;
        prev_best = best.1;
      }
      else
      {
        steps_with_no_improv += 1;   
      }

      if steps_with_no_improv >= self.max_no_improvment_steps
      {
        return res[ 0 ].clone();
      }

      //centroid
      let mut x0_center = vec![ 0.0; dimensions ];
      for ( point, _ ) in res.iter().take( res.len() - 1 )
      {
        for ( i, coordinate ) in point.iter().enumerate()
        {
          x0_center[ i ] += coordinate / ( res.len() - 1 ) as f64;
        }
      }

      //reflection
      let worst_dir = res.last().clone().unwrap();
      let mut x_ref = vec![ 0.0; dimensions ];
      for i in 0..dimensions
      {
        x_ref[ i ] = x0_center[ i ] + self.alpha * ( x0_center[ i ] - worst_dir.0[ i ] );
      }
    
      let reflection_score = ( self.f )( x_ref.clone() );
      let second_worst = res[ res.len() - 2 ].1;
      if res.first().clone().unwrap().1 <= reflection_score && reflection_score < second_worst
      {
        res.pop();
        res.push( ( x_ref, reflection_score ) );
        continue;
      }

      //expansion
      if reflection_score < res.first().clone().unwrap().1
      {
        let mut x_exp = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_exp[ i ] = x0_center[ i ] + self.gamma * ( x0_center[ i ] - worst_dir.0[ i ] );
        }
        let expansion_score = ( self.f )( x_exp.clone() );

        if expansion_score < reflection_score
        {
          res.pop();
          res.push( ( x_exp, expansion_score ) );
          continue;
        }
        else 
        {
          res.pop();
          res.push( ( x_ref, reflection_score ) );
          continue;
        }
      }

      //contraction
      let mut x_con = vec![ 0.0; dimensions ];
      for i in 0..dimensions
      {
        x_con[ i ] = x0_center[ i ] + self.rho * ( x0_center[ i ] - worst_dir.0[ i ] );
      }
      let contraction_score = ( self.f )( x_con.clone() );

      if contraction_score < worst_dir.1
      {
        res.pop();
        res.push( ( x_con, contraction_score ) );
        continue;
      }

      //shrink
      let x1 = res[ 0 ].clone().0;
      let mut new_res = Vec::new();
      for ( point, _ ) in res
      {
        let mut x_shrink = vec![ 0.0; dimensions ];
        for i in 0..dimensions
        {
          x_shrink[ i ] = x1[ i ] + self.sigma * ( point[ i ] - x1[ i ] );
        }
        let score = ( self.f )( x_shrink.clone() );
        new_res.push( ( x_shrink, score ) );
      }

      res = new_res;
    }
  }
}



