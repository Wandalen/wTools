crate::mod_interface!
{

  /// Main aggregating object.
  layer context;
  /// Context changer.
  layer context_changer;

  /// Draw changer.
  layer drawing;
  /// Draw changer.
  layer drawing_changer;
  /// ChangeInterface for drawing constructor.
  layer drawing_change_new;

  /// Brush stroke.
  layer stroke_brush;
  /// ChangerInterface of brush stroke.
  layer stroke_brush_changer;
  /// ChangeInterface of brush stroke constructor.
  layer stroke_brush_change_new;
  /// ChangeInterface of brush stroke color.
  layer stroke_brush_change_color;
  /// Target to draw.
  layer target;

  // exposed use Drawing;

}