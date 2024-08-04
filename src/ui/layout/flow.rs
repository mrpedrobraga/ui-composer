pub enum FlowDirection {
    /// Geometrically, absolutely defined flow.
    Cartesian(CartesianFlowDirection),
    /// Flow aligned with the parent flow.
    Aligned(AlignedFlowDirection),
    /// Flow aligned with the writing flow (usually based on locale).
    Writing(WritingFlowDirection),
}

/// Specifies the flow direction in an geometric/absolute fashion.
/// Use it for things like graphics or symbols.
pub enum CartesianFlowDirection {
    ///→
    LeftToRight,
    ///←
    RightToLeft,
    ///↓
    TopToBottom,
    ///↑
    BottomToTop,
}

pub enum AlignedFlowDirection {
    /// Aligned with parent flow. Default: →.
    MainAxis,
    /// Opposite of parent flow. Default: ←.
    MainAxisReverse,
    /// Aligned with parent cross flow. Default: ↓.
    CrossAxis,
    /// Opposite of parent cross flow. Default: ↑.
    CrossAxisReverse,
}

pub enum WritingFlowDirection {
    /// Aligned with writing flow. en-us: →.
    WritingAxis,
    /// Opposite of writing flow. en-us: ←.
    WritingAxisReverse,
    /// Aligned with writing cross flow. en-us: ↓.
    WritingCrossAxis,
    /// Opposite of writing cross flow. en-us: ↑.
    WritingCrossAxisReverse,
}
