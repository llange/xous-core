use crate::api::{Point, Rectangle, Gid};
use blitstr_ref as blitstr;
use blitstr::{GlyphStyle, Cursor};

/// coordinates are local to the canvas, not absolute to the screen
#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Unarchive)]
pub enum TextBounds {
    // fixed width and height in a rectangle
    BoundingBox(Rectangle),
    // fixed width, grows up from bottom right
    GrowableFromBr(Point, u16),
    // fixed width, grows down from top left
    GrowableFromTl(Point, u16),
    // fixed width, grows up from bottom left
    GrowableFromBl(Point, u16),
}

#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Unarchive, PartialEq)]
// operations that may be requested of a TextView when sent to GAM
pub enum TextOp {
    Nop,
    Render,
    ComputeBounds, // maybe we don't need this
}
impl Into<usize> for TextOp {
    fn into(self) -> usize {
        match self {
            TextOp::Nop => 0,
            TextOp::Render => 1,
            TextOp::ComputeBounds => 2,
        }
    }
}
impl From<usize> for TextOp {
    fn from(code: usize) -> Self {
        match code {
            1 => TextOp::Render,
            2 => TextOp::ComputeBounds,
            _ => TextOp::Nop,
        }
    }
}

// roughly 168 bytes to represent the rest of the struct, and we want to fill out the 4096 byte page with text
const TEXTVIEW_LEN: usize = 3072;

#[derive(Copy, Clone, rkyv::Archive, rkyv::Unarchive)]
pub struct TextView {
    // this is the operation as specified for the GAM. Note this is different from the "op" when sent to graphics-server
    // only the GAM should be sending TextViews to the graphics-server, and a different coding scheme is used for that link.
    operation: TextOp,

    pub untrusted: bool,  // render content with random stipples to indicate the strings within are untrusted
    pub token: Option<[u32; 4]>, // optional 128-bit token which is presented to prove a field's trustability
    pub invert: bool, // only trusted, token-validated TextViews will have the invert bit respected

    // offsets for text drawing -- exactly one of the following options should be specified
    // note that the TextBounds coordinate system is local to the canvas, not the screen
    pub bounds_hint: TextBounds,
    pub bounds_computed: Option<Rectangle>, // is Some(Rectangle) if bounds have been computed and text has not been modified
    pub overflow: Option<bool>,  // indicates if the text has overflowed the canvas, set by the drawing routine
    pub dry_run: bool, // set to true if no drawing is desired and we just want to compute the bounds

    pub style: GlyphStyle,
    pub text: xous::String::<3072>,
    pub cursor: Cursor,
    pub insertion: Option<i32>, // this is the insertion point offset, if it's to be drawn, on the string
    pub ellipsis: bool,

    pub draw_border: bool,
    pub clear_area: bool, // you almost always want this to be true
    pub border_width: u16,
    pub rounded_border: Option<u16>, // radius of the rounded border, if applicable
    pub margin: Point,

    // this field specifies the beginning and end of a "selected" region of text
    pub selected: Option<[u32; 2]>,

    canvas: Gid, // GID of the canvas to draw on
    pub clip_rect: Option<Rectangle>,  // this is set by the GAM to the canvas' clip_rect; needed by gfx for drawing. Note this is in screen coordinates.
}
impl TextView {
    pub fn new(canvas: Gid, bounds_hint: TextBounds) -> Self {
        TextView {
            operation: TextOp::Nop,
            untrusted: true,
            token: None,
            invert: false,
            clip_rect: None,
            bounds_hint,
            bounds_computed: None,
            style: GlyphStyle::Regular,
            text: xous::String::<3072>::new(),
            cursor: Cursor::new(0,0,0),
            insertion: None,
            ellipsis: false,
            draw_border: true,
            border_width: 1,
            rounded_border: None,
            margin: Point { x: 4, y: 4 },
            selected: None,
            canvas,
            clear_area: true,
            overflow: None,
            dry_run: false,
        }
    }
    pub fn set_op(&mut self, op: TextOp) { self.operation = op; }
    pub fn get_op(&self) -> TextOp { self.operation }
    pub fn get_canvas_gid(&self) -> Gid { self.canvas }

    pub fn to_str(&self) -> &str {
        self.text.as_str().unwrap()
    }

    pub fn clear_str(&mut self) { self.text.clear() }

    pub fn populate_from(&mut self, t: &TextView) {
        self.operation = t.operation;
        self.untrusted = t.untrusted;
        self.token = t.token;
        self.invert = t.invert;
        self.bounds_hint = t.bounds_hint;
        self.bounds_computed = t.bounds_computed;
        self.style = t.style;
        self.text = t.text;
        self.cursor = t.cursor;
        self.draw_border = t.draw_border;
        self.clear_area = t.clear_area;
        self.border_width = t.border_width;
        self.rounded_border = t.rounded_border;
        self.margin = t.margin;
        self.selected = t.selected;
        self.canvas = t.canvas;
        self.overflow = t.overflow;
        self.clip_rect = t.clip_rect;
        self.dry_run = t.dry_run;
        self.insertion = t.insertion;
    }
}

// Allow a `&TextView` to be used anywhere that expects a `&str`
impl AsRef<str> for TextView {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl core::fmt::Debug for TextView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // this should definitely be extended to print more relevant data, but for now just render the string itself
        write!(f, "{:?}, {:?}, {:?}, {:?}, {}",
            self.get_op(), self.bounds_hint, self.cursor, self.get_canvas_gid(), self.to_str())
    }
}

// allow printing of the text
impl core::fmt::Display for TextView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

// allow `write!()` macro on a` &TextView`
impl core::fmt::Write for TextView {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        write!(self.text, "{}", s)
    }
}
