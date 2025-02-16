
/// Define a private namespace for all its items.
mod private
{

  use crate::*;
  pub use iter_tools::{ _IterTrait, IterTrait, BoxedIter };

  use std::
  {
    hash::Hash,
    fmt,
  };

//   /// Represent directed graph. Can be zero-sized structure if nodes own all the information.
//   pub trait GraphDirected< 'a >
//   {
//     /// Uniquely identify a node.
//     type NodeId : NodeId;
//     /// Node itself.
//     type Node : Node + 'a;
//
//     /// Get a reference on a node by its id.
//     fn node_ref( &'a self, node_id : Self::NodeId ) -> &'a Self::Node;
//     /// Get id by its node reference.
//     fn node_id( &self, node_id : &'a Self::Node ) -> Self::NodeId;
//
//     /// Iterate over out nodes of
//     fn node_out_nodes( &'a self, node_id : Self::NodeId ) -> BoxedIter< 'a, Self::NodeId >;
//
//   }

  /// Print directed graph as a tree.
  pub trait GraphDirectedPrintAsTree< 'g >
  where
    Self : abs::GraphDirected< 'g >,
  {

    /// Write a graph into foromat stream with all nodes traversed by DFS.
    fn write_as_dfs_tree< 'w >( &'g self, write : &'w mut ( dyn core::fmt::Write + 'w ), node_id : Self::NodeId ) -> fmt::Result
    {
      #![ allow( non_upper_case_globals ) ]
      use iter_tools::Itertools;
      const up_down : &str = "│  ";
      const up_down_right : &str = "├─ ";
      // const _left_right : &str = "─";
      // const _down_right : &str = "┌─";

      let mut visited = collection_tools::HashSet::new();
      let mut stack = collection_tools::Vec::new();

      let prefix = | level : isize |
      {
        let left = if level > 0
        {
          std::iter::repeat( up_down ).take( ( level - 1 ) as usize ).join( " " )
        }
        else
        {
          String::new()
        };
        let right = if level > 0
        {
          up_down_right
        }
        else
        {
          &String::new()
        };
        return format!( "{}{}", left, right );
      };

      let push = | stack : &mut collection_tools::Vec< ( Self::NodeId, isize, bool ) >, node_id, level, preorder |
      {
        // println!( "push {:?} level:{} preorder:{}", node_id, level, if preorder { 1 } else { 0 } );
        stack.push( ( node_id, level, preorder ) );
      };

      push( &mut stack, node_id, 0, true );

      while let Some( ( node_id, level, _preorder ) ) = stack.pop()
      {
        // if !preorder
        // {
        //   write.write_fmt( format_args!( "{}{:?}\n", prefix( level ), node_id ) )?;
        //   continue;
        // }

        if visited.insert( node_id )
        {
          // push( &mut stack, node_id, level, false );
          write.write_fmt( format_args!( "{}{:?}\n", prefix( level ), node_id ) )?;

          for child_id in self.node_out_nodes( node_id ).rev()
          {
            push( &mut stack, child_id, level + 1, true );
          }
        }
      }

      return Ok( () )
    }

    /// Represent a graph as a string with all nodes traversed by DFS.
    fn string_with_dfs_tree< 'w >( &'g self, node : Self::NodeId ) -> String
    {
      // let node = self.node_ref( node );
      let mut result = String::new();
      self.write_as_dfs_tree( &mut result, node ).unwrap();
      result
    }

    /// Write a graph into foromat stream with all nodes traversed by BFS.
    fn write_as_bfs_tree< 'w >( &'g self, write : &'w mut ( dyn core::fmt::Write + 'w ), node_id : Self::NodeId ) -> fmt::Result
    {
      #![ allow( non_upper_case_globals ) ]
      use iter_tools::Itertools;
      const up_down : &str = "│  ";
      const up_down_right : &str = "├─ ";
      // const _left_right : &str = "─";
      // const _down_right : &str = "┌─";

      let mut level : isize = -1;
      let mut visited = collection_tools::HashSet::new();
      let mut stack = collection_tools::Vec::new();
      let mut next = collection_tools::Vec::new();

      let prefix = | level : isize |
      {
        let left = if level > 0
        {
          std::iter::repeat( up_down ).take( ( level - 1 ) as usize ).join( " " )
        }
        else
        {
          String::new()
        };
        let right = if level > 0
        {
          up_down_right
        }
        else
        {
          &String::new()
        };
        return format!( "{}{}", left, right );
      };

      let push = | next : &mut collection_tools::Vec< Self::NodeId >, node_id |
      {
        // println!( "push {:?}", node_id );
        next.push( node_id );
      };

      push( &mut next, node_id );

      while next.len() > 0
      {

        core::mem::swap( &mut stack, &mut next );
        next.clear();
        level += 1;

        while let Some( node_id ) = stack.pop()
        {

          if visited.insert( node_id )
          {
            write.write_fmt( format_args!( "{}{:?}\n", prefix( level ), node_id ) )?;

            for child_id in self.node_out_nodes( node_id )
            {
              push( &mut next, child_id );
            }
          }

        }

      }
      return Ok( () )
    }

    /// Represent a graph as a string with all nodes traversed by BFS.
    fn string_with_bfs_tree< 'w >( &'g self, node : Self::NodeId ) -> String
    {
      // let node = self.node_ref( node );
      let mut result = String::new();
      self.write_as_bfs_tree( &mut result, node ).unwrap();
      result
    }

  }

  impl< 'g, T > GraphDirectedPrintAsTree< 'g > for T
  where
    Self : abs::GraphDirected< 'g >,
  {
  }

  // impl fmt::Debug for Context< '_ >
  // {
  //   fn fmt( &self, c : &mut fmt::Formatter< '_ > ) -> fmt::Result
  //   {
  //     c
  //     .debug_struct( "Context" )
  //     .field( "buf", &"dyn fmt::Write" )
  //     .field( "printer", &self.printer )
  //     .finish()
  //   }
  // }

}

crate::mod_interface!
{
  own use
  {
    GraphDirectedPrintAsTree,
  };
}
