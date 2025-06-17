pub mod located;
pub mod location;
pub mod span;

use codespan::Span as ByteSpan;
pub type Position = u32;

#[cfg(test)]
mod tests {
    use crate::Span;

    #[test]
    fn test_intersects() {
        assert!(Span::from(5..10).intersects(&Span::from(5..10)));

        assert!(Span::from(5..10).intersects(&Span::from(5..5)));
        assert!(Span::from(5..5).intersects(&Span::from(5..10)));

        assert!(Span::from(10..10).intersects(&Span::from(5..10)));
        assert!(Span::from(5..10).intersects(&Span::from(10..10)));

        assert!(Span::from(5..10).intersects(&Span::from(6..9)));
        assert!(Span::from(6..9).intersects(&Span::from(5..10)));

        assert!(Span::from(5..10).intersects(&Span::from(4..11)));
        assert!(Span::from(4..11).intersects(&Span::from(5..10)));

        assert!(Span::from(5..10).intersects(&Span::from(4..6)));
        assert!(Span::from(4..6).intersects(&Span::from(5..10)));

        assert!(Span::from(5..10).intersects(&Span::from(9..11)));
        assert!(Span::from(9..11).intersects(&Span::from(5..10)));

        assert!(!Span::from(5..10).intersects(&Span::from(3..4)));
        assert!(!Span::from(3..4).intersects(&Span::from(5..10)));

        assert!(!Span::from(5..10).intersects(&Span::from(11..12)));
        assert!(!Span::from(11..12).intersects(&Span::from(5..10)));
    }
}
