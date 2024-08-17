//!
//! Print data as table.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use crate::*;
  use std::
  {
    borrow::Cow,
    collections::HashMap,
  };
  use core::
  {
    fmt,
  };
  use former::Former;

  //=

  /// A struct to configure options for printing data as a table.
  ///
  /// The `Styles` struct provides customizable delimiters for formatting table data. It allows
  /// you to define how table data should be separated and formatted, making it adaptable to
  /// various needs.
  ///
  /// # Fields
  ///
  /// - `cell_separator`: A `String` that specifies the delimiter used to separate columns
  ///   within a table. This is the character or string that separates each column.
  ///
  /// - `row_prefix`: A `String` that specifies the prefix added to each row. This can be
  ///   used to add a consistent start to each row.
  ///
  /// - `row_postfix`: A `String` that specifies the postfix added to each row. This can be
  ///   used to add a consistent end to each row.
  ///
  /// - `row_postfix`: A `String` that specifies the postfix added to each row. This can be
  ///   used to add a consistent end to each row.
  ///
  /// ```
  #[ derive( Debug ) ]
  #[ debug ]
  pub struct Styles
  {

    /// Delimiter for adding prefix to a cell.
    pub cell_prefix : String,
    /// Delimiter for adding postfix to a cell.
    pub cell_postfix : String,
    /// Delimiter for separating table columns.
    pub cell_separator : String,

    /// Delimiter for adding prefix to a row.
    pub row_prefix : String,
    /// Delimiter for adding postfix to a row.
    pub row_postfix : String,
    /// Delimiter for adding in between of rows.
    pub row_separator : String,

  }

  // xxx

#[automatically_derived]
impl<> Styles<> where
{
  #[doc = r""]
  #[doc = r" Provides a mechanism to initiate the formation process with a default completion behavior."]
  #[doc = r""]
  #[inline(always)]
  pub fn former() -> StylesFormer<StylesFormerDefinition<(), Styles<>, former::ReturnPreformed>>
  {
    StylesFormer::<StylesFormerDefinition<(), Styles<>, former::ReturnPreformed>>::new_coercing(former::ReturnPreformed)
  }
}

impl<Definition> former::EntityToFormer<Definition> for Styles<>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>>,
{
  type Former = StylesFormer<Definition>;
}

impl<> former::EntityToStorage for Styles<>
where
{
  type Storage = StylesFormerStorage<>;
}

impl<__Context, __Formed, __End> former::EntityToDefinition<__Context, __Formed, __End> for Styles<>
where
  __End: former::FormingEnd<StylesFormerDefinitionTypes<__Context, __Formed>>,
{
  type Definition = StylesFormerDefinition<__Context, __Formed, __End>;
  type Types = StylesFormerDefinitionTypes<__Context, __Formed>;
}

impl<__Context, __Formed> former::EntityToDefinitionTypes<__Context, __Formed> for Styles<>
where
{
  type Types = StylesFormerDefinitionTypes<__Context, __Formed>;
}

#[doc = r" Defines the generic parameters for formation behavior including context, form, and end conditions."]
#[derive(Debug)]
pub struct StylesFormerDefinitionTypes<__Context = (), __Formed = Styles<>>
where
{
  _phantom: ::core::marker::PhantomData<(*const __Context, *const __Formed)>,
}

impl<__Context, __Formed> ::core::default::Default for StylesFormerDefinitionTypes<__Context, __Formed>
where
{
  fn default() -> Self
  {
    Self { _phantom: ::core::marker::PhantomData }
  }
}

impl<__Context, __Formed> former::FormerDefinitionTypes for StylesFormerDefinitionTypes<__Context, __Formed>
where
{
  type Storage = StylesFormerStorage<>;
  type Formed = __Formed;
  type Context = __Context;
}

#[doc = r" Holds the definition types used during the formation process."]
#[derive(Debug)]
pub struct StylesFormerDefinition<__Context = (), __Formed = Styles<>, __End = former::ReturnPreformed>
where
{
  _phantom: ::core::marker::PhantomData<(*const __Context, *const __Formed, *const __End)>,
}

impl<__Context, __Formed, __End> ::core::default::Default for StylesFormerDefinition<__Context, __Formed, __End>
where
{
  fn default() -> Self
  {
    Self { _phantom: ::core::marker::PhantomData }
  }
}

impl<__Context, __Formed, __End> former::FormerDefinition for StylesFormerDefinition<__Context, __Formed, __End>
where
  __End: former::FormingEnd<StylesFormerDefinitionTypes<__Context, __Formed>>,
{
  type Types = StylesFormerDefinitionTypes<__Context, __Formed>;
  type End = __End;
  type Storage = StylesFormerStorage<>;
  type Formed = __Formed;
  type Context = __Context;
}

impl<__Context, __Formed> former::FormerMutator for StylesFormerDefinitionTypes<__Context, __Formed>
where
{}

#[doc = "Stores potential values for fields during the formation process."]
#[allow(explicit_outlives_requirements)]
pub struct StylesFormerStorage<>
where
{
  #[doc = r" A field"]
  pub cell_prefix: ::core::option::Option<String>,
  #[doc = r" A field"]
  pub cell_postfix: ::core::option::Option<String>,
  #[doc = r" A field"]
  pub cell_separator: ::core::option::Option<String>,
  #[doc = r" A field"]
  pub row_prefix: ::core::option::Option<String>,
  #[doc = r" A field"]
  pub row_postfix: ::core::option::Option<String>,
  #[doc = r" A field"]
  pub row_separator: ::core::option::Option<String>,
}

impl<> ::core::default::Default for StylesFormerStorage<>
where
{
  #[inline(always)]
  fn default() -> Self
  {
    Self
    {
      cell_prefix: ::core::option::Option::None,
      cell_postfix: ::core::option::Option::None,
      cell_separator: ::core::option::Option::None,
      row_prefix: ::core::option::Option::None,
      row_postfix: ::core::option::Option::None,
      row_separator: ::core::option::Option::None,
    }
  }
}

impl<> former::Storage for StylesFormerStorage<>
where
{
  type Preformed = Styles<>;
}

impl<> former::StoragePreform for StylesFormerStorage<>
where
{
  fn preform(mut self) -> Self::Preformed
  {
    let cell_prefix = if self.cell_prefix.is_some()
    {
      self.cell_prefix.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'cell_prefix' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let cell_postfix = if self.cell_postfix.is_some()
    {
      self.cell_postfix.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'cell_postfix' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let cell_separator = if self.cell_separator.is_some()
    {
      self.cell_separator.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'cell_separator' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let row_prefix = if self.row_prefix.is_some()
    {
      self.row_prefix.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'row_prefix' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let row_postfix = if self.row_postfix.is_some()
    {
      self.row_postfix.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'row_postfix' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let row_separator = if self.row_separator.is_some()
    {
      self.row_separator.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'row_separator' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<String>).maybe_default()
      }
    };
    let result = Styles::<>
    {
      cell_prefix,
      cell_postfix,
      cell_separator,
      row_prefix,
      row_postfix,
      row_separator,
    };
    return result;
  }
}

#[doc = "\nStructure to form [Styles]. Represents a forming entity designed to construct objects through a builder pattern.\n\nThis structure holds temporary storage and context during the formation process and\nutilizes a defined end strategy to finalize the object creation.\n"]
pub struct StylesFormer<Definition = StylesFormerDefinition<(), Styles<>, former::ReturnPreformed>>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>>,
  Definition::Types: former::FormerDefinitionTypes<Storage = StylesFormerStorage<>>,
{
  #[doc = r" Temporary storage for all fields during the formation process. It contains"]
  #[doc = r"   partial data that progressively builds up to the final object."]
  pub storage: Definition::Storage,
  #[doc = r" An optional context providing additional data or state necessary for custom"]
  #[doc = r"   formation logic or to facilitate this former's role as a subformer within another former."]
  pub context: ::core::option::Option<Definition::Context>,
  #[doc = r" An optional closure or handler that is invoked to transform the accumulated"]
  #[doc = r"   temporary storage into the final object structure once formation is complete."]
  pub on_end: ::core::option::Option<Definition::End>,
}

#[automatically_derived]
impl<Definition> StylesFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>>,
  Definition::Types: former::FormerDefinitionTypes<Storage = StylesFormerStorage<>>,
{
  #[doc = r""]
  #[doc = r" Initializes a former with an end condition and default storage."]
  #[doc = r""]
  #[inline(always)]
  pub fn new(on_end: Definition::End) -> Self
  {
    Self::begin_coercing(None, None, on_end)
  }

  #[doc = r""]
  #[doc = r" Initializes a former with a coercible end condition."]
  #[doc = r""]
  #[inline(always)]
  pub fn new_coercing<IntoEnd>(end: IntoEnd) -> Self
  where
    IntoEnd: Into<Definition::End>,
  {
    Self::begin_coercing(None, None, end)
  }

  #[doc = r""]
  #[doc = r" Begins the formation process with specified context and termination logic."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin(
    mut storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: <Definition as former::FormerDefinition>::End,
  ) -> Self
  {
    if storage.is_none()
    {
      storage = Some(::core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: ::core::option::Option::Some(on_end),
    }
  }

  #[doc = r""]
  #[doc = r" Starts the formation process with coercible end condition and optional initial values."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin_coercing<IntoEnd>(
    mut storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: IntoEnd,
  ) -> Self
  where
    IntoEnd: ::core::convert::Into<<Definition as former::FormerDefinition>::End>,
  {
    if storage.is_none()
    {
      storage = Some(::core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: ::core::option::Option::Some(::core::convert::Into::into(on_end)),
    }
  }

  #[doc = r""]
  #[doc = r" Wrapper for `end` to align with common builder pattern terminologies."]
  #[doc = r""]
  #[inline(always)]
  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    self.end()
  }

  #[doc = r""]
  #[doc = r" Completes the formation and returns the formed object."]
  #[doc = r""]
  #[inline(always)]
  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let mut context = self.context.take();
    <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  #[doc = "Scalar setter for the 'cell_prefix' field."]
  #[inline]
  pub fn cell_prefix<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.cell_prefix.is_none());
    self.storage.cell_prefix = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }

  #[doc = "Scalar setter for the 'cell_postfix' field."]
  #[inline]
  pub fn cell_postfix<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.cell_postfix.is_none());
    self.storage.cell_postfix = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }

  #[doc = "Scalar setter for the 'cell_separator' field."]
  #[inline]
  pub fn cell_separator<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.cell_separator.is_none());
    self.storage.cell_separator = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }

  #[doc = "Scalar setter for the 'row_prefix' field."]
  #[inline]
  pub fn row_prefix<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.row_prefix.is_none());
    self.storage.row_prefix = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }

  #[doc = "Scalar setter for the 'row_postfix' field."]
  #[inline]
  pub fn row_postfix<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.row_postfix.is_none());
    self.storage.row_postfix = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }

  #[doc = "Scalar setter for the 'row_separator' field."]
  #[inline]
  pub fn row_separator<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.row_separator.is_none());
    self.storage.row_separator = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }
}

impl<Definition> StylesFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>, Formed = Styles<>>,
  Definition::Types: former::FormerDefinitionTypes<Storage = StylesFormerStorage<>, Formed = Styles<>>,
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>>,
  Definition::Types: former::FormerDefinitionTypes<Storage = StylesFormerStorage<>>,
{
  #[doc = r" Executes the transformation from the former's storage state to the preformed object as specified by the definition."]
  pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    former::StoragePreform::preform(self.storage)
  }
}

#[automatically_derived]
impl<Definition> StylesFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>, Formed = Styles<>>,
  Definition::Types: former::FormerDefinitionTypes<Storage = StylesFormerStorage<>, Formed = Styles<>>,
{
  #[doc = r""]
  #[doc = r" Finish setting options and call perform on formed entity."]
  #[doc = r""]
  #[doc = r" If `perform` defined then associated method is called and its result returned instead of entity."]
  #[doc = r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
  #[doc = r""]
  #[inline(always)]
  pub fn perform(self) -> Definition::Formed
  {
    let result = self.form();
    return result;
  }
}

impl<Definition> former::FormerBegin<Definition> for StylesFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = StylesFormerStorage<>>,
{
  #[inline(always)]
  fn former_begin(
    storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: Definition::End,
  ) -> Self
  {
    debug_assert!(storage.is_none());
    Self::begin(None, context, on_end)
  }
}

#[doc = r" Provides a specialized former for structure using predefined settings for superformer and end conditions."]
#[doc = r""]
#[doc = r" This type alias configures former of the structure with a specific definition to streamline its usage in broader contexts,"]
#[doc = r" especially where structure needs to be integrated into larger structures with a clear termination condition."]
pub type StylesAsSubformer<__Superformer, __End> = StylesFormer<StylesFormerDefinition<__Superformer, __Superformer, __End>>;

#[doc = "\nRepresents an end condition for former of [`$Styles`], tying the lifecycle of forming processes to a broader context.\n\nThis trait is intended for use with subformer alias, ensuring that end conditions are met according to the\nspecific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.\n    "]
pub trait StylesAsSubformerEnd<SuperFormer>
where
  Self: former::FormingEnd<StylesFormerDefinitionTypes<SuperFormer, SuperFormer>>,
{}
impl<SuperFormer, __T> StylesAsSubformerEnd<SuperFormer> for __T
where
  Self: former::FormingEnd<StylesFormerDefinitionTypes<SuperFormer, SuperFormer>>,
{}

  // xxx

  impl Default for Styles
  {
    fn default() -> Self
    {
      let cell_prefix = "".to_string();
      let cell_postfix = "".to_string();
      let cell_separator = " ".to_string();
      let row_prefix = "".to_string();
      let row_postfix = "".to_string();
      let row_separator = "\n".to_string();
      Styles
      {
        cell_prefix,
        cell_postfix,
        cell_separator,
        row_prefix,
        row_postfix,
        row_separator,
      }
    }
  }

  /// Struct for formatting tables.
  pub struct Context< 'data >
  {
    buf : &'data mut dyn fmt::Write,
    styles : Styles,
  }

  impl< 'data > Context< 'data >
  {
    /// Just constructr.
    pub fn new( buf : &'data mut dyn fmt::Write, styles : Styles ) -> Self
    {
      Self { buf, styles }
    }
  }

  impl fmt::Debug for Context< '_ >
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f
      .debug_struct( "Context" )
      .field( "buf", &"dyn fmt::Write" )
      .field( "styles", &self.styles )
      .finish()
    }
  }

  /// A trait for converting tables to a string representation.
  pub trait TableToString< 'data >
  {
    /// Converts the table to a string representation.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted table.
    fn table_to_string( &'data self ) -> String;
  }

  impl< 'data, T > TableToString< 'data > for T
  where
    T : TableFormatter< 'data >
  {
    fn table_to_string( &'data self ) -> String
    {
      let mut output = String::new();
      let mut context = Context
      {
        buf : &mut output,
        styles : Styles::default(),
      };
      T::fmt( self, &mut context ).expect( "Table formatting failed" );
      output
    }
  }

  /// A trait for formatting tables.
  ///
  /// This trait defines a method for formatting tables, allowing implementations
  /// to specify how a table should be formatted and displayed.
  ///

  pub trait TableFormatter< 'b >
  {
    /// Formats the table and writes the result to the given formatter.
    fn fmt< 'data >( &'b self, f : &mut Context< 'data > ) -> fmt::Result;
  }

  /// A trait for formatting tables.
  impl< 'data, T, RowKey, Row, CellKey, CellFormat > TableFormatter< 'data >
  for AsTable< 'data, T, RowKey, Row, CellKey, CellFormat >
  where
    Self : TableRows< RowKey, Row, CellKey, CellFormat >,
    Self : TableHeader< CellKey >,
    Self : TableSize,
    Row : Clone + Cells< CellKey, CellFormat >,
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
    CellFormat : Copy + 'static,
  {
    fn fmt( &'data self, f : &mut Context< '_ > ) -> fmt::Result
    {
      use md_math::MdOffset;

      FormatExtract::extract
      (
        self,
        All,
        | x |
        {

          let cell_prefix = &f.styles.cell_prefix;
          let cell_postfix = &f.styles.cell_postfix;
          let cell_separator = &f.styles.cell_separator;
          let row_prefix = &f.styles.row_prefix;
          let row_postfix = &f.styles.row_postfix;
          let row_separator = &f.styles.row_separator;

          for ( irow, row ) in x.row_descriptors.iter().enumerate()
          {
            let height = row.0;

            for islice in 0..height
            {

              if irow > 0
              {
                write!( f.buf, "{}", row_separator )?;
              }

              write!( f.buf, "{}", row_prefix )?;

              for k in &x.col_order
              {
                let col = &x.col_descriptors[ &k ];
                let cell_width = x.data[ irow ][ &k ].1[0];
                let width = col.0;
                let icol = col.1;
                let md_index = [ islice, icol, irow as usize ];
                let slice = x.slices[ x.slices_dim.md_offset( md_index ) ];

                // println!( "md_index : {md_index:?} | md_offset : {} | slice : {slice}", x.slices_dim.md_offset( md_index ) );

                if icol > 0
                {
                  write!( f.buf, "{}", cell_separator )?;
                }

                write!( f.buf, "{}", cell_prefix )?;

                let lspaces = ( width - cell_width ) / 2;
                let rspaces = ( width - cell_width + 1 ) / 2 + cell_width - slice.len();
                // println!( "icol : {icol} | irow : {irow} | width : {width} | cell_width : {cell_width} | lspaces : {lspaces} | rspaces : {rspaces}" );

                if lspaces > 0
                {
                  write!( f.buf, "{:<width$}", " ", width = lspaces )?;
                }
                write!( f.buf, "{}", slice )?;
                if rspaces > 0
                {
                  write!( f.buf, "{:>width$}", " ", width = rspaces )?;
                }

                write!( f.buf, "{}", cell_postfix )?;
              }

              write!( f.buf, "{}", row_postfix )?;
            }

          }

          Ok(())
        }
      )
    }
  }

  /// A struct for extracting and organizing table data for formatting.
  ///
  /// `FormatExtract` holds metadata and content necessary for formatting tables,
  /// including dimensions, column order, and data slices. It facilitates the
  /// transformation of raw table data into a structured format suitable for
  /// rendering as a table.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct FormatExtract< 'data, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash, // xxx
  {

    /// Multidimensional size in number of columns per table and number of rows per table.
    pub mcells : [ usize ; 2 ],

    /// Order of columns must be as stable as possible.
    pub col_order : Vec< CellKey >,

    /// Descriptors for each column, including optional title, width, and index.
    //                             key        width, index
    pub col_descriptors : HashMap< CellKey, ( usize, usize ) >,

    /// Descriptors for each row, including height.
    //                           height
    pub row_descriptors : Vec< ( usize, ) >,

    /// Extracted data for each cell, including string content and size.
    //                        key,      string,              size,
    pub data : Vec< HashMap< CellKey, ( Cow< 'data, str >, [ usize ; 2 ] ) > >,

    /// Dimensions of slices for retrieving data from multi-matrix.
    pub slices_dim : [ usize ; 3 ],

    /// Extracted slices or strings for further processing.
    pub slices : Vec< & 'data str >,

    /// Indicates if the table has a header.
    pub has_header : bool,

  }

  //

  pub trait FilterCol< CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn filter_col( &self, key : CellKey ) -> bool;
  }

  pub struct All;
  impl< CellKey > FilterCol< CellKey > for All
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn filter_col( &self, key : CellKey ) -> bool
    {
      true
    }
  }

  pub struct None;
  impl< CellKey > FilterCol< CellKey > for None
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {
    fn filter_col( &self, key : CellKey ) -> bool
    {
      false
    }
  }

  //

  impl< 'data, CellKey > FormatExtract< 'data, CellKey >
  where
    CellKey : fmt::Debug + Clone + std::cmp::Eq + std::hash::Hash,
  {

    pub fn extract< 't, Table, RowKey, Row, CellFormat > // xxx : RowKey?
    (
      table : &'t Table,
      filter_col : impl FilterCol< CellKey >,
      callback : impl for< 'a2 > FnOnce( &'a2 FormatExtract< 'a2, CellKey > ) -> fmt::Result,
    )
    -> fmt::Result
    where
      't : 'data,
      Table : TableRows< RowKey, Row, CellKey, CellFormat >,
      Table : TableHeader< CellKey >,
      Table : TableSize,
      Row : Clone + Cells< CellKey, CellFormat > + 'data,
      CellFormat : Copy + 'static,
    {
      use md_math::MdOffset;

      let mcells = table.mcells();
      //                                 key        width, index
      let mut col_descriptors : HashMap< CellKey, ( usize, usize ) > = HashMap::new();
      //                               height
      let mut row_descriptors : Vec< ( usize, ) > = Vec::with_capacity( mcells[ 1 ] );

      let mut col_order : Vec< CellKey > = Vec::new();
      let mut has_header = false;

      let mut data : Vec< HashMap< CellKey, ( Cow< 'data, str >, [ usize ; 2 ] ) > > = Vec::new();
      let rows = table.rows();
      let mut irow : isize = -1;

      let mut row_add = | row : &'_ mut dyn _IteratorTrait< Item = ( CellKey, Cow< 'data, str > ) > |
      {

        irow += 1;
        row_descriptors.push( ( 1, ) );

        let fields : HashMap< CellKey, ( Cow< 'data, str >, [ usize ; 2 ] ) > = row
        .map
        (
          | ( key, val ) |
          {

            let sz = string::size( &val );
            let l = col_descriptors.len();
            row_descriptors[ irow as usize ] = ( row_descriptors[ irow as usize ].0.max( sz[ 1 ] ), );

            col_descriptors
            .entry( key.clone() )
            .and_modify( | col |
            {
              col.0 = col.0.max( sz[ 0 ] );
            })
            .or_insert_with( ||
            {
              col_order.push( key.clone() );
              ( sz[ 0 ], l )
              // let title = if is_title { Some( val.as_ref() ) } else { None };
              // ( title, sz[ 0 ], l )
            });

            return ( key, ( val, sz ) );
          }
        )
        .collect();
        data.push( fields );


      };

      // process header first

      if let Some( header ) = table.header()
      {
        rows.len().checked_add( 1 ).expect( "Table has too many rows" );
        // assert!( header.len() <= usize::MAX, "Header of a table has too many cells" );
        has_header = true;

        let mut row2 =  header.map( | ( key, title ) |
        {
          // let title_str : Cow< '_, str > = Cow::Owned( format!( "{}", title ) );
          ( key, title )
        });

        row_add( &mut row2 );
      }

      // Collect rows
      //                           key,       string,           size,
      for row in rows
      {
        // assert!( row.cells().len() <= usize::MAX, "Row of a table has too many cells" );

        let mut row2 = row
        .cells()
        .map
        (
          | ( key, val ) |
          {

            let val = match val.0
            {
              Some( val ) =>
              {
                val
              }
              None =>
              {
                Cow::Borrowed( "" )
              }
            };

            return ( key, val );
          }
        );

        row_add( &mut row2 );

      }

      // cook slices multi-matrix

      let mut slices_dim = [ 1, mcells[ 0 ], mcells[ 1 ] + ( if has_header { 1 } else { 0 } ) ];
      slices_dim[ 0 ] = row_descriptors
      .iter()
      .fold( 0, | acc : usize, e | acc.max( e.0 ) )
      ;

      let slices_len = slices_dim[ 0 ] * slices_dim[ 1 ] * slices_dim[ 2 ];
      let slices : Vec< &str > = vec![ "" ; slices_len ];

      let mut x = FormatExtract::< '_, CellKey >
      {
        mcells,
        col_order,
        col_descriptors,
        row_descriptors,
        data,
        has_header,
        slices_dim,
        slices,
      };

      // extract slices

      let mut slices : Vec< &str > = vec![];
      std::mem::swap( &mut x.slices, &mut slices );

      let mut irow : isize = -1;

      for row_data in x.data.iter()
      {

        irow += 1;

        for ( icol, k ) in x.col_order.iter().enumerate()
        {
          let cell = &row_data[ &k ];
          string::lines( cell.0.as_ref() )
          .enumerate()
          .for_each( | ( layer, s ) |
          {
            let md_index = [ layer, icol, irow as usize ];
            slices[ x.slices_dim.md_offset( md_index ) ] = s;
          })
          ;
        }

      }

      std::mem::swap( &mut x.slices, &mut slices );

      return callback( &x );
    }

  }

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use private::
  {
    Styles,
    Context,
    TableFormatter,
    TableToString,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
