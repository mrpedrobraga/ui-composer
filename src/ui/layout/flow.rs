pub enum FlowDirection {
    /// Geometrically, absolutely defined flow.
    Cartesian(CartesianFlowDirection),
    /// Flow aligned relative to the parent flow.
    Aligned(AlignedFlowDirection),
    /// Flow aligned with the writing flow (usually based on locale).
    Writing(WritingFlowDirection),
}

/// Specifies the flow direction in a geometric/absolute fashion.
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
    MainAxisForward,
    /// Opposite of parent flow. Default: ←.
    MainAxisBackwards,
    /// Aligned with parent cross flow. Default: ↓.
    CrossAxisForward,
    /// Opposite of parent cross flow. Default: ↑.
    CrossAxisBackwards,
}

pub enum WritingFlowDirection {
    /// Aligned with writing flow. en-us: →.
    WritingAxisForward,
    /// Opposite of writing flow. en-us: ←.
    WritingAxisBackwards,
    /// Aligned with writing cross flow. en-us: ↓.
    WritingCrossAxisForward,
    /// Opposite of writing cross flow. en-us: ↑.
    WritingCrossAxisBackwards,
}
